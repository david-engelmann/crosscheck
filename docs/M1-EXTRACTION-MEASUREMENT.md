# M1. Extraction measurement plan

Status: design, ready to start the day the repository wall lifts.
The interim label is used in this document; the rename to the final
name does not change the plan.

## Why this milestone exists

DECISIONS.md D6 rules that pattern extraction from unmarked prose is
a v1 component with its own evidence bar: the gate does not trust
extraction until its false positive rate is measured on real
reports. This document fixes how that measurement is run, so the
result is a number the gate can rely on rather than an impression.
The milestone produces a measured pattern set, not a feature. The
extractor ships in the milestone after this one, carrying only the
pattern classes that pass the bar set here.

## What extraction has to find

Candidate pattern classes, each stated as the prose shape, the claim
it maps to, and the evidence that claim needs. The classes come from
the design record's survey of real fleet reports.

1. Byte sizes. Shape: a number followed by bytes, or a file named
   with a size in the same sentence. Maps to the size claim. Needs
   the cited file in the bundle.
2. Hashes. Shape: 64 hex characters (content hash), 40 hex
   characters (git object name), or a short form of 7 to 12 hex
   characters near a commit or head mention. Maps to the hash and
   commit claims. Needs the file or the repository.
3. Test tallies. Shape: a count with the words test or tests, check
   or checks, and the shapes "N passed", "M failed", "exit 0", "exit
   code N". Maps to recorded run facts. Needs the cited log or
   report-internal consistency.
4. Pull request references. Shape: "PR" followed by a number, or a
   bare hash-number in a repository context. Maps to the pull
   request claim. Needs the bundled host record, per the offline
   ruling in D4.
5. Counts of named things. Shape: a number attached to a countable
   noun the report also enumerates, such as files changed, headings,
   findings, or rounds. Maps to the recomputed count claim. Needs
   the cited artifact or list.
6. Quoted spans. Shape: text in quotation marks attributed to a
   named file or document. Maps to the quoted span claim. Needs the
   cited file.

Classes that cannot be tied to evidence in the bundle are out of
scope for extraction, exactly as they are out of scope for the
marker convention: semantic support, quality, and intent are never
extracted as claims.

## Corpus selection

The corpus is real reports, written for human readers, carrying no
markers, produced by the fleet's own lanes before this tool existed.

Sources, all already on this machine or in the owner's records:

- Gate reports and closure records banked in goal workspaces.
- Lane completion reports and audit summaries in the same places.
- The daily records' report-shaped entries, where the cited
  artifacts still exist on disk.

The corpus from the flagged fleet-records clone is excluded. That
clone is not used for anything, per the standing constraint from
its arrival. Nothing in this plan needs it; the goal workspaces hold
more reports than the sample requires.

Selection rules:

- 24 reports, stratified: 8 gate reports, 8 closure or completion
  records, 8 audit or review summaries.
- Each report's cited artifacts must still be reachable (on disk or
  in a local clone's git history), so every extracted claim can be
  adjudicated against evidence rather than memory.
- Reports are frozen into the corpus directory with their SHA-256
  recorded before any pattern work begins. A report that changes
  after freezing is replaced, not edited.
- Target yield: at least 200 extracted claims across the corpus. If
  the first pattern pass yields fewer, reports are added in the same
  strata until the target is met. Rates computed on fewer than 200
  claims are reported as preliminary.

## Hand-scoring protocol

Extraction is run once over the frozen corpus with the pattern set
frozen and versioned (pattern set v0). The output is a claim list:
report, line, pattern class, extracted claim, cited evidence target.

Every extracted claim is then adjudicated by hand against the report
and its evidence, into exactly one of:

- TP: a real factual claim about an artifact, correctly typed, with
  the correct evidence target.
- FP1: not a claim. The number or string is not a factual assertion
  about a checkable artifact (a date, a version, a target or goal
  figure, a hypothetical).
- FP2: a real claim, wrongly typed (a size extracted as a count).
- FP3: a real claim, correctly typed, but the evidence target is
  wrong or missing, so the claim as extracted cannot be checked.
- FP4: a duplicate. The same claim extracted twice from one passage.

The false positive rate is (FP1 + FP2 + FP3 + FP4) over all
extracted claims, reported overall and per pattern class.

Recall is estimated separately. Five reports, one stratum-weighted
sample, are read in full by hand and every checkable claim in them
is listed before the extractor output for those reports is opened.
Recall is extracted-and-TP over hand-listed, per class and overall.
Recall is reported as an estimate on five reports, never as a
corpus-wide figure.

Scoring discipline:

- The scorer does not tune patterns. Pattern changes after scoring
  begins create pattern set v1, and the full corpus is re-extracted
  and re-scored from zero. No partial re-scores.
- Adjudication disagreements with the design record's claim
  definitions are resolved by the definitions as written, and any
  definition that cannot decide a real case is recorded as a finding
  against the taxonomy, not silently interpreted.
- Every adjudicated claim keeps its reason in one line. The scored
  corpus is the milestone's primary artifact and is banked with the
  results.

## The bar

Proposed, for the owner to ratify when the numbers land:

- A pattern class is trusted for gate use when its false positive
  rate on the frozen corpus is at most 2 percent.
- A class between 2 and 5 percent ships behind a warning: extracted
  claims of that class are printed as provisional and never fail a
  run on their own.
- A class above 5 percent does not ship. Its claims are extracted
  only in the measurement harness until the patterns improve and a
  fresh frozen corpus re-measures them.

The reasoning for a tight bar: a false positive in this tool is a
false accusation against a report, and the tool's whole standing is
that its accusations are recomputed, not guessed. A looser bar
spends that standing.

## Risks named in advance

- Survivorship. Reports whose artifacts survive may be tidier than
  the average. The strata and the reachability rule are stated so
  the bias is visible in the write-up, not hidden by it.
- Reports citing reports. A claim about a second report's contents
  is adjudicated against that report as evidence, one level deep.
  Deeper chains are FP3 by definition here, and the count of them is
  reported.
- Tallies without artifacts. A test tally whose log is gone is a
  real claim with no reachable evidence. It scores TP for extraction
  (the extractor's job ends at finding the claim) and is recorded
  separately as unverifiable-at-audit, so the extraction rate and
  the audit rate are never conflated.

## Deliverables

1. The frozen corpus manifest: 24 or more reports, hashes, strata,
   artifact reachability notes.
2. Pattern set v0, as data in the repository, with the per-class
   definitions matching this document.
3. The scored claim list: every extracted claim, its adjudication,
   its one-line reason.
4. The results note: false positive rate overall and per class,
   recall estimate on the five-report sample, taxonomy findings,
   and the recommendation of which classes the gate may trust under
   the bar above.

Milestone M2 (git claim classes) does not start until the results
note is banked and the trusted classes are named.
