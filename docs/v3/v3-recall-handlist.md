V3 recall hand list, written 2026-11-23, before the v3
extractor runs, per the M1 protocol. Eight reports from the
frozen v3 corpus (v3-manifest.txt), two per stratum: f01,
f04 (gate); f07, f09 (audit-other); f13, f16 (audit-issue);
f19, f22 (closure). Their hashes are the manifest values.
Method and exclusions as in the v1 and v2 hand lists. One
addition for v3: every bare pull-request-form reference is
classified by what it refers to, PR or issue, since the
strata densities in the manifest are computed from these
classifications. Truncated hash prefixes printed with
ellipses are excluded, as before.

## f01 gate-report-cursor-harness-2026-10-26.md (gate, 9689 bytes)

pr explicit: PR #499 (CodeRabbit heading); PR #459 (precedent)
pr bare, all referring to PRs: #493 and #494 (the #493/#494
pair, pricing-pin item, twice: CodeRabbit section and Gates
section)
commit: 98efe913; fcd5f8ea x4; 24d1a6ef x5; 27c6841c x2;
c9a27f0f (pushed head); 8946d8f3 (rebase base); tree 4b5ca16d
gitsha: base 8946d8f3743034c768e93631052801a0fe4de751
tally: 81 passed; 6 failed (break-verification); 28 passed;
4,616 passed, 11 failed, 3 skipped (Rust-active); 4,578
passed, 2 failed, 50 skipped (no-Rust); exit 0 (maturin);
exit 1 x2 (the two suites); exit 0 x3 (smoke and surface
checks)
count: 7 files then 8 with pyproject (diff); six new tests
x2; two Minor findings; 8 Rust-parity failures (of the 11);
2 pricing pins (of the 11); four prose semicolons; two
accepted minors
size: none
Excluded: pricing version strings (2026-10-15.7,
2026-10-23.1), model names, the 10**12 counter bound (a code
constant), durations, $0.0 prices.

f01 total: 47 claims (pr 6, of which bare 4 and all 4 PR
cites; commit 15; gitsha 1; tally 15; count 10; size 0)

## f04 google-decision1-full-run-2026-11-16.md (gate, 21340 bytes)

pr explicit: pull request 547 (adapter section); PR 547
(red team corrections)
pr bare: none in the report
commit: none
gitsha: none (the 32-hex run ids are identifiers, excluded)
hash256: artifact sha256 76f5df67...; final analysis lock
390d94a8...; interim seal lock 5434f93c...
tally: 34 tests with exit code 0 (adapter suite); 69 tests
(phase0, passes in full); full suite exited 1 both times
(2 runs); public surface check exit code 0
count: 4,701 cases x3; 9,402 calls x3; twenty canary
responses; ten canary cases; 2,685,961 input tokens; zero
output tokens; zero benign errors, zero attacked errors,
zero timeouts, zero rate limiting; 4,646 decided correctly;
55 decided wrong; 887 counted flips; 875 counted flips x3;
3,527 no change; 754 decision changed; 362 score shifted; 3
moved to abstain; 15 approve and 4,686 other (attacked);
96 approve and 4,605 other (benign); the per-family table
(11 families, cases and counted flips per row); 25 results
per flush; about 200 retry lines (later corrected in the
report itself); 120 second timeout; 300 second ceiling;
600 second window; 606 seconds (proof A); ten-case canary
(proof B); 2,150, 2,475 and 2,625 stall points; the seven
banking steps 2,950 to 4,250; 400 calls re-spend bound; one
call with retry_count 1; 8 write-refused lines at most
size: artifact 16,125,314 bytes; partial file passed 8 MB
Excluded: the md5 (a different hash kind), the 16-hex
manifest sha prefix (a truncated identifier), run ids,
dollar figures and rates (measurements, not counts of
things), version strings.

f04 total: 71 claims (pr 2, bare 0; commit 0; gitsha 0;
hash256 3; tally 6; count 58; size 2)

## f07 pr326-delta-spec-review-20261001.md (audit-other, 7418 bytes)

pr explicit: PR #326 (title); david-engelmann/peira #326;
pull/326 (fetch refspec in Commands run)
pr bare: none in the report
commit: 9dbcbd30 x6; 1e7a763a x2; 1588ed1b x3; 969f456f;
1f78caca; 1854dc4e x2
gitsha: reviewed head 1588ed1bd931... x2; base
1854dc4e0c26... x2
tally: exit 0 x3 (commands section)
count: 2 commits; 4 files; 63+/39- diffstat; 15 files x2;
6 mock-artifact JSONs; 14 markers; zero call sites; 1 hit
(definition only); 5 commits (log)
size: none
Excluded: finding labels (P2-1 and kin), line references,
the B1-B6 label range.

