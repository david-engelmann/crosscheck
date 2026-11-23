# Design note: verdict export in attestation shape

Status: design note only, written 2026-11-23 under the
applicability audit charge of the same date. No build is
charged by this note. Main reviews it before any build.
Reviewed and accepted by Main on 2026-11-23, with the open
choice ruled as proposed in the section below (no wall-clock
timestamp in the predicate; signing stays the caller's
step). A build charge waits until the M4 checkpoint closes
and the v2 measurement lands, so the export is built once
over the settled verdict shape.

## The idea

A crosscheck verdict today lives in the terminal and in the
tool's own JSON output. An optional export renders the same
verdict as an in-toto Statement, so it can be wrapped in a
DSSE envelope and travel inside the attestation format the
rest of the ecosystem already verifies, through SLSA tooling
and Sigstore. The export is a rendering of the verdict, not
a new verdict path. Nothing about how a verdict is produced
changes, and the export is off unless asked for.

## The statement

The exported statement has the standard in-toto v1 shape.
The subject is the report, named as the caller names it and
digested with the same SHA-256 the bundle manifest already
records for it. The predicate type is ours, versioned, and
hosted where we control it:

`https://github.com/david-engelmann/crosscheck/attestation/verdict/v1`

The predicate body carries the verdict:

- `tool` — the crosscheck name and version that produced it.
- `mode` — `verify` or `extract`.
- `exitCode` — the D5 exit code, carried as data.
- `summary` — the claim counts by status. In extract mode the
  counts split by trust, trusted and provisional.
- `claims` — one entry per claim, in line order, with the
  line, the marker spec in verify mode or the class and trust
  in extract mode, the status, and the detail text the CLI
  printed.
- `bundle` — the bundle manifest digest and, when the bundle
  carries a repository, the git head the git claims resolved
  against. This pins the evidence set the verdict rested on
  without making the bundle itself a subject.
- `limit` — the semantic limit line, carried verbatim, so the
  attestation cannot be read as claiming more than the tool
  claims.

## The D14 record mapping

Claims checked against bundled host records cite the record
in an `evidence` entry on the claim, and the flat D14 fields
carry over unchanged:

- A pull request claim maps its record as the record path
  (`records/pr-N.json`), the record file's own SHA-256, and
  the record's `number`, `state`, and `merged` fields.
- A CI claim maps its record as the record path, the record
  file's SHA-256, and the record's `head_sha`, `status`, and
  `conclusion` fields.

Digests of the record files are included so the attestation
pins the exact record bytes the verdict rested on, not just
the values read from them.

## The envelope and signing

Wrapping is the standard DSSE shape, with payload type
`application/vnd.in-toto+json` and the statement carried as
the base64 payload. Crosscheck emits the unsigned statement.
Signing is the caller's step, with the caller's keys or
Sigstore identity, using standard tooling. Key management
belongs to the auditor, not to this tool, and a signature
crosscheck cannot make is not one it should fake. A consumer
verifies the envelope and the predicate type with the usual
tooling, then reads the verdict from the predicate. The exit
code's meaning stays crosscheck's own; verifiers of the
envelope learn that the verdict was made and by whom, and
the predicate tells them what it was.

## One choice, stated for review

The predicate carries no wall-clock timestamp. A crosscheck
verdict is a pure function of the report, the bundle, and
the tool version, and the export keeps that property: the
same verdict exports byte-identically every time. A runner
that needs a time can stamp the envelope layer, outside the
predicate. This is the one place the shape departs from
provenance practice, and it is deliberate.

## What stays unchanged

The marker convention is untouched. The D14 record shapes
are untouched; the export reads them, it does not revise
them. The exit-code contract is untouched, and the plain
text and JSON outputs are untouched. The export adds one
rendering beside them, and a verdict that cannot be produced
today cannot be exported either.

## Kinship

Perishables holds the matching charge for its re-execution
verdicts. The shared part is the envelope and the discipline
of a versioned predicate type of our own. The predicates
differ, because the verdicts differ, and neither tool's
export waits on the other's.
