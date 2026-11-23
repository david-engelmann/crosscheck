V1 recall hand list, written 2026-11-22, before the v1 extractor
exists, per the M1 protocol. Five reports from the fresh frozen
corpus (v1-manifest.txt), hashes re-verified against the manifest
on 2026-11-22 before reading: d01 84bcfe7d, d09 c93cb348,
d12 054a1d8b, d17 19d46062, d22 8b541e84, all match.

Method: each report read in full. Every mechanically checkable
claim in the v1 class shapes is listed once per occurrence, with
a section anchor and the claimed value. Classes: pr (explicit PR
reference), pr-bare (a bare #NNN in pull-request context, the
merged-class candidate), commit (short hex in commit context),
gitsha (40-hex), tally (a run fact: N passed, N failed, N of M,
fractions), count (N against a noun, spelled-out numbers
included), size (N against a byte unit). Not claims, excluded:
prices and spends, dates, MD5 digests (no v1 class), deployment
and thread identifiers, qualitative verdicts.

## d01 gate-report-pr452-openai-trio-2026-10-13.md (gate, 9302 bytes)

pr: PR #452 (title); PR #450 pattern (pre-lane); PR #450's eight
files (builder); PR #450's accepted pattern (CodeRabbit);
PR #452 merged (merge section)
pr-bare: #450 mirrored (standards); #450's accepted pattern
(standards, inherited verbatim); #450 written disposition
(standards); #450 precedent (standards, parenthetical)
commit: local head fdf3201c; pushed head a1cfc044; tree 62180cc2;
sha=a1cfc044 (merge section); PR parent 16ac181d; a1cfc044
(owner gates, worktree at)
gitsha: merge commit b3d3f64c3beb62bbc82d08bf8463c0cd932402b6
tally: 3915 passed and 9 failed (builder Rust-active); 3877
passed and 0 failed (builder no-Rust); 200 passed (builder
targeted); 3915 passed and 9 pre-existing (red team); 3877/0 and
200 (red team, repeated); 3915 passed and 9 failures (owner
gates); 3877 passed and 0 failed (owner gates); 11 of 12 tests
(builder falsifiability); 11 of 12 (red team, luna); 3 of 4
(red team, astra logprobs); 4/4 corrected shape (pre-lane)
count: 7 probes per model; three JSONL logs; 133 models visible;
eight files (PR #450's); six points (spec fidelity); 39 tests
mirrored; 54 to 56 entries (pricing diff, two values); 5 added
lines (U+2014 scan); two rewrapped comments and one test comment
(inside the scan sentence, two values); two Minor comments
(CodeRabbit); nine quoted error strings; four pre-merge checks;
11 tests (standards falsifiability); 4,701 cases (runs header);
4,019/4,701 partial (terra resume); 2,065 and 3,404 cases (astra
partials); 111 attacked parse_failures; 2 benign parse_failures;
1 attacked api_error; 28 attacked parse_failures; 6 attacked
api_errors; 1 attacked parse_failure (astra); n_eligible 4,541;
n_eligible 4,602; n_eligible 4,567; first of 16 (terra rank);
16 sealed artifacts; 13 pre-existing rows; three new rows
size: 187,981 bytes (staging render)

d01 total: 66 claims (pr 5, pr-bare 4, commit 6, gitsha 1,
tally 15, count 34, size 1)

## d17 canary-pod-report-2026-10-15.md (closure, 4204 bytes)

gitsha: merged main 5b8effc81aa444889f882c946a65687e7776092e;
adapter pin a0a01d6f8135298f400a8c856b355793012ae971
commit: peira-decider @ 2d305a53; peira-strands-decider @
3fc64d81
tally: script exit 0 (Strands canary); exit 0 (Decider canary)
count: three failures preceded the pass; one early failure
size: 24 GB (RTX 3090 VRAM stated as a quantity);
gpu_mem_allocated_mb=1868 (MB quantity in an identifier)
Excluded as not claims: the instance id, the port number,
driver and package version numbers, model size names (0.8B,
2B), and every confidence and score decimal in the tables.

d17 total: 10 claims (gitsha 2, commit 2, tally 2, count 2,
size 2)

## d09 a6-run-plan-redteam-report-20261001.md (audit, 15232 bytes)

pr: PR #309 (title); peira#309 (review header); PR #309 head
(completion note)
commit: main @ 19176ee7 (base); worktree @ cc1a6395; PR head
1d501863 (task cited); 1d501863 (head-sha section); main @
19176ee7 (completion note, repeat)
gitsha: blob 14f65958020e497f456874a6a6b528fdfa997df3; PR head
1d501863bfa6778bb03f10074e2a1daa9e368713
count: 20 cases; 10 score; 7 abstain; 3 choice; 40 case-arm
pairs; max 3 runner attempts; max 2 provider requests; 240
requests (upper bound); 240 requests per adapter (quoted doc);
80-request figure; 40 expected case; 40 calls (cost basis);
1000 prompt tokens; 300 completion tokens; 40 arm-prompts; max
826 chars; 512 max_tokens default; 2 files (diff); 190 lines;
5-line backing row; six verification sections; 3 commits;
2 changed files; no P1; one P2; one P3; three times (D-33
invoked); four adapters (scope a); four-adapter smoke instance;
one of these four; A6 four adapters; four different adapters;
four A6 sections; zero usage delta; 240 ceiling (completion
note)
Excluded as not claims: the Decisions.md line range
#1188-1210 (a bare number that is not a pull request), line
cites (llm.py:616 and siblings, Adapters.md lines 339-441),
ledger identifiers (the CG numbers), HTTP status codes
(408/409/429/5xx), money amounts, and decimal measurements
(506.5 chars, usage readings).

d09 total: 45 claims (pr 3, commit 5, gitsha 2, count 35)

## d12 perfection-data-audit-cycle1-2026-10-23.md (audit, 17473 bytes)

pr: PR #476 (D1); PR #460 (D2 table); PR #460 (D2 prose, commit
message); PR #458 (tax keyed); PR #476 (slice rows section)
pr-bare: #460 was fixing (D2 prose)
tally: 396 of 396 row fields (headline); 26 of the 29 models
(P1); 36/36 rows reconciled; 37/37 artifacts verify; 27/27
taxes recomputed; 7/7 tie bands; 36/36 registry rows; 36/36
calibration panels; 26/26 scatter points (all counts section);
36/36 run register (clean section)
count: 36 rows (headline); 396 row fields; 37 artifacts; two
defects; 27 full-suite rows; one row per metric; five 373-case
slices; five rows at benign 1.0; 33 vs 35 displayed (D2
headline, two values); three unrelated models (D2 headline);
three unrelated models (D2 body); 33 tested, 29 ranked,
4 flagged (printed tiles); 36/32/4 (pair rule row); 35/31/4
(display rule row); 35 distinctly named models; 3 entries (tie
band); top 3 intervals; 2 models tied (quoted tile); three 1.0
cards; 4,701 cases x6 (D1, P2 twice, P3, Clef twice); 373 cases
x3 (P2, slice rows, calibration); 364 and 4,640 eligible bases;
1,610 cases (gemini re-run); 3,274 cases (kimi re-run); 9,402
priced calls; 10 excluded rows; 36 as a count x8 (P4 rows agree,
staged artifacts x2, card entries, hero rows, panels, pairs);
11 families x3 (tiles note, P-clean families, heatmap cells);
376 recorded cells; family bases 41 and 470; 15 reliability
bins; 4 unranked rows; five slice rows (tax None); defects 2
and questions 4 (counts section); n_eligible 4,600 (celeris
anchor); n_eligible 4,615 (Clef); two wires x2 (luna wires,
two-wire case); one shared helper; 26 labels and 26 points
(P1); 26 labels and 26 points (clean section repeat);
26 plotted and 10 excluded (scatter set)
Excluded as not claims: decimal metrics and percentages
throughout, money amounts, MD5 digests (no v1 class), the two
truncated sha256 prefixes printed with ellipses (a hash-prefix
shape with no v1 class), rank positions, and the 1:1 stem
ratio.