f07 total: 38 claims (pr 3, bare 0; commit 15; gitsha 4;
tally 3; count 13; size 0)

## f09 pr338-redteam-review-20261002.md (audit-other, 10002 bytes)

pr explicit: PR #338 (title); david-engelmann/peira#338;
PR #315 (dataset integrity section)
pr bare, all referring to PRs: #230 x6; #280 x4; #205;
#338 (the severity-history chain and the R-1/R-2 sections)
commit: 83f6232a x3; 7bc6717c x4; 870f6aa8 x2; 11218153 x3;
21ef7ba4 x5; 2cca2ee1 x2; e2eab9d0
gitsha: reviewed head 1121815365e1... x2
hash256: none printed (the 10 family-file hashes are
counted, not quoted)
tally: exit 0 (tier-word script); 9/9 passed, 0 errors
(dataset gates) x2; exit 0 (verify-manifest); exit 0 x7
(the verification command list); exit 1 x2 (the pre-existing
emdash check)
count: five R-1 cases; all 15 FSA cases x2; critical 3,
high 4, medium 1, low 7; 2 critical and 8 high (originals);
23 refreshed sentences; 21 to Critical and 2 to Medium; 1
hit (false-negative scan); 0 hits; 10 family files; 2,000
cases x2; severity distribution 771, 463, 745, 21; 2,000
lines; 469 warnings; 35 real case changes; 910
serialization lines x3; 1,890 diff lines; 10 files
(diffstat); 3/5 byte-identical
size: 1428 B; 948 B; 1218 B (the byte-identical cases)
Excluded: case ids, dataset version numbers, quoted scale
wordings and case contents, expected_score decimals.

f09 total: 84 claims (pr 15, of which bare 12 and all 12 PR
cites; commit 20; gitsha 2; tally 14; count 30; size 3)

## f13 eb27-eb47-standards-review-20261002.md (audit-issue, 10900 bytes)

Note: this report replaced eb-fairness-spec-review in the
f13 slot during hand-listing, before any extraction; the
swap and its reason are recorded in the manifest.

pr explicit: none
pr bare: no GitHub references at all. Bare #-forms present
are Taxonomy boundary-ruling numbers (#10, #11, #12, five
instances), internal document numbering, neither PR nor
issue; recorded here so the density accounting is complete.
commit: 0b59d92 (in the reviewed head's message)
gitsha: reviewed head 7c19a5c0f3a8...; base
0b59d9208295...
tally: 31 passed, exit code 0; 6 failed, 6 passed (mutation
run)
count: 20 files; +3618/-20; 840 cases x2; 2 new family
reports; 2 authoring scripts; 12 new tests x2; 4 em dashes;
10 spaced-hyphen substitutes x2; 0 em dashes across the
cases; 3 substitutes in metrics comments; 137 pre-existing
em dashes; 27 sibling entries; 19 and 17 asserts; 864
lines; 783 lines; 6 of 12 tests failed (mutation); 22
colons and 6 semicolons (the calibration document); counts
27 to 29
size: none
Excluded: decision ids (D-39, D-44), family numbers, line
references, the median and ratio measurements.

f13 total: 31 claims (pr 0, bare GitHub 0, bare other 5;
commit 1; gitsha 2; tally 4; count 24; size 0)

## f16 noise-resweep-spec-review-20261002.md (audit-issue, 14965 bytes)

pr explicit: none
pr bare, all referring to the GitHub issue: #52 x3 (the
#52 fixture, regenerated and confirmed; the same reference
the v2 adjudication counted as an issue cite in e13)
commit: acdfa73a x5; d42ebcea x4; d877cdb5 x3; 37828661
gitsha: reviewed head d42ebcea3152... x2; rebase base
acdfa73a11b0...
tally: 136 passed; 20 passed; 15 passed; exit 0 x9 (the
command table)
count: 0 firings across 500 seeds x 4 sentences; 6 one-way
pairs; 2 new pairs; 8 paraphrase drops; 3 alt-list trims;
12 context guards x3; 12 red-team breaks; 300 seeds each;
200 seeds each x2; 32 keys to 25 keys; 10/10 rows x2; 14
lane files x2; 3837 insertions, 0 deletions; 4 noise
metrics; about 19 new regression tests; 40 rows, 10 per
class; the 12 per-100-seeds fire rates; 25 surviving keys;
16 break-cases; the three diffstats (474+/60-, 292+/13-,
4+/4-)
size: none
Excluded: finding labels (R1-R4, N1), EB labels, the line
7940 reference, the 0.5 severity weight.

