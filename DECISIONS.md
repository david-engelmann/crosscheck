# Decisions

This record banks the decisions retrial was built under. It is the
in-repo form of the Phase 1 design record, confirmed at the design
gate on 2026-11-20. Later decisions are appended with their dates.

## D1. The product

Retrial re-tries a finished agent work report against its evidence.
The unit of audit is the whole long-form prose report exactly as the
agent wrote it, checked against an evidence bundle the auditor
assembles from the artifacts the report cites, plus the git history
behind them. It fails closed. Any claim it cannot verify fails the
run, unless the report declared that claim unchecked, in which case
the declaration is printed for the reader.

## D2. Ground ceded, on the record

Single-claim adjudication against pinned evidence bytes belongs to
probity-verify. It is the incumbent in that shape, with typed
adapters and a policy model, and retrial does not compete with it.
Retrial's ground is the report as a whole, a shape none of the tools
surveyed at the design gate takes as input. The surveyed field and
the evidence for this boundary live in the design record and are
summarised in docs/DESIGN-SUMMARY.md when that file lands.

Standing watch item. If probity-verify ships whole-report prose
auditing with fail-closed defaults, retrial's reason to exist is
re-argued that week.

## D3. The limit stated on the face of the product

Retrial proves that a quoted span exists verbatim, that a hash
matches, that a count recomputes, and that a cited commit resolves.
It does not prove that a span semantically supports the sentence it
is attached to. This limit is printed in the README and in the
verdict output. It is a boundary of the method, not a defect to be
patched with a model. There is no model anywhere in retrial.

## D4. Claims taxonomy, v1

Mechanically checkable: quoted span, content hash, size, recomputed
count, existence, commit resolution, diff scope, recorded run facts
parsed from cited logs, and bundle-internal references.

Checkable only from bundled records: pull request state and hosted
CI state. Ruled at the gate on 2026-11-20: these verify offline only
from records the bundle carries. Unbundled means unverifiable and
fails the run. There is no network mode in v1. A network mode, if
ever proposed, returns to the owner as its own decision with the
determinism cost stated.

Not mechanically checkable, and named as unchecked rather than
passed: semantic support, quality, intent, unrecorded events, and
the truth of evidence that was fabricated before it was bundled.

## D5. Failure semantics

Exit 0 verified. Exit 1 contradicted. Exit 2 malformed. Exit 3
unverifiable. Usage and IO errors exit 64 and 66 so a gate never
mistakes a crash for a verdict. Precedence when several classes are
present is 1, then 3, then 2. A gate that checks only zero against
nonzero is always safe.

## D6. Extraction is a v1 component with an evidence bar

Reports carry claims in two forms. New reports can carry explicit
evidence markers. Existing reports carry claims in plain prose
shapes, such as byte counts, blob hashes, pull request numbers, and
tallies of checks and failures. Both forms ship. The gate does not
trust pattern extraction until its false positive rate is measured
on real reports. That measurement is the first build milestone,
ahead of the git claim classes.

## D7. Stack

Rust, edition 2024, toolchain pinned in rust-toolchain.toml. The
core crate retrial-core is standard library only and holds the claim
model, bundle reading, recomputation, and verdict logic. The crate
retrial-cli holds argument parsing, output, and adapters. External
crates live in the CLI and adapter layer and in dev dependencies.
Cargo.lock is committed. Licensed MIT OR Apache-2.0.

## D8. Access

This repository is not part of the GitHub App or PAT installation.
It carries its own per-repo SSH deploy key, and git transport runs
over SSH with that key. The key is generated on the build machine at
creation and registered through the API with the stored credential,
following the fleet pattern recorded in the deploy key inventory.

## D9. Build order

M0, this scaffold with the mechanical core. The five claim kinds
proven in the design phase run end to end through the CLI.

M1, the extraction measurement. Pattern extraction over real reports
with a measured false positive rate, reported before it is trusted.

M2, the git claim classes. Commit resolution, ancestry, diff scope.

M3, recorded run facts and bundled host records. Test tallies from
cited logs, pull request and CI state from bundled records.

Each milestone closes under the house checkpoint rule, which is
three reviews before landing.

## D10. M1 ratification (2026-11-21)

The extraction measurement (docs/M1-RESULTS.md) was ratified by the
owner on the measured evidence. The bar stands as proposed: a
pattern class is trusted at a false positive rate of at most 2
percent, provisional between 2 and 5 percent, and does not ship
above 5 percent. The trusted set is as measured: explicit PR
references, full git SHAs, full content hashes (small sample, noted
on the record), and run tallies. Short commit references are
provisional and never fail a run alone. Counts, sizes, bare-number
references, and quotes do not ship in v1.

Two precisions ride with the ratification. The bare-number merge
into the PR class is a pattern set v1 candidate only: it inherits
nothing until the merged class re-measures at the trusted rate on a
fresh frozen corpus. The quote class stays out until a structural
signal exists, not merely a better pattern. Every v1 pattern change
is re-measured on a fresh frozen corpus before any class moves, per
the measurement protocol's no-tuning rule. The headline stands:
extraction narrows the audit, and the marker convention remains the
complete path.

## D11. M2 git claim classes (2026-11-21)