d12 total: 87 claims (pr 5, pr-bare 1, tally 10, count 71)

## d22 gate-report-score-fallback-fix-2026-10-22.md (closure, 29824 bytes)

pr: PR #478 (intro); PR #480 x3 (fix section, CI line,
close-out); PR #481 x8 (fix-forward, reviews, tidy, gates
headings x2, rebased gates heading, merged heading,
close-out); PR #477 x5 (rebase section heading, merge of,
8 files, files byte-identical, Mercury section heading)
pr-bare: #480 merged (addendum heading); #480 Spec review
(the miss); #480 entry (GOAL.md list); #480 Spec review
(close-out); #477 altered (rebase survival)
commit: 12a9d025 x5 (base, revert restore, smoke control, CI
baseline, pre-merge fetch); 0812a5db x3 (fix commit, suites
ran on, token-identical to); e0115530 x2 (remote, PR head);
tree 3af44903 x2; 8e97807a x4 (standards commit, comments-only,
smoke re-run, adapter files at); 78e5bc2d x3 (remote,
CodeRabbit head, pre-merge head); tree 36ee9159 x2; c98cc2d1
x4 (merge grep, addendum verified at, 481 base, close-out);
850c591e x5 (481 commit, reviews of, gates on, reconciliation,
red-team audited); 730e9406 (remote); tree 068f30a3; 5838c01d
x4 (tidy local, final-head heading, reconciliation, red-team);
a749e57c x2 (tidy remote, final-head heading); tree a3d5a5e8
x2; 1bf0407b x5 (new main, rebased onto, CI baseline,
pre-merge, merge diff base); ba4360c4 x5 (branch head, gates
heading, CI on, mergeable at, rebased local pair); 4364f22a
(rebased local); tree f552f10b x2 (gates heading, post-merge
tree); tree e04c34c6 (first push); cbcc0af0 x2 (post-merge
main, close-out)
gitsha: merge commit c98cc2d122deb70fd193b6789321fb914b12bef1;
merge commit cbcc0af06d5e9f0b8d01f41f24c8db623cbb62eb
tally: 9,402 lines x2 (step 0, Clef audit); 1,412 score records
x4 (table, corroboration, Clef each, Clef all); zero tallies in
the step 0 table x4 (missing score, engagements, non-canonical,
fabricated); zero engagements x2 spelled (Clef); zero
engagements (corroboration); six new tests FAIL (480 revert);
151 tests pass x4 (all pass, 151 passed twice, 151/151);
9 failed, 4292 passed, 3 skipped (480 Rust-active); 4254
passed, 50 skipped (480 no-Rust); exit tallies x6 (pytest 1,
no-Rust 0, surface 0, smoke control 2, smoke final 0, surface
final 0); 6/6 new tests fail (owner verification); six new
tests fail (Clef revert); 53 tests pass (Clef file); 210 group
x3 (210/210, 210-test pass, 210 from three files); 4,297
passed, 10 failed (850 Rust-active); 4,260 passed (850
no-Rust); 4,304 passed, 9 failed (final Rust-active); 4,266
passed (final no-Rust); 289 tests pass (rebase behavioural);
4,382 passed, 11 failed (rebased Rust-active); 4,345 passed
(rebased no-Rust); 31 passed (site ingest isolated); 2 passed
(s8b isolated); 4 of them verbatim (tidy semicolons)
count: exactly two copies; two adapters and two test files
(seven files sentence); seven files (fix); 24 CI checks;
7 files (post-merge diff); 144 insertions and 32 deletions;
9 files x3 (rebase diff, CodeRabbit nine spelled, post-merge);
254 insertions; 8 files (477 delta); five files (481 fix,
spelled); four briefed cases; four named test cases (spec
axis); 3 per adapter (revert); 3 huge-int and 3 parity tests;
6 tidy tests; four affected test files; 3 actionable comments;
44-case battery; 57 in-file tests (Clef); 5 added comment
lines; two CHANGELOG semicolons; one semicolon (Clef test);
zero U+2014 and zero semicolons x2 (standards axis, tidy scan);
two low notes (red team); three adapter files (210 sentence);
three helpers (F4)
Excluded as not claims: HTTP status codes, score levels,
durations in seconds, load averages, the 10**400 test value,
and the two truncated SHA-256 prefixes (08ba514e, d81e502d)
printed with ellipses.

d22 total: 157 claims (pr 17, pr-bare 5, commit 56, gitsha 2,
tally 48, count 29)

## Hand-list totals (five reports)

365 claims: pr 25, pr-bare 10, commit 64, gitsha 5, tally 75,
count 181, size 3. (d01 66, d09 45, d12 87, d17 10, d22 157.)
Hash256 (64-hex) claims in the five reports: none; the class
carries no recall signal in this corpus, as in v0.