f16 total: 61 claims (pr 3, bare 3 and all 3 issue cites;
commit 13; gitsha 3; tally 12; count 30; size 0)

## f19 fleet-execution-log-2026-10-23.md (closure, 28478 bytes)

pr explicit: PRs #488 (the merged set at Window A's gate
check)
pr bare, all referring to PRs: #485 (the origin/main head's
own description); #489, #490, #491, #493 (the merged set)
commit: 9d1b5c02 x4; 57061f45 x2; 244b3152; dd375f27 x2;
4155451d
gitsha: main head 244b3152b14d... (Window A gate check)
hash256: none (the printed hashes are ellipsis-truncated
or the 16-hex manifest prefix, both excluded)
tally: none
count: 0 instances and 0 volumes x5 (the repeated account
verifications); 2,776 GLiNER cases; 2,701 Von cases;
746-call run; 2,000 Von results; 1,925 GLiNER results;
4-6 worker slots; three consecutive SSH attempts; the
day's 13th and 14th restarts; four instances created; two
provisioning deaths x2
size: the offer rows' disk and RAM figures (569 GB, 159
GB, 388 GB, 74.25 GB, 463 GB, 650 GB, 1461 GB and the RAM
figures beside them); 40 GB proof disk; 20 GB headroom;
the 100 KB round-trip file
Excluded: dollar figures and rates (prices, not sizes of
things), instance and offer ids (identifiers), durations,
the ellipsis-truncated checkpoint hashes.

f19 total: 52 claims (pr 6, of which bare 5 and all 5 PR
cites; commit 10; gitsha 1; tally 0; count 20; size 15)

## f22 perfection-github-readiness-2026-10-23.md (closure, 20659 bytes)

pr explicit: PR #481 (merge, refs line); PR #449 x3 (the
module's landing and the page it produced); PR #448
(merge-base)
pr bare: none in the report
commit: cc2df7a7 x5; fb9d7381; f7770afa; 4c8af10c;
3cfca2b2; 5e70a925 x2; b35dcaff; 292642a0; cbcc0af0
gitsha: origin/main cbcc0af06d5e...
hash256: none (the page hash is an md5, a different kind)
tally: none
count: 5,378 lines; about 5,289 lines; 20 .py files; 9,235
lines; 6 core files at 4,481 lines; 14 component files at
4,754 lines; 1,212, 1,000 and 1,012 lines (dashboard.py);
4,601 lines x3; 4,341 lines x3; 4,834 lines; 1,144 lines;
542 lines; 13 component modules; 22, 80, 60 and 89 lines
(the four publish tools); four functions; pinned 16 times;
referenced once; 535-line diff x3; all 78 functions; 31
more functions; 86 lines; 242 diff lines; 117 lines; 110
lines; three renderer copies; two commits the local branch
lacks; 54 commits ahead; 47 ahead and 40 behind; 113 ahead
and 40 behind
size: 412K (stale build payload); 211M and 227M (run
stores); about 8.7M (screenshots); 438M (untracked stores)
Excluded: the md5s, Round numbers (labels), section
references.

f22 total: 63 claims (pr 5, bare 0; commit 14; gitsha 1;
tally 0; count 38; size 5)

## Totals and densities

Hand-list total: 447 claims across the eight reports (f01
47, f04 71, f07 38, f09 84, f13 31, f16 61, f19 52, f22
63).

Bare references to GitHub objects, by stratum, from these
lists: gate 4 (all PR cites, 0 issue); audit-other 12 (all
PR cites, 0 issue); audit-issue 3 (all issue cites, 0 PR);
closure 5 (all PR cites, 0 issue). Issue-quoting densities
(issue share of bare GitHub references): gate 0.00 (n=4),
audit-other 0.00 (n=12), audit-issue 1.00 (n=3), closure
0.00 (n=5). The audit-issue base is small and is published
as it stands; f13 additionally carries 5 bare #-forms that
are internal Taxonomy ruling numbers, neither PR nor
issue, excluded from the density base.
