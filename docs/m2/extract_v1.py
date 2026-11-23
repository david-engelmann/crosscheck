#!/usr/bin/env python3
"""Pattern extractor v1 for the M2 Track A re-measurement.

The v0 pattern set (docs/m1/extract.py) with exactly the five
changes approved in docs/M1-RESULTS.md and ratified in D10:

1. The size pattern owns decimal numbers: "16.9 GB" matches as
   one claim instead of the fragment "9 GB".
2. The count pattern refuses a number immediately preceded by
   a letter, digit, dot, or underscore, which suppresses
   name-internal digits ("v2 families") and label fragments
   ("P1 findings", the "P1.1" comment).
3. The bare-number pattern merges into the PR class as a
   candidate: a bare #NNN on a line that already carries an
   explicit PR reference to the same number is suppressed.
   Each merged claim records whether it arrived via the
   explicit or the bare shape, for scoring diagnostics. The
   merged class inherits nothing until it re-measures.
4. The quote pattern is dropped, pending a structural signal.
5. Tally guard: a number-and-tally-word match is suppressed
   when the number is fused to a preceding letter, digit, dot,
   or underscore ("G5 passed"), or when the token immediately
   before the number is a commit-like hex token or a single
   letter.

One completion, disclosed: the tally class as designed in
docs/M1-EXTRACTION-MEASUREMENT.md includes the of-form
("N of M"); v0 defined OF_RE but never wired it into the
extraction. V1 wires it. No other pattern changes.

Read-only: it never writes. Output is one claim per line to
stdout, pipe-separated, with a per-class summary on stderr.
"""
import json
import re
import sys
from pathlib import Path

