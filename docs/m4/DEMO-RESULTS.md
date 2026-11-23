# M4 demonstration run, results

Run 2026-11-23 with the crosscheck binary built locally
from the exact tree at 70c230e (the M4 code commit), over
the v1 corpus report d01 (the PR #452 gate report) with a
bundle assembled by docs/m4/run_demo.py: a local clone of a
peira checkout as repo/, the fetched flat records for PR
450 and PR 452 in records/.

Outcome: demo PASSED, 32 claims extracted, exit code 3.

- Extraction counts reproduce the measured Python v1
  extraction on this report exactly: pr 10, commit 5,
  gitsha 1, tally 16, no hash256, count, or size claims.
  The Rust port is faithful on a real report, not only on
  its unit tests.
- Four of five commit claims verify against the bundled
  repository. The fifth cites a git tree (62180cc2, line
  22) in a commit context; a tree does not resolve as a
  commit, so the claim is unverifiable. That verdict is
  the honest one and it is what sets the exit code.
- All ten PR claims verify against the bundled records
  (both PRs closed and merged), each naming its record.
- The one gitsha claim, the PR's merge commit in full,
  verifies against the bundled repository.
- All sixteen tallies are provisional and unverifiable:
  no bundled log records them, and none moves the exit.

The script's first run failed two expectations, both the
script's errors: it expected the tree cite to verify, and
it described the gitsha claim as a trial payload fingerprint
when the claim is the merge commit. The tool's verdicts
were right on both counts; the script now carries the
correction in its header, and the corrected run is the one
recorded here.

The same build window carried the local suite: 37 tests
passed, 0 failed, run from the core's compiled test binary.
That is the first local full-suite verdict for this repo on
this box, and it matches the hosted gate's count on the
same tree.
