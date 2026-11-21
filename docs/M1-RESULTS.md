# M1. Extraction measurement results

Measured 2026-11-21 under the protocol in
docs/M1-EXTRACTION-MEASUREMENT.md. Pattern set v0 (docs/m1/
extract.py) ran over the frozen 24-report corpus (docs/m1/
manifest.txt, 238,077 bytes, stratified 8 gate reports, 8 audit
summaries, 8 closure records). It produced 831 extracted claims.
Every one was adjudicated by hand against its report and evidence
(docs/m1/scores.md). Recall was estimated against a hand list
written before the extractor ran (docs/m1/recall-handlist.md).

One deviation from the plan, recorded in the manifest: corpus
copies could not be materialized because the build box's write
fault zeroed every copy attempt. The corpus is frozen by recorded
SHA-256 of the source files instead, which pins the measured bytes
the same way and makes any later drift detectable. Copies are to
be materialized and verified against the manifest when writes
recover.

## False positive rates, against the proposed bars

The bar from the plan: a class is trusted at a false positive rate
of at most 2 percent; between 2 and 5 percent it ships provisional
and never fails a run alone; above 5 percent it does not ship.

| Class | Extracted | False positives | Rate | Verdict |
|---|---|---|---|---|
| pr (explicit) | 43 | 0 | 0.0% | trusted |
| gitsha (40 hex) | 29 | 0 | 0.0% | trusted |
| hash256 (64 hex) | 4 | 0 | 0.0% | trusted, small sample |
| tally | 117 | 2 | 1.7% | trusted |
| commit (short hex) | 107 | 4 | 3.7% | provisional |
| count | 247 | 17 | 6.9% | does not ship |
| size | 158 | 58 | 36.7% | does not ship |
| pr-bare (#NNN) | 103 | 41 | 39.8% | does not ship as a class |
| quote | 23 | 16 | 69.6% | does not ship |

Overall: 693 true positives, 138 false positives, 16.6 percent.

## What the false positives are

Most false positives are not judgment failures. They are four
mechanical artifacts, each named with examples in the scored
record:

1. Decimal fragments. The size pattern matches the fractional tail
   of a decimal size: "9 GB" from 16.9 GB, "9 MiB" from 223.9 MiB,
   "47 GB" from 2.47 GB. This one artifact is 50 of the 58 size
   false positives. The pattern needs to own the whole number,
   decimals included, or refuse numbers with a decimal point.
2. Same-line duplication. Every pr-bare false positive is an FP4:
   the bare pattern re-fires a number the explicit pattern already
   claimed on the same line. As a standalone class it fails; merged
   into the pr class with same-line, same-number suppression, its
   62 true positives (standalone references nothing else catches,
   47 of them in one PR-state audit) come in at the pr class's rate.
3. Version-prefixed nouns. "2 families" from "v2 families",
   "1 commit" from "v1 commit", "3 finding" from "peira-cycle3
   finding". The digit belongs to a name, not a quantity.
4. Label fragments. "1 findings" from the severity label P1,
   "0 defect" from P0, "5 passed" from the gate name G5, "97560
   test" from a commit SHA followed by the word test.

The quote class fails on design, not artifacts: a quoted span plus
a filename on the same line does not identify a quote from that
file in real prose. The spans are code literals, search strings,
commit messages, and things people said. Only 7 of 23 were genuine
file quotes with the right target. The class needs a different
signal (attribution verbs, blockquote structure) or stays out.

The commit class's 4 false positives in 107 are instructive: three
are truncated hashes of other kinds (an md5 fragment, a cut-down
sha256, a manifest hash) typed as commits, one is a tool run ID.
Provisional is the right standing: extracted commit claims are
printed as provisional and never fail a run alone until a revision
separates object kinds by length and context and re-measures.

## Recall

About 51 percent overall on the five hand-listed reports (93 of
182 claims matched by a true-positive extraction), ranging from 82
percent on a tally-dense gate report to 35 percent on a table and
balance-sheet audit. The misses are systematic: bare numbers with
no adjacent noun (diff stats such as +346/-47, severity mixes such
as 147/74/199, table cells), spelled-out numbers ("eight
requirements"), and fraction forms the patterns do not cover. A
report can therefore carry many true claims the extractor never
sees. Extraction narrows the audit; it does not complete it, and
the marker convention remains the complete path.

## Recommendation

1. The gate may trust four classes from pattern set v0: explicit
   PR references, full git SHAs, full content hashes (small sample,
   noted), and run tallies.
2. Short commit references ship provisional under the bar.
3. Count, size, bare-number, and quote extraction do not ship.
   Pattern set v1 work, in order: own decimal numbers in the size
   pattern; suppress name-internal digits for count; merge the
   bare-number pattern into the pr class with same-line
   suppression; drop the quote pattern pending a structural signal.
   Any v1 class is trusted only after re-measurement on a fresh
   frozen corpus, per the protocol's no-tuning-on-the-test rule.
4. M2 (git claim classes) may start: its inputs are the trusted
   and provisional classes above, and the marker convention, which
   never depended on extraction.

Artifacts: docs/m1/manifest.txt (corpus), docs/m1/extract.py
(pattern set v0), docs/m1/scores.md (every adjudication),
docs/m1/recall-handlist.md (the pre-extraction hand list).
