# Extraction measurement, pattern set v2 results

Status: measured 2026-11-23, reported for ratification.
Artifacts banked under docs/m3/: the corpus manifest, the
extractor, the scoring record, the recall hand list.

## What was measured

Pattern set v2 is pattern set v1 with exactly the three
changes fixed in the corpus manifest before the extractor
was written: the of-form gains the fused-token number
guard; the of-form does not fire after a series word; the
guard set extends to hash, hyphen, tilde, and equals for
the tally and count patterns. The corpus is 24 fresh fleet
reports (368,434 bytes, none in the v0 or v1 corpora),
frozen by recorded SHA-256. Hand lists were written before
the extractor existed. Every extracted claim was
hand-adjudicated. No class inherits its v1 rate.

## Results

1,138 claims extracted: 1,093 true, 45 false (4.0%).

- pr (merged): 199 claims, 15.6% false
- commit: 331 claims, 0.6% false
- gitsha: 49 claims, 0.0% false
- hash256: 14 claims, 0.0% false
- tally: 377 claims, 0.3% false
- count: 106 claims, 2.8% false
- size: 62 claims, 12.9% false

## The tally class

The v2 changes were built for this class, and they worked.
The v1 mechanisms are absent from the output: no decimal
fragment of-forms, no document-series or rank of-forms, no
fused-label tallies. The single false positive is a
comma-form fusion ("0, passed" from "exit 0, passed"). At
0.3% the class measures under the 2% bar on a fresh corpus.
Proposed disposition: run tallies return to the trusted
set, on Main's ratification.

## The PR class

The merged class measured 0.6% on the v1 corpus and 15.6%
here, and the mechanism is isolated. All 31 false positives
are bare references to GitHub ISSUES quoted in audit
reviews from disposition documents (the EB items' absorbed
issues #14, #16, #17 and the noise work's #50, #51, #52).
Every explicit form scored clean, and bare references that
are genuinely pull requests scored clean. The class's rate
is corpus-sensitive through its bare half. Proposed
disposition, a v3 design question rather than a v2 fix:
split the class by form, explicit forms trusted, bare forms
provisional. Until then the merged class's trusted standing
rests on the v1 corpus and should be read with this
corpus beside it.

## The rest

Commit improved to 0.6% on a larger sample; its two false
positives are the named standing risk, truncated SHA-256
prefixes of dataset files in a commit-context sentence.
Gitsha and hash256 are clean again, and the hash256 sample
is no longer tiny (14 claims, 11 of them a weight-staging
section of full digests). Count reached 2.8%, inside the
provisional band for the first time, because the extended
guard set killed the fused-label mechanisms; its remaining
false positives are an exit-tally fusion, an HTTP code with
the noun "heading", and one slash fusion the guard set does
not cover. Size improved to 12.9% and stays out: what
remains is the byte-fragment fusion (a hex tail or a count
fused to "byte-identical") and budget caps stated beside
measured transfers. Recall on the five hand-listed reports
is unchanged in shape: full for gitsha and hash256, high
for tally on canonical gate reports, low for count by
design. The full adjudication, with every false positive
named by report and line, is the scoring record in
docs/m3/scores-v2.md.
