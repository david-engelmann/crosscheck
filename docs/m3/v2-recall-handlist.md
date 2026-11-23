V2 recall hand list, written 2026-11-23, before the v2
extractor exists, per the M1 protocol. Five reports from the
fresh frozen v2 corpus (v2-manifest.txt): e01, e09, e13, e17,
e19. Their hashes are the manifest values, recorded at freeze
time the same day from these files. Method and exclusions as
in the v1 hand list; the PR class is the merged class per
D15. Truncated hash prefixes printed with ellipses are
excluded, as before.

## e17 gate-report-fleet-pins-landings-2026-10-23.md (closure, 9721 bytes)

pr: #491 (PR section); PR #491 (CodeRabbit findings)
commit: base 9d1b5c02 x2 (header, PR section); local commits
6566f8b3, 5488cbc6, bb8e66d9, 3dfa02ed, 9aec3a24, 947f8217;
fix commit 943e4330
gitsha: cerno-9b revision 80010f97eaa989b41c908ecba7d7b7fadaa99239;
cerno-27b revision c2f7efe54fe4d3dfd7136bb2e9daf2c0c0633e3d;
head 7a1a572aa1d519ba3d071a6c7960c6f89cada82e; final head
943e4330753a64a99c43e9f4060df9593ea95ac8; wildguard revision
cbba4823f3e8020e5a74a5e29bf85072def6f2ff
hash256: the eleven per-file staging hashes (config,
generation_config, index, special_tokens_map, tokenizer.json,
tokenizer.model, tokenizer_config, model shards 1 to 4)
tally: 156 tests pass; 35 subtests; 4448 passed; 11 failed;
exit code 0 (no-Rust suite)
count: canary n=7; 23 categories; 1568 chars; five table rows;
five test classes; five HF revisions; two pricing tests
(asserting); 13 files inventoried; 9 files changed; 9 files
for push; 3 inline findings; 4 partial blobs; 2 pricing
version tests (gate table); 8 of one failure class; 9 known
failures
size: 18,505,338,955 bytes total; the eleven per-file byte
sizes in parentheses beside their hashes; 3.0 GB of 15 GB
downloaded; 3 GB of 15 GB (deviations repeat)
Excluded: the subagent id 59c22aab (an identifier, not a
commit), the five truncated revision prefixes printed with
ellipses, ports and version numbers.

e17 total: 63 claims (pr 2, commit 9, gitsha 5, hash256 11,
tally 5, count 15, size 16)

## e09 a6-run-plan-spec-review-20261001.md (audit, 12821 bytes)

pr: PR #309 (title); #309 (PR line); #304 (merge of, base line)
commit: 19176ee7 x2 (base line, footer); 1d501863 x3 (reviewed
head, verified-against line, footer); fcbb464; 76f8a12;
1d50186 (the commits chain)
tally: none asserted. The pass and abort numbers (38/40,
under 3/40, zero 401/403, over 3 consecutive, 5x estimate,
200 calls projected) are criteria the reviewed doc sets, not
results this report asserts, and are excluded on the v1
threshold precedent.
count: 20 cases x2 (trial slice, exactly 20); 10 + 10 (the
two families); 10 score, 7 abstain, 3 choice (primitives);
40 calls x3 (per adapter, corrected count, CG-0026); both
arms (2); 240 bound x5 (upper bound, command ceiling x2,
request ceiling, call ceiling); attempts default 3 x2 (one
spelled); one repair x2 (spelled); about 507 chars; the
1000 token assumption x3; 300 tokens x2; five adapters x2;
four adapters x8
Excluded: prices and spends throughout, the usage decimal,
506.5 chars (decimal measurement), all line cites (4527,
4692, 653, 764, 294-304, and the Decisions.md line 1188, the
v1 line-cite trap in new dress), HTTP codes, CG ledger ids,
D-33 (a decision id).

e09 total: 47 claims (pr 3, commit 8, count 36)

## e13 noise-redteam-review-20261001.md (audit, 15080 bytes)

