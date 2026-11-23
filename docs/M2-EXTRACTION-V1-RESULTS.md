# Extraction pattern set v1, measured on the fresh corpus

M2 Track A. The v1 pattern set was run over the fresh frozen
corpus of 24 real unmarked fleet reports (264,736 bytes, frozen
by recorded SHA-256 on 2026-11-21, no report shared with the v0
corpus). The recall hand list was written from five of the
reports before the v1 extractor existed. Every extracted claim
was hand-adjudicated. The scoring record and the run artifacts
are banked under docs/m2/.

## What changed in v1

Five changes, each approved in the M1 results note and ratified
in decision D10. The size pattern owns decimal numbers instead
of matching their fragments. The count pattern refuses a number
fused to a preceding letter, digit, dot, or underscore. The
bare-number pattern merged into the PR class as a candidate,
with same-line suppression when an explicit reference to the
same number is present. The quote pattern was dropped. A tally
guard refuses a tally whose number is fused to a label or
follows a commit-like hex token or a single letter. One
completion is disclosed here rather than buried. The tally
class as designed includes the of-form, which v0 defined but
never wired into the extraction. V1 wires it, and the wiring is
where most of the tally movement comes from.

## Results

726 claims extracted. False positive rates by class, against
the ratified bar (trusted at or under 2 percent, provisional
from 2 to 5 percent, does not ship above 5 percent).

- PR, merged explicit and bare: 172 claims, 1 false positive,
  0.6 percent. The single false positive is a Decisions.md line
  range read as a pull request through the bare shape. The
  explicit subset is 0 of 81 and the bare subset is 1 of 91.
- Full git SHAs: 23 claims, 0 false positives, 0.0 percent.
- Full content hashes: 1 claim, 0 false positives, 0.0 percent.
  The sample is a single claim and says little.
- Short commit references: 174 claims, 3 false positives,
  1.7 percent. All three are truncated SHA-256 prefixes typed
  as commits, the one mechanism shape alone cannot separate.
- Run tallies: 246 claims, 9 false positives, 3.7 percent.
  Five of the nine arrive through the newly wired of-form,
  as document-series positions, a rank position, and decimal
  fragments the of-form never guarded against.
- Counts: 89 claims, 7 false positives, 7.9 percent. The guard
  cut extraction volume by nearly two thirds against v0 and
  the decimal and version mechanisms are gone. What survives
  is fusion through characters the guard does not cover, HTTP
  status codes against the error noun, a range endpoint, and
  a line cite.
- Sizes: 21 claims, 5 false positives, 23.8 percent. The
  decimal-fragment mechanism that dominated v0 is eliminated,
  with zero instances. The survivors are thresholds, a range
  endpoint, and a tree SHA fragment fused to the word byte.

## Recall

Measured against the hand list of 365 claims across five
reports. The PR class matched every hand-listed claim. Full
SHAs matched 7 of 7. Short commits matched 48 of 69, with the
misses on lines carrying no commit context word and on pure
digit SHAs the pattern refuses by design. Tallies matched 52
of 75, missing spelled-out zeros, bare table values, tallies
split across a line wrap, and fraction forms whose noun sits
outside the fraction list. Counts matched 22 of 171, because
the noun list is narrow by design and spelled-out numbers are
out of scope. Sizes matched 2 of 3.

## Dispositions proposed

These take effect only on ratification, as D10 did for v0.

- The merged PR class measured at the trusted rate on the
  fresh corpus and joins the trusted set. The merge is no
  longer a candidate.
- Short commit references move from provisional to trusted at
  1.7 percent, with the hash-prefix mechanism named as the
  standing risk. The rate is corpus sensitive and the class
  should be re-measured if the corpus character changes.
- Run tallies move from trusted to provisional at 3.7 percent.
  Extracted tallies are printed as provisional and never fail
  a run alone until the of-form carries the same number guard
  the count pattern received and the class re-measures.
- Counts and sizes remain out. Both improved in mechanism and
  neither is near the bar.
- The trusted set otherwise stands. Explicit PR references,
  full git SHAs, and full content hashes remain trusted.

## The honest headline

The merge worked and the tally wiring cost more than it
bought. Extraction still narrows the audit rather than
replacing it, and the marker convention remains the complete
path. No class moved on unmeasured numbers, and the two
classes that moved down or stayed out did so on the record.
