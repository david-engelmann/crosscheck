#!/usr/bin/env python3
"""The M4 demonstration run (docs/M4-DESIGN.md, acceptance).

Assembles a bundle for one report from the v1 frozen corpus
(d01, the PR #452 gate report), runs the built crosscheck
binary in extract mode over it, and checks the outcome
against the hand adjudication banked in docs/m2/scores-v1.md.

The bundle carries:
- repo/: a local clone of a peira checkout whose history
  holds the commits the report cites (the clone source is
  the first of the CANDIDATES below that resolves a1cfc044,
  the PR #450 head the report's owner gates ran on).
- records/: the flat PR records banked beside this script
  under demo-bundle/records/, fetched from the GitHub API on
  2026-11-23 and trimmed to the D14 flat shape.

Expected, from the banked adjudication of d01 (all 36 of its
extracted claims were true positives):
- Extraction finds pr 10, commit 5, gitsha 1, tally 16, and
  no hash256, count, or size claims: the Rust port reproduces
  the measured Python v1 extraction on this report.
- The 5 commit claims verify against the bundled repository.
- The PR claims verify against the bundled records, which
  show both PRs closed and merged.
- The one gitsha claim is a trial payload fingerprint, not a
  commit: the bundle cannot answer it and it is unverifiable.
- No bundled log records the tallies, so every tally claim
  is unverifiable and provisional, and none moves the exit.
- The exit code is 3: trusted claims stand unverifiable
  (the fingerprint), none is contradicted.

Usage: run_demo.py [path-to-crosscheck-binary]
Default binary: ~/workspace/crosscheck/target/debug/crosscheck
Exit 0 when every expectation holds.
"""
import json
import shutil
import subprocess
import sys
from pathlib import Path

WS = Path.home() / "workspace"
REPO = WS / "crosscheck"
SCRATCH = WS / "crosscheck-demo"
REPORT_SRC = (
    WS
    / "goals/peira-decision-model-robustness-benchmark/hidden_files"
    / "gate-report-pr452-openai-trio-2026-10-13.md"
)
CANDIDATES = [
    WS / "peira-452-verify",
    WS / "peira-454-verify",
    WS / "peira-174-review",
]


def resolves(clone: Path, sha: str) -> bool:
    return (
        subprocess.run(
            ["git", "-C", str(clone), "rev-parse", "--verify", "--quiet", f"{sha}^{{commit}}"],
            capture_output=True,
        ).returncode
        == 0
    )


def main() -> int:
    binary = Path(sys.argv[1]) if len(sys.argv) > 1 else REPO / "target/debug/crosscheck"
    if not binary.exists():
        print(f"binary not found: {binary}")
        return 2
    src = next((c for c in CANDIDATES if resolves(c, "a1cfc044")), None)
    if src is None:
        print("no peira clone on this box resolves a1cfc044")
        return 2
    if SCRATCH.exists():
        shutil.rmtree(SCRATCH)
    bundle = SCRATCH / "bundle"
    (bundle / "records").mkdir(parents=True)
    for rec in (REPO / "docs/m4/demo-bundle/records").glob("*.json"):
        shutil.copy(rec, bundle / "records" / rec.name)
    subprocess.run(
        ["git", "clone", "--local", "--quiet", str(src), str(bundle / "repo")],
        check=True,
    )
    report = SCRATCH / "report.md"
    shutil.copy(REPORT_SRC, report)

    out = subprocess.run(
        [str(binary), "extract", str(report), "--bundle", str(bundle), "--json"],
        capture_output=True,
        text=True,
    )
    verdict = json.loads(out.stdout)
    claims = verdict["claims"]
    failures = []

    def check(label, ok, detail=""):
        print(("PASS " if ok else "FAIL ") + label + (f" ({detail})" if detail and not ok else ""))
        if not ok:
            failures.append(label)

    by_class = {}
    for c in claims:
        by_class.setdefault(c["class"], []).append(c)
    counts = {k: len(v) for k, v in by_class.items()}
    check(
        "extraction reproduces the measured v1 counts on d01",
        counts == {"pr": 10, "commit": 5, "gitsha": 1, "tally": 16},
        f"got {counts}",
    )
    check(
        "all commit claims verify against the bundled repository",
        all(c["status"] == "OK" for c in by_class.get("commit", [])),
    )
    check(
        "all PR claims verify against the bundled records",
        all(c["status"] == "OK" for c in by_class.get("pr", [])),
    )
    check(
        "the payload fingerprint is unverifiable, never contradicted",
        [c["status"] for c in by_class.get("gitsha", [])] == ["UNVERIFIABLE"],
    )
    check(
        "every tally is provisional and unverifiable without a bundled log",
        all(
            c["trust"] == "provisional" and c["status"] == "UNVERIFIABLE"
            for c in by_class.get("tally", [])
        ),
    )
    check(
        "every verified verdict names its evidence",
        all(
            ("records/pr-" in c["detail"] or "bundled repository" in c["detail"])
            for c in claims
            if c["status"] == "OK"
        ),
    )
    check("exit code is 3", verdict["exit_code"] == 3 and out.returncode == 3,
          f"got json {verdict['exit_code']}, process {out.returncode}")
    print(f"demo {'FAILED' if failures else 'PASSED'}: {len(claims)} claims, exit {verdict['exit_code']}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