FILE_RE = re.compile(r"[\w./~-]+\.(?:md|py|rs|toml|json|jsonl|txt|log|sh|ya?ml)\b")
SIZE_RE = re.compile(r"(\d[\d,]*(?:\.\d+)?)\s*(bytes?|KiB|MiB|GiB|KB|MB|GB)\b")
SHA256_RE = re.compile(r"\b[0-9a-f]{64}\b")
SHA40_RE = re.compile(r"\b[0-9a-f]{40}\b")
SHORTSHA_RE = re.compile(r"\b[0-9a-f]{7,12}\b")
COMMIT_CTX = re.compile(r"commit|head|base|sha|merge|reviewed|branch", re.I)
NUM_GUARD = r"(?<![A-Za-z0-9_.])"
PASSED_RE = re.compile(NUM_GUARD + r"(\d[\d,]*)\s+passed\b")
FAILED_RE = re.compile(NUM_GUARD + r"(\d[\d,]*)\s+failed\b")
SKIPPED_RE = re.compile(NUM_GUARD + r"(\d[\d,]*)\s+skipped\b")
TESTS_RE = re.compile(NUM_GUARD + r"(\d[\d,]*)\s+(tests?|checks?|subtests)\b")
EXIT_RE = re.compile(r"exit(?: code)?\s+(\d+)\b")
FRACTION_RE = re.compile(r"(\d+)\s*/\s*(\d+)\s+(gates?|checks?|tests?|calls?)\b")
OF_RE = re.compile(r"(\d[\d,]*)\s+of\s+(\d[\d,]*)\b")
PR_RE = re.compile(r"\bPR\s*#(\d+)")
PULL_RE = re.compile(r"pull/(\d+)")
REPO_PR_RE = re.compile(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+#(\d+)")
BARE_RE = re.compile(r"(?<![\w#])#(\d{2,})\b")
COUNT_RE = re.compile(
    NUM_GUARD +
    r"(\d[\d,]*)\s+(files?|findings?|rounds?|repos?|crates?|modules?|headings?|cases?|"
    r"commits?|lines?|errors?|warnings?|pairs?|instances?|defects?|suites?|families|"
    r"arms|prompts|documents?|entries|items|comments?|nits|violations?)\b")
HEX_TOKEN = re.compile(r"^[0-9a-f]{7,40}$")


def first_file(line: str):
    m = FILE_RE.search(line)
    return m.group(0) if m else None


def tally_blocked(line: str, start: int) -> bool:
    """The v1 tally guard (change 5), second clause: the token
    immediately before the tally number is a commit-like hex
    token or a single letter. The fused-label clause is the
    NUM_GUARD lookbehind in the tally regexes themselves."""
    before = line[:start].rstrip()
    if not before:
        return False
    token = before.split()[-1].strip("`*(|")
    if len(token) == 1 and token.isalpha():
        return True
    return bool(HEX_TOKEN.match(token)) and any(c.isalpha() for c in token)


def extract(report_name: str, text: str):
    claims = []
    seen = set()

    def add(line_no, cls, value, target, line, via="-"):
        key = (line_no, cls, value)
        if key in seen:
            return
        seen.add(key)
        claims.append({
            "report": report_name, "line": line_no, "class": cls,
            "value": value, "target": target, "via": via,
            "text": line.strip()[:200],
        })

    for i, line in enumerate(text.splitlines(), start=1):
        fname = first_file(line)
        for m in SIZE_RE.finditer(line):
            add(i, "size", m.group(0), fname, line)
        for m in SHA256_RE.finditer(line):
            add(i, "hash256", m.group(0), fname, line)
        for m in SHA40_RE.finditer(line):
            add(i, "gitsha", m.group(0), fname, line)
        if COMMIT_CTX.search(line):
            for m in SHORTSHA_RE.finditer(line):
                if not re.fullmatch(r"\d+", m.group(0)):
                    add(i, "commit", m.group(0), fname, line)
        for regex, label in ((PASSED_RE, "passed"), (FAILED_RE, "failed"),
                             (SKIPPED_RE, "skipped"), (TESTS_RE, "tests"),
                             (EXIT_RE, "exit")):
            for m in regex.finditer(line):
                if label != "exit" and tally_blocked(line, m.start()):
                    continue
                add(i, "tally", f"{label} {m.group(0)}", fname, line)
        for m in FRACTION_RE.finditer(line):
            add(i, "tally", m.group(0), fname, line)
        for m in OF_RE.finditer(line):
            add(i, "tally", m.group(0), fname, line)
        explicit = set()
        for regex in (PR_RE, PULL_RE, REPO_PR_RE):
            for m in regex.finditer(line):
                explicit.add(m.group(1))
                add(i, "pr", f"PR {m.group(1)}", fname, line, via="explicit")
        for m in BARE_RE.finditer(line):
            if m.group(1) in explicit:
                continue
            add(i, "pr", f"PR {m.group(1)}", fname, line, via="bare")
        for m in COUNT_RE.finditer(line):
            add(i, "count", m.group(0), fname, line)
    return claims


SOURCES = [
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pr452-openai-trio-2026-10-13.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pr454-anthropic-families-wire-2026-10-14.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pr456-openai-decisions-2026-10-14.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pr458-robustness-tax-identity-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pr459-decisions-emdash-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pr461-jev114-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-celeris-adapter-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-cerno-adapter-2026-10-22.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/a6-run-plan-redteam-report-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/display-scaffold-redteam-report-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/eb-conv-redteam-report-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/perfection-data-audit-cycle1-2026-10-23.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/ui-convergence-reaudit-2026-10-23.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/v2-draft-audit-completion-gate-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/eb-reporting-hygiene-gate-report.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/docs-perfection-gate-report-2026-09-30.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/canary-pod-report-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/von-driver-wiring-report-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pod-repairs-2026-10-23.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-fix-forward-main-red-20260930.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-jev-fallback-fix-2026-10-23.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-score-fallback-fix-2026-10-22.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-score-fallback-followups-2026-10-25.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-lg3-wiring-2026-10-23.md",
]


def main() -> int:
    root = Path.home() / "workspace"
    all_claims = []
    for i, rel in enumerate(SOURCES, start=1):
        p = root / rel
        claims = extract(f"d{i:02d}", p.read_text(errors="replace"))
        all_claims.extend(claims)
    for c in all_claims:
        print(f"{c['report']}|{c['line']}|{c['class']}|{c['value']}|{c['target'] or '-'}|{c['via']}|{c['text']}")
    from collections import Counter
    counts = Counter(c["class"] for c in all_claims)
    print("PER-CLASS " + json.dumps(dict(counts)), file=sys.stderr)
    print(f"total claims: {len(all_claims)}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
