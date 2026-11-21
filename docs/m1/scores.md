# M1 adjudication record (pattern set v0, scorer: this seat)

Rules applied uniformly, per the protocol in
docs/M1-EXTRACTION-MEASUREMENT.md. FP reasons in one line each.
pr-bare on a line where the pr class fired the same number is FP4
by construction. A quote scores TP only when the span is presented
as verbatim text from the named target file.

## c01 (36 claims)
pr 2 TP. pr-bare 1 FP4 (dup of pr, line 1). commit 12 TP. tally
12 TP. quote: 1 TP (Frontier ceiling, Adapters.md); FP1 x2 (pytest
-k filter string; code literal claude-fable-5-1); FP3 x1 (README
span, target taken as Adapters.md). count 5 TP.

## c02 (28 claims)
pr 4 TP. pr-bare: 2 TP (#191 lines 42 and 69, no pr-class hit on
those lines); FP4 x3 (lines 1, 40, 153). commit 4 TP. tally 4 TP.
hash256 1 TP. count: 5 TP; FP1 x1 ("0 defect" fired on "P0
defect", not a count). quote: 1 TP (Taxonomy.md entry); FP1 x3
(commit message as file quote; two mangled nested-quote fragments).

## c03 (22 claims)
pr 3 TP. pr-bare FP4 x4 (every bare hit duplicates an explicit PR
mention on the same line). commit 5 TP. count 8 TP. quote: 1 TP
(script output string, in the named script); FP3 x1 (check output
span, target taken as Taxonomy.md instead of the script).

## c04 (11 claims)
pr 1 TP. pr-bare 1 FP4. commit 4 TP. tally 5 TP.

## c05 (20 claims)
commit 5 TP. pr 2 TP. pr-bare 2 FP4. count 3 TP. tally 7 TP.
quote 1 TP (script output string, named script).

## c06 (36 claims)
pr 2 TP. pr-bare 2 FP4. gitsha 3 TP. commit 10 TP. tally 15 TP.
count 2 TP. quote FP1 x2 (git-grep fragment; David's ruling quoted,
target taken as a docs file).

## c07 (43 claims)
pr 5 TP. pr-bare: 5 TP (#197, #451 x3, #449 standalone lines);
FP4 x5. commit: 10 TP; FP2 x1 (md5 fragment 38465a72 typed as
commit). gitsha 1 TP. tally 12 TP. count 3 TP. size 1 TP.

## c08 (41 claims)
pr 4 TP. pr-bare: 3 TP (sibling PRs #442/#446/#447 on one line);
FP4 x4. gitsha 4 TP. commit: 5 TP; FP2 x1 (truncated sha256
b7054287 typed as commit); FP1 x1 (CodeRabbit run id 64d73194).
hash256 3 TP. tally 13 TP. size: 1 TP; FP1 x1 ("26, byte" fired on
a blob SHA plus the word byte-identical). quote FP1 x1 (David's
ruling, target taken as metrics.py).

## c09 (34 claims)
pr 1 TP. pr-bare: 1 TP (#410), 1 FP4. commit: 1 TP; FP2 x1
(truncated manifest sha b8b877fa typed as commit). tally 1 TP.
count: 18 TP; FP1 x4 ("2 families" from v2 families, "2 files"
from v2 files, "1 commit" from v1 commit, "2 suite" from v2
suite); FP4 x1 ("9 warning", mangled duplicate of 9,712 warnings).
quote: FP1 x4 (two mangled fragments, a code key, a fragment);
FP3 x1 (docstring quote, target taken as the tags script).

## c10 (12 claims)
pr 4 TP. pr-bare 4 FP4. tally 1 TP. count FP1 x2 (the canary
"10 cases" is a recommendation, not a measured claim, twice).
size FP1 x1 ("7GB" extracted from 7.7GB).

## c11 (9 claims)
pr 1 TP. pr-bare 1 FP4. tally 5 TP. count 1 TP. commit 1 TP.

## c12 (8 claims)
count: 6 TP; FP1 x2 ("10 prompts" fired on the 0-10 scale, real
claim is 3 prompts; "4401 pair" fired on the K-4401 identifier).

## c13 (70 claims)
commit 7 TP. pr 6 TP. pr-bare: 47 TP (standalone PR references in
an audit of PR states; the bare pattern is the only pattern that
fires on most of them); FP4 x6. count 3 TP. quote 1 TP (verbatim
charter quote, correct target).

## c14 (17 claims)
commit 1 TP. count 16 TP.

## c15 (8 claims)
commit 2 TP. count: 4 TP; FP1 x2 ("1 findings" and "2 findings"
fired on the severity labels P1 and P2, not counts).

## c16 (24 claims)
pr 3 TP. pr-bare 2 FP4. commit 8 TP. count 8 TP. tally 1 TP.
quote: 1 TP (SPEC.md rule, correct target); FP1 x1 (a searched
person's name typed as a quote from fairness.py).

## c17 (65 claims)
size: 21 TP; FP1 x5 (decimal fragments: "9 GB" from 16.9 GB,
"16 GB" from 4.16 GB, "1 GB" from 15.1 GB, "41 GB" from 10.41 GB,
"4 GiB" from 53.4 GiB). count: 37 TP; FP1 x1 ("3 finding" from
peira-cycle3). commit 1 TP.

## c19 (25 claims)
gitsha 1 TP. commit 3 TP. tally: 17 TP; FP1 x1 ("97560 test"
fired on the commit SHA 60f97560 followed by the word test).
count 2 TP. quote 1 TP (new subsection heading, correct file).

## c18 (133 claims)
count 87 TP (every per-path file count is a real claim). commit
3 TP. size: 23 TP; FP1 x20 (19 decimal fragments of the form
"9 MiB" from 223.9 MiB, "8 GiB" from 17.8 GiB, and one threshold,
the 12 GiB cron alert level, which is not a measurement).

## c20 (19 claims)
commit 2 TP. gitsha 1 TP. pr 1 TP. pr-bare 1 FP4. size 1 TP.
tally 12 TP. count FP1 x1 ("1 comment" from the P1.1 label).

## c21 (48 claims)
gitsha 3 TP. tally 1 TP. size: 24 TP (RAM readings and box
properties); FP1 x20 (decimal fragments such as "5 GB" from
2.5 GB and "47 GB" from 2.47 GB, the 5,500 MB launch threshold
stated four times, range bounds extracted as point values).

## c23 (5 claims)
count 2 TP. commit 3 TP.

## c24 (32 claims)
commit 11 TP. count: 11 TP; FP1 x1 ("2 families" from v2
families). tally: 8 TP; FP1 x1 ("5 passed" fired on gate G5).

## c22 (82 claims)
gitsha 16 TP (merged-main head and model revision pins). commit
4 TP. pr 4 TP. pr-bare: 4 TP; FP4 x4. count: 8 TP; FP1 x2 (a
mangled fraction, "9 cases" from an elided unit). size: 29 TP;
FP1 x11 (decimal fragments: "9 GB" from 41.9 GB, "38 GB" from
16.38 GB, "07 GB" from 16.07 GB, and kin).

## Reconciliation corrections (mechanical recount against the
extractor's per-report and per-class totals, 831 overall)
c01 tally is 13 TP (recorded as 12 in its section; the running
totals already used 13). c09 commit is 2 TP (the base SHA was
missed in its section). c15 count is 5 TP (the 0 warnings row was
missed in its section). c08 size is 1 TP and 1 FP (the TP was
dropped from one running-total edit). All class totals below are
reconciled to the extractor's own counts: pr 43, pr-bare 103,
commit 107, tally 117, quote 23, count 247, hash256 4, gitsha 29,
size 158.

## FINAL (831 claims, TP 693, FP 138, overall FP rate 16.6%)
pr: 43 TP, 0 FP — 0.0%
pr-bare: 62 TP, 41 FP (all FP4) — 39.8%
commit: 103 TP, 4 FP (3 FP2, 1 FP1) — 3.7%
gitsha: 29 TP, 0 FP — 0.0%
tally: 115 TP, 2 FP (FP1) — 1.7%
quote: 7 TP, 16 FP (13 FP1, 3 FP3) — 69.6%
count: 230 TP, 17 FP (16 FP1, 1 FP4) — 6.9%
hash256: 4 TP, 0 FP — 0.0%
size: 100 TP, 58 FP (FP1) — 36.7%

## Recall estimate (hand-listed before extraction, 182 claims)
Matched hand-listed claims against TP extractions per report:
c01 27 of 33 (82%), c03 10 of 24 (42%), c09 21 of 55 (38%),
c14 14 of 40 (35%), c19 21 of 30 (70%). Overall about 93 of 182,
roughly 51%. The misses concentrate in bare-number claims with no
adjacent noun (diff stats, severity mixes, table cells), spelled
out numbers, and fraction forms. Estimate on five reports only,
per the protocol.
