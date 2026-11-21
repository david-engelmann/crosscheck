# Invariants

Each invariant traces to the code and tests that hold it. The list
grows with the build, one entry per landed behaviour.

- INV-1. Nothing a report names is ever executed. The verifier reads
  bytes and git facts only. Held by the core having no process
  spawning surface at all; the one process surface in the product is
  the CLI's git adapter, which invokes the auditor's git with a
  fixed subcommand set and argument vectors, never a shell, and
  report-derived strings reach it only as validated data arguments.
- INV-2. No network in the verification path. The core performs no
  IO beyond reading the bundle and the repository it is pointed at.
- INV-3. Fail closed. A claim that cannot be verified produces a
  nonzero verdict, never a pass, unless the report declared the
  claim unchecked and the declaration is printed in the output.
- INV-4. Determinism. The same report bytes and the same bundle
  bytes produce the same verdict and the same per-claim output, in
  report order, with no clock in the verdict path.
- INV-5. Bundle containment. Paths cited by a report resolve inside
  the bundle only. Absolute paths and parent escapes are malformed.
- INV-6. Verbatim means verbatim. A quoted span matches byte for
  byte. Any normalisation is a declared bundle option, never silent.
- INV-7. The semantic limit is printed, not buried. Every verdict
  output restates that span existence does not prove support.
- INV-8. Git claims are checked against the auditor's repository,
  never the report's. The repository sits at a fixed place in the
  bundle layout; a report names commit identifiers only, never a
  repository path. Identifiers are validated as hex of bounded
  length in the core before any adapter sees them.