M2 adds three claim kinds over the repository a bundle carries:
commit resolution, ancestry, and diff scope. The repository lives
at a fixed place in the bundle layout, the repo directory the
auditor assembles. A report names commit identifiers only, never a
repository path, so the evidence a git claim is checked against is
always the auditor's, never the report author's choice.

The core stays free of process spawning (INV-1). It defines the
claim model and verdict logic over a RepoFacts interface: whether
an identifier resolves to a commit, whether one commit is an
ancestor of another, and which files a diff touches. The CLI layer
supplies the facts through the auditor's git, invoked with argument
vectors only, against the bundled repository, with identifiers
validated as hex in the core before they are ever passed on. Facts
are data in, verdicts are data out.

Verdict semantics follow the M0 precedent: evidence absent is
unverifiable, evidence present and disagreeing is contradicted. A
bundle with no repository makes every git claim unverifiable. An
identifier that does not resolve in a bundled repository is
unverifiable. Resolved commits that are not in the claimed ancestry
relation, and diffs whose file set or count disagrees with the
claim, are contradicted.

Extraction does not widen for M2. Git claims enter through the
marker convention, and through extraction only in the ratified
trusted and provisional shapes: full SHAs trusted, short references
provisional.

## D12. The post-scope pass landed (2026-11-22)

The repository carries its final name. David's dedicated token
for the six new flagship repos (connector custom.github-flagships)
replaced the exhausted fleet token for this repo. On 2026-11-22
the seat renamed the repository from retrial to crosscheck
through the API, registered the per-repo deploy key (key id
170768117, read and write) under the title deploy-crosscheck,
fingerprint SHA256:wTE8XICtda+XWT8p2UiFsGG9VvPWe3XK5EI+6BXcNp8,
and pushed the banked local history over SSH with that key. The
first hosted gate ran on the pushed head e42e312d: run 45062633931,
conclusion success, covering the workspace test suite and the
verdict viewer build. The local workspace directory is now
~/workspace/crosscheck.

Still owed under the rename: the crate and binary names
(retrial-core, retrial-cli, the retrial binary) and the product
name inside the documentation. That sweep is its own change,
gated by the hosted suite like any other, and lands after the
first green verdict on the code as banked.

## D13. The rename sweep (2026-11-22)

The sweep named in D12 is done. The crates are crosscheck-core
and crosscheck-cli, the binary is crosscheck, and the README and
the CLI's own text carry the product's final name. The dated
records keep the name they were written under: the M1 corpus and
results, the M2 plan, the viewer's recorded run, and decisions
D1 to D12 are history and were not rewritten. From this decision
forward, crosscheck is the only name in current-state text.

## D14. M3 shapes (2026-11-22)

Recorded run facts. A run-fact claim names one fact and the log
that records it: run passed in PATH == N, and the same form for
failed, skipped, and exit. The cited file resolves inside the
bundle like any cited file. The core scans the log with the tally
shapes the M1 measurement trusted (a number against passed,
failed, skipped, or an exit code) and takes the last match in the
file as the run's record, because logs append and the final
summary is the authoritative one. The boundary is stated here so
the implementation has no freedom left in it: when the claimed
fact's tally is present and disagrees, the claim is contradicted;
when the cited log does not record that fact at all, the claim is
unverifiable, whatever else the log records.

Bundled host records. The bundle carries a records directory of
saved host responses, one JSON object per file, flat fields only.
A pull request claim names the record by number: pr N state ==
merged, or == open. The record file records/pr-N.json must carry
number, state, and merged fields; merged holds when state is
closed and merged is true, open holds when state is open. A CI
claim names the head it describes: ci SHA state == success, or
== failure. The record file records/ci-SHA.json must carry
head_sha, status, and conclusion; the claim compares against the
conclusion once status is completed. A missing record file is
unverifiable, per D4: unbundled means unverifiable. A record whose
fields disagree with the claim is contradicted. A record that
does not parse, or lacks the fields its claim needs, is
malformed evidence and the claim is unverifiable; the distinction
matters because the report author does not control the record's
shape, the auditor does.

The core parses these records with a small flat-object reader.
Nested host payloads are trimmed to the flat record when the
bundle is assembled, the same trimming the verdict viewer's
bundled records already use.

## D15. Track A measured, dispositions proposed (2026-11-23)

The pattern set v1 was measured on the fresh frozen corpus
under the M1 protocol, and the full record stands in
docs/M2-EXTRACTION-V1-RESULTS.md with its artifacts under
docs/m2/. The merged PR class measured 0.6 percent false
positives and is proposed to join the trusted set. Short
commit references measured 1.7 percent and are proposed to
move from provisional to trusted, with the truncated
hash-prefix mechanism named as the standing risk. Run tallies
measured 3.7 percent and are proposed to move from trusted to
provisional, the movement driven by the newly wired of-form.
Counts at 7.9 percent and sizes at 23.8 percent stay out.
These dispositions are proposed, not ratified. They take
effect on Main's ratification, as D10 did for the v0 set.

Ratified by Main on 2026-11-23, all four dispositions as
written. The merged PR class is trusted. Short commit
references are trusted, with the truncated hash-prefix risk
standing in this record. Run tallies are provisional until
the of-form carries the number guard and re-measures at or
under the bar. Counts and sizes remain out. The tally guard
work proceeds as pattern set v2, scored from zero on a fresh
frozen corpus under the same protocol.
