# crosscheck

Crosscheck is a deterministic verifier for agent work reports. An
agent finishes a job and hands over a report full of claims. This
file says so. That count is 397. That pull request merged at this
head. Crosscheck checks those claims against the evidence the
report cites and fails on any claim it cannot verify.

## What it does

You give crosscheck two things. The first is a work report, a
Markdown document written by an agent at the end of a job. The
second is an evidence bundle, a directory holding the files the
report cites, the records it relies on, and the repository behind
it. Crosscheck reads the claims out of the report and recomputes
each one from the bundle. A quoted span must exist word for word
in the file it cites. A stated hash must match the file on disk. A
stated count must come out the same when the artifact is recounted.
A cited commit must resolve in the repository, and a claimed
ancestry or diff scope must hold in its history. Any claim that
cannot be checked this way fails the run unless the report declared
that claim unchecked itself, in which case the declaration is
printed for the reader.

Crosscheck runs no command the report names. It uses no model and
no network. The same report and the same bundle always produce the
same verdict.

One limit is stated plainly. Crosscheck proves that a quoted span
exists and that a number recomputes. It does not prove that a span
supports the sentence it is attached to. That judgment stays with
the reader.

## How it is used

```
crosscheck verify report.md --bundle ./bundle
```

The exit code is the verdict. Zero means every checkable claim
verified. One means at least one claim is contradicted by the
evidence. Two means the report or the bundle is malformed. Three
means at least one claim needs evidence the bundle does not carry.
The printed output names every claim, what kind of claim it is, and
what the evidence said.

Adding --attest to either mode prints the verdict as an in-toto
Statement instead. The statement is unsigned and reproducible byte
for byte from the same report and bundle, ready for the caller to
wrap and sign with their own tooling.

## The verdict viewer

The web directory holds a verdict viewer, a Next.js app that
renders one real work report checked claim by claim against its
evidence bundle, every claim shown with its verdict and the
evidence behind it. It is showcase material for the tool. The tool
itself is the command line above.

## Current state

Every claim kind in the design is built and gated. Through the
marker convention a report can claim existence, size, content
hash, a quoted span, and a recomputed count, and each is checked
against the file it cites. Commit resolution, ancestry, and diff
scope run against the repository a bundle carries. Recorded run
facts are read from the logs a report cites, and pull request and
CI state are read from the host records a bundle carries.

A second mode audits reports that carry no markers at all.
Extraction finds the claims in unmarked prose and checks each one
against whatever the bundle can answer, and a claim the bundle
cannot answer is unverifiable. Extraction was measured before it
was trusted, on three frozen corpora of real fleet reports with
every extracted claim scored by hand against a two percent false
positive bar. Explicit pull request references, short and full
commit references, full content hashes, and run tallies measured
at or under the bar and are trusted. Bare pull request references
and counts sit in the provisional band above it, so they are
extracted and printed as provisional and never fail a run on
their own. Sizes and quotes measured too far above the bar and
are not extracted.

Verdicts export as in-toto Statements under --attest, with the
records behind a verdict pinned by their own digests inside the
statement. The decision record stands in DECISIONS.md and the
measurement record in the docs directory.

## License

Licensed under either of MIT or Apache-2.0 at your option.
