# Design record: extraction pattern set v3 measurement

Status: design record, written 2026-11-23 under Main's
charge of the same date. Ruled as written by Main on
2026-11-23, both questions: the corpus design below is
approved, and the suppression recommendation is adopted as
a standing rule, banked as DECISIONS.md D20. The sequence
at the close of this record is authorized. The measurement
discipline is unchanged from M1: the corpus is frozen
before patterns, hand lists are written before the
extractor runs, every extracted claim is hand-adjudicated,
and no class inherits a rate from an earlier corpus.

## The open question

D17 split the merged pull request class by form. The
explicit form (PR #N, pull/N, owner/repo#N) stayed trusted.
The bare form (#N) went provisional, because the merged
class measured 15.6 percent false positives on the v2
corpus while every explicit reference scored clean. D17
left two questions for a v3 design, and this record answers
both before any corpus exists:

1. What corpus measures the bare form honestly, given that
   its rate is corpus-sensitive?
2. Is refusing a bare reference inside issue-tracker
   context a legitimate extraction rule, or a corpus-fitting
   move?

## What v2 actually measured, split by form

The v2 results note scored the merged class. Split by form
from the banked extractor's own output over the v2 corpus,
the picture is sharper than the merged rate suggested:

- 199 pull request claims: 95 explicit, 104 bare.
- Explicit: 0 false positives in 95 (0.0 percent).
- Bare: 31 false positives in 104 (29.8 percent). Every one
  of the 31 was a bare reference to a GitHub issue, quoted
  inside an audit review of a spec for an issue-tracked
  work programme (reports e12, e13, e14, all in the audit
  stratum).
- Bare extractions by stratum: gate 43, audit 50, closure
  11. The audit stratum's bare false positive rate was 62
  percent (31 of 50). Gate and closure together produced 54
  bare extractions and no false positives at all.

So the bare form is two different classes wearing one
pattern. Where a report cites pull requests, bare references
behave like the explicit form. Where a report quotes an
issue tracker, a bare number is more likely an issue than a
pull request. Any v3 corpus that does not state which of
those worlds it samples, and in what proportion, measures
nothing portable.

## Question one: the corpus

### Strata

The v3 corpus keeps the fleet's report kinds and splits the
one stratum v2 proved heterogeneous. Four strata, six
reports each, 24 reports:

- Gate reports (6).
- Closure reports (6).
- Audit reports whose subject is not an issue-tracked work
  programme (6).
- Audit reports whose subject is an issue-tracked work
  programme (6).

Stratum membership is a bibliographic fact about the report
(what it reviews), decidable before any extraction, the
same way gate, audit, and closure always were. Reports are
drawn from the same source pool as v0 through v2 (the peira
goal's banked reports) and are disjoint from all three
earlier corpora.

### Density stated up front

Each stratum's issue-quoting density is stated in the
corpus manifest before the extractor runs. Density is
measured from the hand lists: among the bare references in
a stratum's hand-listed reports, the share adjudicated as
issue cites rather than pull request cites. Because the
density statement is a load-bearing part of the design,
hand-list coverage rises from the usual five reports to
eight, two per stratum, so every stratum's density rests on
two full reads. The manifest publishes, per stratum, the
report count and the hand-list density, and the corpus is
frozen at that point. If a stratum cannot be filled from
the pool, or a hand list shows a stratum's density is not
what its label claims, that is reported with the manifest,
not repaired quietly after the fact.

### Yield floor and decision rule, stated in advance

- The corpus must yield at least 150 bare extractions in
  total, so the rate's resolution is finer than the bar
  (one false positive in 150 is 0.7 percent). If the frozen
  24 reports yield fewer, reports are added stratum by
  stratum in the manifest's stated order until the floor is
  met, and the additions are recorded in the manifest
  before extraction.
- The bar is the ratified bar, unchanged: the bare form
  returns to trusted only at a false positive rate at or
  under 2 percent, and only on Main's ratification.
- The decision rate is the bare form's rate over the whole
  corpus, over the stratum mix this record states (equal
  quarters). Per-stratum bare rates are published with the
  results regardless of the outcome, so a reader can see
  the conditions the overall rate was earned under.
- The explicit form is re-measured on the same corpus as a
  control. It is expected at or near zero; a material
  explicit rate would indict the corpus or the adjudication
  before it said anything about the bare form.
- Every other class re-measures as well, per protocol. No
  class inherits its v2 rate.

### What this design buys

The v2 number (29.8 percent) and a hypothetical clean
number from a corpus with no issue-quoting audits would
both be true and both be useless alone. This corpus makes
the rate a function with its conditions printed on it: a
stated mix, stated densities, and per-stratum rates. If the
bare form passes at equal quarters, it has passed against a
corpus a quarter of which is the hostile environment, and
the trust is earned. If it fails, the per-stratum rates say
exactly where it fails, and provisional stands as a
measured answer rather than a suspicion.

## Question two: issue-context suppression

The proposed pattern change: refuse a bare reference when
its context marks it as issue-tracker matter. Candidate
shapes range from the mechanical (a line carrying three or
more bare references is a citation list, not a pull request
cite) to the semantic (a bare reference governed by an
issue noun, or inside a passage quoting a spec).

### The case for it as a legitimate rule

The programme has always let patterns encode facts about
how fleet reports refer to things. The commit class fires
only with commit context on the line. The tally guard
refuses a number fused to a gate label. D17 itself split a
class by reference form. A cluster of bare numbers on one
line is a shape fact of the same kind: fleet prose cites
pull requests one or two at a time with the work's own
sentences around them, and cites issues in runs, in
parentheses, in quoted programme text. On this view,
suppression is not judging what a passage means; it is
declining to extract where the reference form carries no
signal, exactly as the extractor already declines spelled
out numbers and bare numbers with no noun.

### The case against: corpus fitting in pattern form

Every suppression feature anyone can now propose was
learned from 31 false positives in three reports from one
programme. The mechanical sub-forms do not survive contact
with the genuine cases: gate reports do cite two pull
requests on a line when two changes land together, so a
cluster rule spends real recall to buy precision on one
programme's quoting habits. The semantic sub-forms go
further and ask the pattern to decide what a passage is
about, which is the judgment this tool exists to avoid;
its whole method is that a claim checks mechanically or it
does not check at all. And the transfer question is
unanswerable from inside this fleet: issue cites in another
ecosystem wear other clothes (tracker keys, ticket words,
URLs), so a suppression tuned here would be a rule about
the EB programme's prose wearing a general rule's clothes.
The v2 tally guard is the honest contrast. Its three
changes each answered a defect in a token's own shape (a
decimal fragment is not a count of anything), visible in
the token itself, and they re-measured clean on a corpus
that had never seen them. Issue context is not a defect in
the token. A bare #52 is a perfectly formed reference; it
just refers to an issue, and no property of the token or
its line says so in general.

### Recommendation

Do not build suppression into pattern set v3. Measure the
bare form as it stands on the density-stated corpus above,
and let the class's standing follow that measurement:

- If the bare form measures at or under the bar on the
  stated corpus, it returns to trusted on Main's
  ratification, and the per-stratum rates stand in the
  record as the conditions of that trust.
- If it measures above the bar, provisional is the final
  answer for this pattern set, stated with its per-stratum
  rates. That is not a failure of the measurement; it is
  the measurement. A reference form whose precision depends
  on what the surrounding prose is about is what the
  provisional standing was designed for: extracted, checked,
  printed as provisional, never failing a run alone.
- Suppression may be proposed again only with evidence from
  reports this programme has not already scored, meaning a
  tuning set and a measurement corpus drawn from a different
  report ecosystem than the one that produced the v2 false
  positives. Inside this fleet, any suppression proposed now
  would be fitted to the evidence that would then be used to
  judge it, and the protocol exists to prevent exactly that.

The cost of this recommendation is small and worth stating.
A provisional bare reference is still extracted, still
checked against the bundled record, and still printed with
its verdict. What it cannot do is move the exit code. The
product loses nothing it can honestly claim, and keeps the
line that makes its trusted claims worth trusting.

## What waits on the ruling

On Main's ruling, in order: freeze the corpus and publish
the manifest with per-stratum densities from the eight hand
lists; port the v2 extractor to v3 unchanged (no pattern
changes under this recommendation); score from zero;
adjudicate every claim; bank the results note with
per-stratum and per-form rates; dispositions follow only on
ratification.
