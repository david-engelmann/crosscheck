# Pattern set v2 scoring record

Corpus: the 24 reports frozen in v2-manifest.txt (368,434
bytes, none in the v0 or v1 corpora). Extractor:
extract_v2.py, the v1 pattern set with exactly the three
changes fixed in the manifest before it was written.
Extraction run: 1,138 claims. Every claim hand-adjudicated
against its source line. Codes as in the v0 and v1 records:
TP (a checkable claim of the class), FP1 (not a claim of the
class), FP2 (wrong kind of referent), FP4 (duplicate).
Totals below are the extractor's mechanical per-class counts
minus the adjudicated false positives listed here.

## Headline

1,138 claims: 1,093 TP, 45 FP (4.0% overall).

- pr: 199 extracted, 168 TP, 31 FP, 15.6%
- commit: 331 extracted, 329 TP, 2 FP, 0.6%
- gitsha: 49 extracted, 49 TP, 0 FP, 0.0%
- hash256: 14 extracted, 14 TP, 0 FP, 0.0%
- tally: 377 extracted, 376 TP, 1 FP, 0.3%
- count: 106 extracted, 103 TP, 3 FP, 2.8%
- size: 62 extracted, 54 TP, 8 FP, 12.9%

## The tally verdict (the point of the round)

The v1 tally rate was 3.7% (9 FP of 246), over the 2% bar,
and the class dropped to provisional. The v2 changes were
aimed at its named mechanisms. On this corpus the class
extracted 377 claims with exactly one false positive:

- e23 L119 "0, passed" FP1: the exit tally's number fused
  to the verb "passed" in "exit 0, passed." Not one of the
  mechanisms the v2 changes targeted; a comma-form fusion.

The v1 mechanisms are absent from the v2 output. No decimal
fragment of-forms ("0.01 of 1"): the of-form now carries the
fused-token guard. No document-series or rank of-forms: the
series-word suppression held everywhere it could fire. No
fused-label tallies (the C-01, SO-01, HEAD~1, NO_RUST=1,
#480 shapes): the extended guard set covers them. Every
of-form that fired is a genuine proportion: 2,625 of 4,701
and 4,554 of 4,701 (e04), 24 of 27 and 10 of 10 and 4,627 of
4,701 (e05), 12 of 13 (e06), 18 of 19 (e18), 4 of 5 (e19),
23 of 24 (e20), 396 of 396 (e23). Tally rate 0.3%, under the
bar. The class's return to trusted is Main's ratification
to make, on these numbers.

## The PR class finding (the round's surprise)

All 31 PR false positives are bare references, and every one
is a GitHub ISSUE number cited in an audit review, not a
pull request:

- e12: 15 FP. The EB spec review quotes the disposition
  doc's "absorbs #14, #17" and the EB-16 build text's #14,
  #16, #17 throughout (L11, L22, L23, L27, L28, L30, L31,
  L47, L48, L110, L113). Issues, folded into EB items.
- e14: 12 FP. The noise spec review quotes the EB-1 spec's
  "(#50/#51/#52)" sub-requirement cites and the #52
  validation rubric (L20, L23, L42, L73, L75, L88, L99).
- e13: 4 FP. The red-team review's "#52 rule" and "#52
  rubric" cites (L15, L39, L40, L87), the same issue as
  e14's #52.

