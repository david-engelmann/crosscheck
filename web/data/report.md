# Crosscheck milestone report — M1 measured, M2 landed locally

Report of the verifier seat, 2026-11-22. Subject: the deterministic
verifier for agent work reports (interim repository name retrial,
final name crosscheck). Every checkable claim below is written in a
shape the M1 measurement rated trusted, plus the provisional short
commit class, which is marked provisional wherever it appears.

## History the bundle can check

The repository opened with the scaffold commit
694d3a9b93c5b87b069d598681a4689cb7f7659d. The M1 results were
banked at f226e98428256fde7f6df9e660f411f5d776931c, and the M2 git
claim classes landed at db218197dc5ad39a03fab03fa45c4a8d061f5fef.
The ratification commit e462212e0e6a7bfaac1a9938a3ccab32dbea04d0
sits between the M1 results and the M2 code in the history.
In short commit form (provisional class): 694d3a9, 59494ca, f226e98, e462212, db21819.

## The measurement record

The M1 results note is a fixed record: docs/M1-RESULTS.md carries SHA-256 9331d5de32e1a2014c891e440e977d3a19bd61138f6b827543d1bac4cc841d3d.
The run behind it extracted 831 claims from the frozen corpus and
scored every one by hand.

## Tallies

At the M2 landing the workspace suite carried 15 tests. The Phase 1
cheapest-failure prototype, re-run in-process against its honest
report and the dossier bundle for this record, came out 5 passed,
0 failed, exit 0.

## Host records in the bundle

For the neighbouring surface, peira PR #568 merged on 2026-11-21,
and the bundled host record shows it closed with merged set true.
This repository has no pull requests of its own yet. PR #1 has
never been opened here, and the bundled pulls record stands empty.

## Outside the bundle

Fleet records put the peira main head at d599cdc1 after its later
landings. That commit sits in a repository this bundle does not
carry, so the claim cannot be checked from the evidence here and
must fail closed.

## Planted error, for demonstration

The workspace suite carries 16 tests. This sentence is wrong on
purpose, so the run shows a contradicted verdict against the same
recomputation that verified the tally above.
