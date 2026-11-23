# Extraction pattern set v3: measurement results

Measured 2026-11-23 on the corpus frozen in
docs/v3/v3-manifest.txt, to the design in
docs/EXTRACTION-V3-DESIGN.md as ruled by Main (DECISIONS.md
D20). The pattern set is v2's, unchanged; no suppression
was built. Dispositions proposed at the end take effect
only on Main's ratification.

## The corpus, and two disclosures owed in the note itself

Thirty-five reports, 364,726 bytes, from the peira goal's
banked reports, disjoint from the v0, v1, and v2 corpora.
Four strata: gate (8 reports after additions), audit-other
(8), audit-issue (9), closure (10). The audit split is by
the report's subject, a bibliographic fact decided before
extraction. Each stratum's issue-quoting density was
published in the manifest from eight hand lists before the
extractor ran: gate 0.00, audit-other 0.00, audit-issue
1.00 (a small base, n=3, published as it stood), closure
0.00.

First disclosure: the design's yield floor was not met.
The design set a floor of 150 bare extractions so the
rate's resolution would be finer than the 2 percent bar.
The frozen 24 reports yielded 53; the manifest's addition
rule was executed as written, drawing every reserve
round-robin until the lists were exhausted, and the final
corpus yields 69. The floor is unreachable from this
source pool, whose remaining files are design documents
and plans rather than work reports. Main ruled option A
(adjudicate as frozen, with the resolution disclosed)
over widening the pool mid-measurement. The decision rate
below therefore carries a resolution of 1 in 69, or 1.4
percent: at or under the bar means at most one false
positive in 69.

Second disclosure: two boundary events stand in the
manifest as recorded. The pool's spec reviews of PR #295
and PR #297 were excluded entirely, as reviews whose
subjects are pull requests serving the issue-tracked
programme. And the original f13 report was replaced
before extraction when hand-listing showed its subject
was PR #295 itself; the replacement came from the reserve
list, with the first reserve passed over for the same
stated reason. No extraction output existed when either
decision was made.

## Results

694 claims extracted and hand-adjudicated; 22 false
positives (3.2 percent overall). Totals reconciled
mechanically against the extractor's output.

| class | claims | FP | rate |
|---|---|---|---|
| pr explicit | 36 | 0 | 0.0% |
| pr bare | 69 | 18 | 26.1% |
| commit | 149 | 0 | 0.0% |
| gitsha | 36 | 0 | 0.0% |
| hash256 | 4 | 0 | 0.0% |
| tally | 168 | 0 | 0.0% |
| count | 153 | 4 | 2.6% |
| size | 79 | 0 | 0.0% |

The question the measurement exists to answer, the bare
form by stratum:

| stratum | bare claims | FP | rate |
|---|---|---|---|
| gate | 24 | 0 | 0.0% |
| audit-other | 20 | 0 | 0.0% |
| audit-issue | 18 | 18 | 100.0% |
| closure | 7 | 0 | 0.0% |

The split is total. Outside the issue-quoting stratum,
every bare extraction was a genuine pull request cite, 51
of 51. Inside it, every bare extraction was something
else, 18 of 18: eleven GitHub issue cites quoted in EB
programme reviews (the #50/#51/#52 family, the same
referents the v2 adjudication counted), and seven
internal document numbers (Taxonomy ruling numbers cited
as #10, #11, #12, a mechanism the v2 corpus did not
produce). The explicit form, re-measured as the design's
control, scored 0 of 36, so the corpus and the
adjudication are not what produced the bare result.

The four count false positives are all fragments: a date
fragment ("15 instance" from "Oct 15 instance"), two
pixel widths in UI build reports whose following noun
belonged to the next list item, and one comma-group
fragment ("289 lines" from 5,289). No new mechanism; the
slash-fusion candidate from v2 did not recur.

Size scored 0 of 79, and the note declines to celebrate
it: this corpus's size claims are artifact and machine
specifications in build and execution reports, a
friendlier population than the prose sizes that measured
36.7, 23.8, and 12.9 percent on the three earlier
corpora. Like the bare form, the size rate is a property
of the population sampled; unlike the bare form, size was
not given a density-stated design, so its number here is
an observation, not a measurement it can be promoted on.

Recall against the eight hand lists (448 claims) is
roughly 52 percent, the v0 level, on a corpus heavier in
prose and tables. The misses concentrate where they
always have: spelled-out numbers, table cells, repeated
instances on shared lines, and size forms written
without a space before the unit.

## Proposed dispositions (for Main's ratification)

- pr bare stays provisional, and the D17 question is
  closed with a measured answer: the form's precision is
  a property of the report's stratum (0.0 percent across
  51 claims outside the issue-quoting stratum, 100
  percent across 18 inside it), and a pattern cannot see
  the stratum. Provisional is the honest standing for a
  form whose trust depends on what the surrounding prose
  is about. This is the outcome the design record named
  in advance as a legitimate final answer.
- pr explicit stays trusted (0.0 percent, the control).
- commit, gitsha, hash256, and tally standings are
  reconfirmed on a fresh corpus (0.0 percent each; tally
  0 of 168 following v2's 0.3 percent).
- count stays provisional at 2.6 percent, its third
  consecutive measurement inside the provisional band
  (7.9, 2.8, 2.6).
- size stays out. Its 0.0 percent here is recorded with
  its population explanation; if the class is ever to be
  re-measured for promotion, it needs a density-stated
  design of its own, on the v3 pattern.

## Artifacts

docs/v3/ holds the frozen manifest with densities and
the addition record, the eight-report hand list, the
extractor (the v2 pattern set unchanged), the extraction
output, and the score sheet.
