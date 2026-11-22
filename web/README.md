# Verdict viewer

A web frontend that renders one real agent work report checked
claim by claim against its evidence bundle. Every claim in the
report is shown with its verdict and the evidence behind the
verdict. It is the repository's showcase surface: demo material
under the Stageshow law. The product is the deterministic CLI in
the crates above this directory; nothing here verifies anything
at view time.

## The run it renders

data/report.md is the verifier seat's milestone report for M1 and
M2, written in the claim shapes the M1 measurement rated trusted
(explicit PR references, full git SHAs, full content hashes, run
tallies) plus the provisional short-commit class. data/run.json is
the recorded run over it, produced by scripts/generate_run.py,
which performs the checks for real:

- commit identifiers resolve in this repository's own git history;
- the content hash is recomputed from the cited file on disk;
- the test tally is recomputed from the crates' test attributes;
- the prototype tallies are recomputed by running the Phase 1
  prototype checker in-process over its honest report;
- PR claims check against data/bundle/records/github-records.json,
  real GitHub API responses fetched on 2026-11-22.

The report carries one planted error (it must come out
contradicted) and one claim whose evidence is outside the bundle
(it must come out unverifiable). A run that cannot show those two
verdicts is not demonstrating the tool.

## Run it

npm install
npm run dev

npm run build produces a static export in out/.

Regenerate the run record after the report or the evidence moves:

python3 scripts/generate_run.py
