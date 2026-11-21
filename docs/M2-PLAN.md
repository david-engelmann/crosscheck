# M2 plan. Git claim classes, with the v1 re-measurement scheduled

Written 2026-11-21, the day the M1 ratification landed (DECISIONS
D10, D11). This plan schedules two advances together, as the
ratification requires: the pattern set v1 re-measurement and the
M2 git claim classes. Neither widens extraction beyond the ratified
trusted and provisional sets.

## Track A. Pattern set v1 and its re-measurement

Changes, exactly the approved work list from docs/M1-RESULTS.md:

1. Size pattern owns the whole number, decimals included, or
   refuses a number with a decimal point. Kills the decimal
   fragment artifact, 50 of the 58 v0 size false positives.
2. Count pattern suppresses digits inside names: a digit directly
   preceded by a letter or a version prefix is part of a name, not
   a quantity. Kills the version-prefixed noun and label fragment
   artifacts in the count class.
3. The bare-number pattern merges into the PR class with same-line,
   same-number suppression. A candidate only: it inherits nothing
   until the merged class re-measures at the trusted rate.
4. The quote pattern is dropped. The class stays out until a
   structural signal exists (attribution verbs, blockquote
   structure), not merely a better pattern.
5. The tally class's two v0 false positives get one targeted guard:
   a tally word immediately preceded by a commit-like hex token or
   a single-letter gate label is not a tally. This is a v1 change
   like any other and is re-measured, not assumed.

Re-measurement protocol, unchanged from M1: a fresh frozen corpus
of 24 real unmarked reports, none of them in the v0 corpus,
stratified the same way, frozen by recorded SHA-256 before the
extractor runs; the recall hand list written before extraction on
five of them; every extracted claim hand-adjudicated; per-class
false positive rates reported against the same bar. No pattern
changes mid-score. A class moves only on the fresh numbers.

Deliverables: docs/M2-EXTRACTION-V1-RESULTS.md with the fresh
manifest, the v1 pattern set, the scored record, and the recall
estimate, banked in the repo like the M1 artifacts.

## Track B. Git claim classes

Marker forms (DECISIONS D11):

- `[evidence: commit SHA]` resolves the identifier to a commit in
  the bundled repository. Full 40-hex and short 7 to 39 hex forms.
- `[evidence: ancestor A of B]` both resolve, and A is an ancestor
  of B.
- `[evidence: diff A..B files == N]` the diff between the two
  commits touches exactly N files.
- `[evidence: diff A..B touches PATH]` PATH is one of the files the
  diff touches. PATH is matched against git output only, never
  opened, and must be a relative path with no parent components.

Architecture: the core defines a RepoFacts interface (commit
resolution, ancestry, diff file list) and the verdict logic over
it, and stays free of process spawning. The CLI supplies the facts
through the auditor's git against the repository at the bundle's
repo directory, argument vectors only. Core tests run against a
stub RepoFacts with a synthetic commit graph, so the verdict logic
is testable without a repository. The CLI adapter is exercised end
to end once the local test suite can run.

Verdict semantics: no repository in the bundle, or an identifier
that does not resolve, is unverifiable. A resolved ancestry claim
that is false, a diff count that disagrees, or a touched path
absent from the diff, is contradicted. A malformed identifier or
path is malformed.

Out of scope for M2: recorded run facts and bundled host records
(M3), any network mode (parked by D4), packfile parsing of our own
(the auditor's git reads its own object store).

## Order

Track B builds first because its inputs are already ratified.
Track A runs in parallel as measurement work: freeze the fresh
corpus, write the hand list, then run v1 when the patterns are
written. M3 does not start until both tracks report.
