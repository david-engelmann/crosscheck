#!/usr/bin/env python3
"""Pattern extractor v0 for the M1 measurement.

Implements the six candidate pattern classes from
docs/M1-EXTRACTION-MEASUREMENT.md over the frozen corpus and prints
the claim list as JSON to stdout. Read-only: it never writes.
"""
import json
import re
import sys
from pathlib import Path

CORPUS = Path.home() / "workspace/retrial-m1/corpus"

FILE_RE = re.compile(r"[\w./~-]+\.(?:md|py|rs|toml|json|jsonl|txt|log|sh|ya?ml)\b")
SIZE_RE = re.compile(r"(\d[\d,]*)\s*(bytes?|KiB|MiB|GiB|KB|MB|GB)\b")
SHA256_RE = re.compile(r"\b[0-9a-f]{64}\b")
SHA40_RE = re.compile(r"\b[0-9a-f]{40}\b")
SHORTSHA_RE = re.compile(r"\b[0-9a-f]{7,12}\b")
COMMIT_CTX = re.compile(r"commit|head|base|sha|merge|reviewed|branch", re.I)
PASSED_RE = re.compile(r"(\d[\d,]*)\s+passed\b")
FAILED_RE = re.compile(r"(\d[\d,]*)\s+failed\b")
SKIPPED_RE = re.compile(r"(\d[\d,]*)\s+skipped\b")
TESTS_RE = re.compile(r"(\d[\d,]*)\s+(tests?|checks?|subtests)\b")
EXIT_RE = re.compile(r"exit(?: code)?\s+(\d+)\b")
FRACTION_RE = re.compile(r"(\d+)\s*/\s*(\d+)\s+(gates?|checks?|tests?|calls?)\b")
OF_RE = re.compile(r"(\d+)\s+of\s+(\d+)\b")
PR_RE = re.compile(r"\bPR\s*#(\d+)")
PULL_RE = re.compile(r"pull/(\d+)")
BARE_RE = re.compile(r"(?<![\w#])#(\d{2,})\b")
COUNT_RE = re.compile(
    r"(\d[\d,]*)\s+(files?|findings?|rounds?|repos?|crates?|modules?|headings?|cases?|"
    r"commits?|lines?|errors?|warnings?|pairs?|instances?|defects?|suites?|families|"
    r"arms|prompts|documents?|entries|items|comments?|nits|violations?)\b")
QUOTE_RE = re.compile(r'"([^"]{15,})"')


def first_file(line: str):
    m = FILE_RE.search(line)
    return m.group(0) if m else None


def extract(report_name: str, text: str):
    claims = []
    seen = set()

    def add(line_no, cls, value, target, line):
        key = (line_no, cls, value)
        if key in seen:
            return
        seen.add(key)
        claims.append({
            "report": report_name, "line": line_no, "class": cls,
            "value": value, "target": target,
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
                add(i, "tally", f"{label} {m.group(0)}", fname, line)
        for m in FRACTION_RE.finditer(line):
            add(i, "tally", m.group(0), fname, line)
        for m in PR_RE.finditer(line):
            add(i, "pr", f"PR {m.group(1)}", fname, line)
        for m in PULL_RE.finditer(line):
            add(i, "pr", f"PR {m.group(1)}", fname, line)
        for m in BARE_RE.finditer(line):
            add(i, "pr-bare", f"#{m.group(1)}", fname, line)
        for m in COUNT_RE.finditer(line):
            add(i, "count", m.group(0), fname, line)
        if fname:
            for m in QUOTE_RE.finditer(line):
                add(i, "quote", m.group(1)[:60], fname, line)
    return claims


SOURCES = [
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pr232.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pr240.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pr247.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/r13-r14-gate-report.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/fixforward-gate-report-20260930.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/pr442-gate-report.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/gate-report-pr449-dashboard-publish-2026-10-13.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/metrics-bootstrap-gate-report.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/FINAL-AUDIT-2026-10-03.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/PILOT-AUDIT-2026-10-03.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/POST-PILOT-AUDIT-2026-10-03.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/dataset-independent-audit-2026-09-24.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/goal-audit-2026-09-29.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/v2-draft-audit-report-20261001.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/distractor_flooding-audit-2026-09-24.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/eb-fairness-redteam-report-20261001.md",
    "github-app-hub/hygiene/cleanup-log-2026-11-20.md",
    "github-app-hub/hygiene/prune-log-2026-11-20.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/anthropic-workspace-lane/BUILDER_REPORT.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/dashkey-lane/BUILDER_REPORT.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/canary-rerun-report-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/vast-sweep-report-2026-10-15.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/board-review/board-report.md",
    "goals/peira-decision-model-robustness-benchmark/hidden_files/EB27-EB47-three-review-record.md",
]


def main() -> int:
    root = Path.home() / "workspace"
    all_claims = []
    for i, rel in enumerate(SOURCES, start=1):
        p = root / rel
        claims = extract(f"c{i:02d}", p.read_text(errors="replace"))
        all_claims.extend(claims)
    for c in all_claims:
        print(f"{c['report']}|{c['line']}|{c['class']}|{c['value']}|{c['target'] or '-'}|{c['text']}")
    from collections import Counter
    counts = Counter(c["class"] for c in all_claims)
    print("PER-CLASS " + json.dumps(dict(counts)), file=sys.stderr)
    print(f"total claims: {len(all_claims)}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