pr: pull request #325 (the merge line); #52 x4 (the rule,
the human review, the validation fixture, the rubric)
commit: ecce49c3 x2 (lane head, HEAD at end); 23242ab9 x6
(merge-base line, survival section, four command rows);
1854dc4e x6 (current main line, survival section x2, grep
line, fetch row, caveat); 6e0a2a5e (the stale HEAD);
2fc4b6e5 (the other lane's branch)
tally: 42 passed, 35 passed, 9 passed (clean items);
42 passed, 35 passed, 9 passed (commands table repeats)
count: 1800 pairs; 156 of 300 seeds; 35 assertions; 2000
fuzz runs; 4000 seeds; 2 pairs from duplicate ids; 9 failure
modes; five metrics x2; 0 violations x2 (paraphrase,
digits); 0 em dashes, 0 colons, 0 semicolons; four pairs or
classes x3; two defects; both generators (2); 40 pair
fixture; 10 sampled cases; 14 files; 2523 insertions;
0 deletions; 5 modified; 9 new files; 3 hunks; 2 conflicted
files; 11 untouched files; 3 commits touching shared files;
108 added lines; a single FAIL
Excluded: EB lane ids, line cites (noise.py:40-45 and the
rest), the float example values, version-like tokens.

e13 total: 59 claims (pr 5, commit 16, tally 6, count 32)

## e01 gate-report-clef-adapter-2026-10-15.md (gate, 18314 bytes)

pr: #462 x4 (title, PR line, squash-merge line, final merge
line); #461 (base line); #459 x2 (the owner's ruling, the
post-#459 form)
commit: 38c6ef32 x3 (main head, driver checkout, pre-merge
check); 1a53f00f; e7c0a265 x7 (five findings, the test-gap
finding, the repair list); 401bea64 x3; 2d1dc2ac x4;
32ba62e8; 21e3869b x3; dbeb956f; 1dbb4eb5; 5b8effc8 x4
(title, driver fast-forward, merged-main grep, final line)
gitsha: the base full SHA x2 (base line, rebase command);
20fc45ba424f069f50ce4248e46d7ef219efc31e (new tree);
2b431d4a638db2d1f221920e9a419347e3eb41c7 (post-repair tree);
5b8effc81aa444889f882c946a65687e7776092e (merge commit)
hash256: b08c2c515cf3232788099afaa2c41dcc9fdee49dc797e69c993dc778529b1796
(the Clef artifact digest)
tally: 159 passed and 15 subtests; 168 passed and 15
subtests; 4032 passed, 3 skipped, 889 subtests, 11 failed;
4034 passed, 3 skipped, 889 subtests, 9 failed; 3996 passed,
50 skipped, 743 subtests, 0 failed; 4035 passed, 3 skipped,
889 subtests, 9 failed; 3997 passed, 50 skipped, 743
subtests, 0 failed; 69/69 (the pin file); exit 0 x6 (the
check scripts and the smoke); 9 of 10 spec items; 10/10
canary cases x3
count: 7 lane files x2; 1,153 insertions; 5 deletions;
8 files x2; one partial finding; nine new tests x2; nine of
the failures (spelled); exactly 2 em dashes x2; 3 inline
findings; 5 api_error arms; 3 and 87 (the ineligible
census); 4,701 results x2; 1,575 partial results; 4,554 and
4,611 eligible; two red checks; about 340 input tokens;
0 output tokens; zero errors or rate limits x4 (the
canaries); zero error records (the flash seal)
size: 16,009,997 bytes (the Clef artifact)
Excluded: the two MD5 digests (no class), rank 14 of 23
(a position), prices, rates and decimal metrics, the
publish batch numbers, HTTP status codes throughout,
line cites.

e01 total: 106 claims (pr 7, commit 28, gitsha 5, hash256 1,
tally 35, count 29, size 1)

## e19 gate-report-presentation-lane-2026-10-15.md (closure, 31936 bytes)

pr: #464 (Von, base line); #471 (Strands); #473
(Perplexity); #476 x3 (opened, merged, the landed entry);
#475 (Decider)
commit: 13cb9ebd x3; 9f13bfd2 x3; 0277f208 x3; 5d01bb7f x2;
9648e9a1; ac92bda8 x3; 1481d259; efd14cca x4; 06f19579 x4;
5b0f85d1; 7b5a6554 x3; 4fb09f5d x2 (the pushed tree);
e6be8729 (the remote branch); 65589dfb (main advanced)
gitsha: 13cb9ebdc9595e4f3415be60820037b2df262d00 (base);
4fb09f5dae76c7601845f521f07ea3484d5e38ed (pushed tree);
f0e243bb5699cb3028aa12f9fc925e721c31bb47 (merge commit)
tally: 50 passed x2 (focused, both modes); 52 passed;
57 passed x5 (the repair, the Step 5 re-run x2, the Step 7
behavioral check x2); 101 passed x2; 11 failed and 4098
passed; 2 failed and 4060 passed; 9 failed, 4101 passed,
3 skipped; 4063 passed, 50 skipped, exit 0; exit 0 x4 (the
surface and CLI smokes, Steps 9 and 11); 9 failed, 4194
passed, 3 skipped, exit 1; 4156 passed, 50 skipped,
exit 0; exit 2 x3 (the smoke environment failures)
count: 7 canary cases x2; 9 new tests; two fallback tests;
five new tests; all 9 new tests (mutation); three
regression tests; two board tests; one regression test
(Step 8); 4 of 5 display tests; six files; six lane files
x2; seven lane files x2; 19 vs 18 ranked; 18 shared rows;
4 unranked rows; unranked 1 to 0; 373 cases x2; 8 banned
phrase hits; 8 places reworded; 93 tests from the merges;
+21 lines; +42 lines; two full-suite artifacts staged;
cases=40 x2 (the smoke exercises)
size: 29,357 bytes and 29,530 bytes (the two canonical rows)
Excluded: the two truncated 12-hex revision pins, decimal
metrics throughout (taxes, ASR, ECE), percentages, times,
the reconciliation arithmetic whose parts are counted
where they stand.

e19 total: 110 claims (pr 7, commit 32, gitsha 3, tally 34,
count 32, size 2)

## Totals, five reports

385 claims: pr 24, commit 93, gitsha 13, hash256 12,
tally 80, count 144, size 19.
