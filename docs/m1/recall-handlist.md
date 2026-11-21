# Recall hand-list, written before the extractor was run

Five reports read in full on 2026-11-21. Every checkable claim in
the six pattern classes (plus PR and commit references, which the
plan counts inside classes 2 and 4) is listed per report. Version
strings, money figures, dates, and agent IDs are excluded by the
class definitions. Counts are claim instances.

## c01 gate-report-pr232 (gate) — 33 claims
PR #232. Heads ee68243, 84169f1c, tree cb7120e5, base 92160b0.
Focused 18 passed. NO_RUST 2110 passed, 25 skipped, 529 subtests,
exit 0. Rust 2133 passed, 2 skipped, 529 subtests, exit 0. Scan of
453 added lines, 0 hits. Red team 17 focused tests. Rebase stats
+346/-47. Reviewed f12a36d5 on base 1d1d94ef. Diff 6 files +455/-47.
b27c825 +346/-47, f12a36d +121/-12. 18 focused tests at head. Scan
of 455 added lines, 0 hits. 13 new tests. CodeRabbit 1 comment,
later 2 comments. Final head ccce5f8a. Final No-Rust 2129 passed,
25 skipped, 532 subtests, exit 0. 118 OSError failures (ENOSPC try).

## c03 gate-report-pr247 (gate) — 24 claims
PR #247, supersedes PR #188. Base e6813b2c (PR #245). Reviewed
63d3566. Diff 15 files, 1446 insertions, 31 deletions. 420 cases.
Severities 147/74/199. Primitives 300/50/70. 70 per language, six
languages. Registry 24 families, numbering 1..24. Rebased tree
2ed732fd. Eight requirements present. Three touched test files.

## c09 FINAL-AUDIT-2026-10-03 (audit) — 55 claims
Base 15382b37, PR #408. Findings 31 (4 P0, 11 P1, 12 P2, 4 P3).
Code 3 P1, 6 P2, 4 P3. Red-team 4 P1, 3 P2. Dataset 2 P0. Run path
1 P0, 2 P1, 4 P2. Tests 2 P1, 1 P2, 4 P3. Holdout 2 P1, 3 P2.
Suites: v2 3,861 cases, 9/9 gates, 0 errors. v1 2,000, 9/9, 0
errors. Conversational 840, 5/5, 0 errors. Combo 800. Safety 85,
9/9, 0 errors. Trial 100. Trial-demo 12. Five manifests verify.
0 collisions across 7,698 cases. v1 35 PII warnings. G9: v2 9,712
warnings, 160 pairs, safety 3,555 warnings, 30/60 fairness cases,
v1 399 warnings. Verified run 421 cases, manifest sha b8b877fa.
Stale cases.jsonl 25 lines, sealed suite 85 cases. 10 root jsonl
duplicates. Fixtures renumbered 801-803 against retired 901-1000.
10 tests pass. Dangling commit 81b8c659. Forgery demo 12-case run.
Mutation score 11/12. PR #410.

## c14 v2-draft-audit-report (audit) — 40 claims
Base 73327c3f. 2,201 cases, 4,402 arms. Families 470, 470, 420,
420, 421. Manual review 100 cases, 20 per family. 0 defects after
repair. 302 instances fixed: 198 article, 60 doubled-line (30 cases
x 2 arms), 44 notes. v1 abstain 200 cases. O-1 shared scenarios 5
cases. O-2 338 cases. O-3 52 cases. O-5 80 cases x 2 arms, 160
prompts. Balance table 15 counts (300/70/50, 147/74/199, 315/52/53,
80/100/240, 375/46/0, 40/137/244, 330/80/60, 90/160/220, 330/80/60,
90/110/270). Reproducibility exit 0, 0 defects, fixes 198/60/44.

## c19 anthropic BUILDER_REPORT (closure) — 30 claims
Base b3d3f64c3beb62bbc82d08bf8463c0cd932402b6 (full SHA). Head
93a705e2. Commits 60f97560, 93a705e2. Diff: 12, 18, 48 lines per
file, 3 files, 75 insertions, 3 deletions. Line refs 2926, 209,
3144, 3097. Four new tests. Fail-first 4 failed, exit 1. After fix
4 passed, exit 0. Suites 175 passed, 1 skipped, exit 0 (stated for
two modes). Per file 75 passed 1 skipped, 63 passed 40 subtests,
37 passed 100 subtests. 11 of 12 canary calls clean. Public-surface
check exit 0. Codepoint scan 0 lines with U+2014.

## Total hand-listed: 182 claims across 5 reports
