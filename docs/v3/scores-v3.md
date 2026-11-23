V3 pattern set scores, adjudicated 2026-11-23
Corpus: v3-manifest.txt (35 reports, 364726 bytes, four
strata with the audit stratum split by subject; densities
published in the manifest before extraction). Extractor:
extract_v3.py, the v2 pattern set unchanged (D20).
Extraction: 694 claims. Every claim adjudicated by hand
against its source line; totals reconciled mechanically
against the extractor output (694 = 672 TP + 22 FP).

Per class (n, FP, rate):
- pr explicit: 36 claims, 0 FP, 0.0 percent (the control;
  clean as designed)
- pr bare: 69 claims, 18 FP, 26.1 percent
- commit: 149 claims, 0 FP, 0.0 percent
- gitsha: 36 claims, 0 FP, 0.0 percent
- hash256: 4 claims, 0 FP, 0.0 percent
- tally: 168 claims, 0 FP, 0.0 percent
- count: 153 claims, 4 FP, 2.6 percent
- size: 79 claims, 0 FP, 0.0 percent
Overall: 694 claims, 22 FP, 3.2 percent.

The bare form by stratum (the measurement the corpus was
built for):
- gate: 24 bare, 0 FP, 0.0 percent
- audit-other: 20 bare, 0 FP, 0.0 percent
- audit-issue: 18 bare, 18 FP, 100.0 percent
- closure: 7 bare, 0 FP, 0.0 percent
Outside the issue-quoting stratum, every bare extraction
was a genuine pull request cite (51 of 51). Inside it,
every bare extraction was something else (18 of 18).

The 18 bare FPs, by mechanism:
- GitHub issue cites quoted in EB programme reviews: 11
  (f16 #52 x4, f17 #50/#51/#52 + #52, f33 #52 x3; the same
  referents the v2 adjudication counted as issue cites)
- Internal document numbering: 7 (f13 Taxonomy ruling
  numbers #10/#11/#12 x5, f15 #10/#11 x2; a mechanism the
  v2 corpus did not produce)

The 4 count FPs, all fragments: a date fragment ("15
instance" from "Oct 15 instance", f19), two pixel-width
fragments in UI build reports (f20 "427, findings", f32
"390, document"), one comma-group fragment ("289 lines"
from 5,289, f22).

Notes on other classes:
- size at 0.0 percent is a corpus fact, not a pardon: this
  corpus's size claims are artifact and machine specs in
  build and execution reports, a friendlier population
  than the prose sizes that measured 36.7, 23.8, and 12.9
  percent on v0, v1, and v2. The class's standing does not
  move on one friendly corpus.
- tally at 0 of 168 follows v2's 0.3 percent; the of-form
  guards held on a fresh corpus.
- f03 (google-wave-canary) produced zero extractions; its
  claims are spelled out or rates.

Recall (hand lists, eight reports, 448 claims after one
hand-list correction in f01): the extractor's output
matches about 232 hand-listed claims, roughly 52 percent,
the v0 level, on a corpus heavier in prose and tables.
Misses concentrate where they always have: spelled-out
numbers, table cells, repeated instances on shared lines,
and size forms without a space before the unit (412K,
211M, 1428 B).