Every explicit form (PR #NNN, pull/NNN, owner/repo#NNN)
scored clean across all 24 reports, and the bare references
that are genuinely PRs scored clean too (e03's long #466
and #468 chains, e19's #471 and #475, e12's #197 and #239).
The v1 corpus measured the merged class at 0.6% because its
audit stratum did not quote issue trackers this densely.
The merged class's rate is therefore corpus-sensitive in a
way the v1 number did not show: 0.6% on the v1 corpus, 15.6%
on this one, with the mechanism isolated to bare references
in review reports quoting disposition documents. The
disposition this suggests (for Main, not taken here): split
the class by form, explicit forms trusted, bare forms
provisional. That is a v3 design question; v2 changed no PR
pattern, per its manifest.

## Commit

Two false positives, both the mechanism named when the
class went trusted in D15: e15 L51 "99bbf834" and "1dbfa831"
are truncated SHA-256 prefixes of dataset files, typed in a
sha256-chain sentence with commit context words present.
2 FP of 331, 0.6%, better than the v1 round's 1.7% on a
larger sample. The standing risk stands as named.

## Count

Three false positives, 2.8%, inside the provisional band
for the first time:

- e01 L120 "0, error" FP1: the exit tally's 0 fused to the
  next list item's first word ("exit 0, error-string rule").
- e08 L329 "401 heading" FP1: an HTTP status code with the
  noun "heading".
- e12 L12 "60 items" FP1: the EB-60 item id fused across a
  slash ("EB-15/16/60 items"); the guard set does not cover
  the slash, the one fusion character the v2 set missed.

The v1 fused-label mechanisms (C-01, SO-01, HEAD~1,
NO_RUST=1, #480) are absent; the extended guard set did its
work. A slash in the guard set is the obvious v3 candidate.
The class's recall remains low by design (narrow noun list);
the rate is a precision statement only.

## Size

Eight false positives, 12.9%, down from 23.8% but still far
over the bar. The decimal-fragment mechanism stays dead
(zero instances, second corpus running). What remains:

- The byte-fragment fusion, three instances: e04 L441
  "78, byte" (tree SHA 03926c78 before ", byte-identical"),
  e16 L63 "7 byte" ("the other 7 byte-identical", the 7
  counts files), e23 L126 "717, byte" (tree 0cbef717 before
  ", byte-identical"). A hex fragment's tail or a count
  fused to the word "byte-identical" reads as a byte size.
- Thresholds and budgets, five instances: e21 L50 "5 GB"
  (the sweep cap), e24 L57 and L229 "500.0 KB" x2 and
  "120.0 KB" x2 (the board's transfer budgets beside the
  measured transfers, which scored TP).

## gitsha and hash256

Clean again: 49 of 49 and 14 of 14. The hash256 sample is
no longer tiny (e17 carries 11 of the 14, a weight-staging
section of full per-file digests, all genuine).

## Recall (five hand-listed reports, 385 claims)

Hand lists were written before the extractor existed
(v2-recall-handlist.md). Matched means a hand-listed claim
the extractor also produced as a TP.

- pr: 20 of 24 (83%). The four misses are e13's #52 cites,
  which the extractor produced as FPs, not matches.
- commit: about 62% (58 TP extractions against 93 listed).
  Misses concentrate in no-context lines, truncated
  revision prefixes the hand list counted and the class
  refuses by design, and e17's prefix forms.
- gitsha: 13 of 13 (100%).
- hash256: 12 of 12 (100%).
- tally: about 96% on these five (77 of 80). The reports
  are tally-dense gate reports in canonical shapes. Named
  misses: the canary fractions "10/10 cases" (the fraction
  noun list has no "cases"), "passes 69/69" (no noun after
  the fraction), "156 targeted tests" (an adjective between
  the number and the noun).
- count: about 15% (21 of 144), the narrow noun list by
  design, unchanged from v1.
- size: 6 of 19 (32%). Misses are unit-before-number forms
  and sizes stated inside tables the hand list read as
  claims in prose order.

## Method notes

Adjudication was by the FP-list method against the
extractor's mechanical per-class counts, after two full
passes over the output in slices; per-report extraction
counts reconcile exactly to the run totals (1,138). Two
judgment calls are recorded where they were made: the
20 MiB gateway limit in e04 scored TP (a third party's
documented limit stated as fact, unlike the v1 rounds'
own-work thresholds), and the e24 budget caps scored FP1
on the threshold precedent. The corpus's audit stratum is
issue-citation dense in a way the v0 and v1 corpora were
not; that composition fact, not a pattern change, is what
moved the PR class.
