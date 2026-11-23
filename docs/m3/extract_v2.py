#!/usr/bin/env python3
"""Pattern extractor v2 for the Track A second re-measurement.

The v1 pattern set (docs/m2/extract_v1.py) with exactly the
three changes fixed in v2-manifest.txt before this extractor
existed, each aimed at a false positive mechanism measured
in the v1 round:

1. The of-form gains the fused-token number guard (the v1
   tally decimal fragments, "01 of 1" from "0.01 of 1").
2. The of-form does not fire when the word before its
   number is a series word (review, rank, checkpoint,
   section, part, step, phase). Document series and rank
   positions are not run facts. The hash-mark case of the
   same mechanism is covered by change 3's guard set.
3. The fused-token guard set extends from letters, digits,
   dot, and underscore to also cover hash, hyphen, tilde,
   and equals, for the tally and count patterns (the
   measured C-01, SO-01, HEAD~1, NO_RUST=1, and #480-test
   fusions).

No other pattern changes. Every class re-measures; no class
inherits its v1 rate.

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
NUM_GUARD = r"(?<![A-Za-z0-9_.#\-~=])"
PASSED_RE = re.compile(NUM_GUARD + r"(\d[\d,]*)\s+passed\b")
FAILED_RE = re.compile(NUM_GUARD + r"(\d[\d,]*)\s+failed\b")
SKIPPED_RE = re.compile(NUM_GUARD + r"(\d[\d,]*)\s+skipped\b")
TESTS_RE = re.compile(NUM_GUARD + r"(\d[\d,]*)\s+(tests?|checks?|subtests)\b")
EXIT_RE = re.compile(r"exit(?: code)?\s+(\d+)\b")
FRACTION_RE = re.compile(r"(\d+)\s*/\s*(\d+)\s+(gates?|checks?|tests?|calls?)\b")
OF_RE = re.compile(NUM_GUARD + r"(\d[\d,]*)\s+of\s+(\d[\d,]*)\b")
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
SERIES_WORDS = {"review", "rank", "checkpoint", "section", "part", "step", "phase"}


def first_file(line: str):
    m = FILE_RE.search(line)
    return m.group(0) if m else None


def prev_token(line: str, start: int) -> str:
    before = line[:start].rstrip()
    if not before:
        return ""
    return before.split()[-1].strip("`*(|")


def tally_blocked(line: str, start: int) -> bool:
    """The v1 tally guard, second clause, unchanged."""
    token = prev_token(line, start)
    if len(token) == 1 and token.isalpha():
        return True
    return bool(HEX_TOKEN.match(token)) and any(c.isalpha() for c in token)


def of_blocked(line: str, start: int) -> bool:
    """V2 change 2: the word before the of-form's number is a
    series word, so the phrase is document metadata or a
    position, not a run fact."""
    token = prev_token(line, start).strip("#").lower()
    return token in SERIES_WORDS


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
            if of_blocked(line, m.start()):
                continue
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
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-clef-adapter-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-decider-adapter-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-gliner-decide-adapter-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-inception-decide-adapter-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pplx-decider-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-sol-adapter-2026-10-13.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-strands-decider-adapter-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-von-adapter-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/a6-run-plan-spec-review-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/a6-run-plan-standards-review-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/display-scaffold-spec-review-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/eb-conv-spec-review-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/noise-redteam-review-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/noise-spec-review-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/pr295-repair-redteam-review-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/pr338-spec-review-20261002.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-fleet-pins-landings-2026-10-23.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-kpi-distinct-model-count-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-presentation-lane-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/github-finalization-2026-10-23.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/local-run-sweep-2026-10-14.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/openai-logprobs-fix-gate-20261003.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/readme-freeze-refresh-2026-10-23.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/ui-round40-landing-2026-10-23.md",
]


def main() -> int:
    root = Path.home() / "workspace"
    all_claims = []
    for i, rel in enumerate(SOURCES, start=1):
        p = root / rel
        claims = extract(f"e{i:02d}", p.read_text(errors="replace"))
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
