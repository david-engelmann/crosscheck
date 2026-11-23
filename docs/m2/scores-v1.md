V1 extraction scoring record, 2026-11-23. Pattern set v1
(extract_v1.py) run over the fresh frozen corpus
(v1-manifest.txt, 24 reports, 264,736 bytes). Every extracted
claim hand-adjudicated TP or FP (FP1 not a claim, FP2 wrong
type, FP3 wrong evidence target, FP4 duplicate), the v0
taxonomy. Totals mechanically reconciled against the
extractor's per-class counts after adjudication; the
reconciliation caught and corrected one miscount in this
record (d21 pr) and four subtotal slips in the hand list's
totals block (corrected in the results note).

Extractor totals: 726 claims. pr 172, commit 174, tally 246,
count 89, gitsha 23, size 21, hash256 1.

Per-report adjudication (TP counts per class; every FP named):

d01: pr 10 TP. commit 5 TP. tally 16 TP. count 3 TP.
gitsha 1 TP. size 1 TP. No FPs.
d02: pr 3 TP. commit 6 TP. tally 10 TP. size 1 TP. No FPs.
d03: pr 5 TP. commit 9 TP. tally 8 TP. No FPs.
d04: pr 4 TP. commit 5 TP. tally 10 TP. size FP1 x1
("1, byte" from a tree SHA fused to "byte-identical").
d05: pr 6 TP. commit 6 TP. tally 10 TP. count FP1 x1
("422 error", an HTTP status code).
d06: pr 14 TP. commit 15 TP. gitsha 1 TP. tally 14 TP,
FP2 x1 ("13 test" from "13 test lines", a line count).
count 4 TP, FP1 x1 ("460 entries", the endpoint of the
#457-#460 range fused to the noun).
d07: pr 8 TP. commit 15 TP. gitsha 1 TP. hash256 1 TP.
tally 33 TP, FP1 x1 ("01 of 1" from "0.01 of 1", a decimal
fragment through the of-form). count 5 TP.
d08: pr 8 TP. commit 9 TP. gitsha 3 TP. tally 10 TP.
count 1 TP. size 10 TP, FP1 x1 ("6,200 MB", a kill threshold,
not a measured quantity).
d09: pr 3 TP, FP1 x1 ("PR 1188" via bare, from the
Decisions.md line range #1188-1210; not a pull request).
commit 5 TP. gitsha 2 TP. count 6 TP. tally FP1 x1
("1 of 3" from "checkpoint review #1 of 3", document
metadata, not a run fact).
d10: commit 4 TP. count 2 TP. tally 6 TP, FP1 x1
("1 of 3", same review-metadata shape as d09).
d11: pr 10 TP. commit 8 TP. count 2 TP. tally 9 TP. No FPs.
d12: pr 6 TP. tally 1 TP. count 7 TP. No FPs.
d13: count 7 TP, FP1 x1 ("01 suite" from the C-01 check
label). tally 3 TP, FP1 x2 ("26 of 35", a rank position;
"01 test" from the SO-01 test label).
d14: pr 7 TP. commit 10 TP. tally 15 TP. count 12 TP,
FP1 x1 ("1 files" from "HEAD~1 files", revision syntax).
d15: pr 1 TP. gitsha 1 TP. commit 4 TP. tally 1 TP,
FP4 x1 ("38 tests" restating the denominator of the
"32/38 tests" fraction claim on the same line).
d16: pr 12 TP. commit 8 TP. tally 8 TP. count 12 TP,
FP1 x1 ("1 suite" from "PEIRA_NO_RUST=1 suite", an env
assignment fused to the noun).
d17: gitsha 2 TP. tally 2 TP. count 1 TP. size 1 TP.
No FPs. (Both hand-listed short commits missed: their
lines carry no commit-context word.)
d18: pr 1 TP. commit 2 TP. count 5 TP. size 2 TP,
FP1 x3 ("3,800 MB" twice, a memory requirement threshold;
"3 GB", the endpoint of a "~2-3 GB" range).
d19: pr 2 TP. commit 5 TP. tally 9 TP. No FPs.
d20: pr 3 TP. commit 1 TP. gitsha 1 TP. tally 2 TP.
count 1 TP. No FPs.
d21: pr 28 TP. gitsha 5 TP. commit 4 TP, FP2 x1
("08ba514e", a truncated SHA-256 prefix typed as a
commit). tally 10 TP, FP1 x2 ("01 of 1" decimal fragment;
"480 test" from "#480 test shapes", the PR number fused
to the tally word). count 2 TP.
d22: pr 29 TP. gitsha 2 TP. commit 38 TP, FP2 x2
("08ba514e" and "d81e502d", truncated SHA-256 prefixes
typed as commits). tally 33 TP. count 5 TP, FP1 x2
("409 error", an HTTP status code; "274 comment" from
the "line-274 comment" line cite).
d23: pr 9 TP. commit 11 TP. gitsha 1 TP. tally 14 TP.
count 4 TP. No FPs.
d24: pr 2 TP. gitsha 3 TP. commit 1 TP. tally 9 TP.
count 2 TP. size 1 TP. No FPs.

Class totals, reconciled to the extractor's counts:
pr 172 = 171 TP + 1 FP. commit 174 = 171 TP + 3 FP.
tally 246 = 237 TP + 9 FP. count 89 = 82 TP + 7 FP.
gitsha 23 = 23 TP. size 21 = 16 TP + 5 FP.
hash256 1 = 1 TP. Total 726 = 703 TP + 23 FP.

False positive rates: pr 0.6% (merged class; explicit
0 of 81, bare 1 of 91 = 1.1%). commit 1.7%. tally 3.7%.
count 7.9%. size 23.8%. gitsha 0.0%. hash256 0.0% (n=1).

FP mechanisms, v1:
- Of-form positions and metadata (tally, 3): "review #1 of
  3" twice, "Rank 26 of 35". The of-form asserts a run fact
  only sometimes; document series and rank positions are
  the same shape.
- Decimal fragments through the of-form (tally, 2):
  "0.01 of 1" fires "01 of 1". The of-form never got the
  number guard the count pattern received.
- Truncated hash prefixes as commits (commit, 3): an
  8-hex SHA-256 prefix in commit context is
  indistinguishable from a short SHA by shape alone.
- Fused tokens the guard does not cover (count, 4, tally
  2): the v1 guard refuses a number preceded by a letter,
  digit, dot, or underscore; the surviving fusions are
  preceded by "-" (C-01), "~" (HEAD~1), "=" (NO_RUST=1),
  "#" (#480 test), or are range endpoints and line cites.
- HTTP status codes against the error noun (count, 2).
- Thresholds and range endpoints (size, 4): a kill
  threshold, a memory requirement stated twice, and the
  top of a "~2-3 GB" range. The decimal-fragment mechanism
  that dominated v0 size FPs is gone (zero instances).
- The "byte-identical" fragment (size, 1): a tree SHA's
  trailing "1," fused to the word "byte". Present in v0
  too ("26, byte").

Recall, against the hand list (five reports, 365 claims;
class subtotals corrected during reconciliation: pr 40
merged, commit 69, gitsha 7, tally 75, count 171, size 3):
- pr: every hand-listed claim matched (48 TP extractions
  in the five reports against 40 listed; the surplus is
  hand-list undercount of repeated heading references,
  found during matching). Recall effectively complete.
- gitsha: 7 of 7, 100%.
- commit: 48 of 69, 70%. Misses: short SHAs on lines with
  no commit-context word (both d17 adapter pins), pure
  digit SHAs (refused by design), and occurrences inside
  long SHA-dense lines the context gate did not reach.
- tally: 52 of 75, 69%. Misses: spelled-out zero tallies,
  the step-0 table's bare zero values, tallies split by a
  line wrap ("4292" at one line's end, "passed" at the
  next), and X/Y forms whose noun is outside the fraction
  list ("36/36 rows").
- count: 22 of 171, 13%. The noun list is narrow by
  design and spelled-out numbers are out of scope; most
  hand-listed counts use nouns the pattern does not carry
  (requests, attempts, pairs in parentheses, rows, models).
- size: 2 of 3, 67%. The miss states the unit before the
  number ("gpu_mem_allocated_mb=1868").
