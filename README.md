# retrial

Retrial is a deterministic verifier for agent work reports. An agent
finishes a job and hands over a report full of claims. This file says
so. That count is 397. That pull request merged at this head. Retrial
checks those claims against the evidence the report cites and fails
on any claim it cannot verify.

## What it does

You give retrial two things. The first is a work report, a Markdown
document written by an agent at the end of a job. The second is an
evidence bundle, a directory holding the files the report cites and
the records it relies on. Retrial reads the claims out of the report
and recomputes each one from the bundle and the git history behind
it. A quoted span must exist word for word in the file it cites. A
stated hash must match the file on disk. A stated count must come out
the same when the artifact is recounted. A cited commit must resolve
in the repository. Any claim that cannot be checked this way is named
as unchecked, and an unchecked claim fails the run unless the report
declared it unchecked itself.

Retrial runs no command the report names. It uses no model and no
network. The same report and the same bundle always produce the same
verdict.

One limit is stated plainly. Retrial proves that a quoted span exists
and that a number recomputes. It does not prove that a span supports
the sentence it is attached to. That judgment stays with the reader.

## How it is used

```
retrial verify report.md --bundle ./bundle
```

The exit code is the verdict. Zero means every checkable claim
verified. One means at least one claim is contradicted by the
evidence. Two means the report or the bundle is malformed. Three
means at least one claim needs evidence the bundle does not carry.
The printed output names every claim, what kind of claim it is, and
what the evidence said.

## Current state

Retrial is at the start of its build. The design is settled and
banked in DECISIONS.md. The core crate checks the five mechanical
claim kinds that were proven out in the design phase, which are
existence, size, content hash, quoted span, and recomputed count.
Claims about commits, diffs, pull requests, and recorded test runs
are designed and land in later slices. The first build milestone is a
measurement, not a feature. Reports in the wild carry their claims in
plain prose without markers, and retrial will extract those claims by
pattern. The extraction is not trusted until its false positive rate
has been measured on real reports, and that measurement gates the
work that follows it.

## License

Licensed under either of MIT or Apache-2.0 at your option.
