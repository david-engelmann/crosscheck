# M4 design record: extraction mode

For Main to set. This record proposes the scope, the one
design decision that needs a ruling, the acceptance evidence,
and what the milestone closes.

## Why this milestone

The measurement program existed to earn extraction its trust.
Amendment 2 made extraction a v1 component with its own
evidence bar, and the bar has now been worked. D15 ratified
the trusted set on two independent measurements: the merged
PR class, full git SHAs, full content hashes, and short
commit references are trusted; run tallies are provisional;
counts and sizes are out. M4 is the milestone that spends
that trust. It gives the CLI an extraction mode, so a report
that carries no evidence markers can still be audited for
the claims extraction has earned the right to check.

## Scope

A new CLI mode that takes an unmarked report and an evidence
bundle and produces the same verdict shape as the marker
mode, under these rules.

- Extraction runs the ratified pattern set, the exact code
  whose false positive rates D15 rests on, ported to the core
  crate without pattern changes. A pattern change after this
  milestone is a new pattern version and re-measures under
  the protocol before the mode trusts it.
- Trusted classes are extracted and checked. Provisional
  classes are extracted, printed as provisional, and never
  fail the run alone, per the D10 rule. Classes that are out
  are not extracted at all.
- The marker mode is untouched. The marker convention remains
  the complete path, and extraction remains the narrowing
  path. The mode prints that limit with its verdict, in the
  same spirit as the semantic limit line.

## The decision that needs a ruling

A marked claim names its evidence. An extracted claim does
not, and the bundle must answer for it. The proposed rule is
that the bundle answers what it can, in the most specific
way available, and the rest is unverifiable.

- A commit or full SHA checks against the bundle's
  repository, exactly as the marked git classes do.
- A PR reference checks against the bundled host records,
  exactly as the marked PR class does. No record bundled
  means unverifiable.
- A content hash checks against every file the bundle
  carries. A match anywhere verifies it. No match anywhere
  leaves it unverifiable rather than contradicted, because
  the extractor cannot know which file the author meant.
- A tally checks against every log the bundle carries, under
  the M3 last-tally rule. A tally found and agreeing in any
  bundled log verifies provisionally. A tally found and
  disagreeing in every log that records that fact is
  contradicted provisionally, and still cannot fail the run
  alone. A tally no bundled log records is unverifiable.

The alternative is to require the report's own sentence to
name the evidence file before extraction checks the claim.
That is stricter and closer to the marker semantics, and it
would leave most extracted claims unverifiable, because
unmarked reports rarely name their logs in a pattern the
extractor can bind. The proposed rule is recommended because
it keeps the failure semantics honest in both directions.
Nothing extracted is ever contradicted on evidence the
report did not point at, except the tally case, and there
the provisional status carries the weakness openly.

## Acceptance evidence

- Core tests for every class and every verdict path in the
  new mode, each able to fail for a real reason, including
  the hash-anywhere rule, the no-record PR path, and the
  provisional tally that cannot move the exit code alone.
- A demonstration run over one report from the v1 frozen
  corpus with a bundle assembled for it, its extracted
  verdicts checked against the hand adjudication banked in
  docs/m2/. The trusted classes must agree with the
  adjudication on every claim the bundle can answer.
- The existing suite green and the hosted gate green on the
  exact head, as always.

## What it closes

M4 closes the extraction component that Amendment 2 required,
and with it the original M1 question. What remains open
after M4 is named here so the record is complete. Bundle
internal references are the one claim class from the design
taxonomy still unbuilt. The tally class returns to trusted
only on the pattern set v2 measurement now in flight. A
network mode remains parked under Amendment 1 and returns
only as its own decision.
