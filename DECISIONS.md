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
