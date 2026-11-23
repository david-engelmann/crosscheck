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

## The verdict viewer

The web directory holds a verdict viewer, a Next.js app that
renders one real work report checked claim by claim against its
evidence bundle, every claim shown with its verdict and the
evidence behind it. It is showcase material for the tool. The tool
itself is the command line above.

## Current state

The mechanical claim kinds are built and gated. Existence, size,
content hash, quoted span, and recomputed count run through the
marker convention. Commit resolution, ancestry, and diff scope run
against the repository a bundle carries. Pattern extraction over
unmarked prose reports has been measured on real fleet reports
rather than trusted on faith. Explicit pull request references,
full commit hashes, full content hashes, and run tallies measured
at or under the two percent false positive bar and are trusted.
Short commit references are provisional. Counts, sizes, bare
numbers, and quotes measured above the bar and do not ship.
Recorded run facts and hosted records are the next milestone, and
the full measurement record stands in docs/M1-RESULTS.md.

## License

Licensed under either of MIT or Apache-2.0 at your option.
