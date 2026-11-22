#!/usr/bin/env python3
"""Generate the verdict viewer's run record from real checks.

Demo tooling for the verdict viewer (Stageshow scope: showcase
material for the deterministic CLI, not the tool itself). It reads
web/data/report.md, extracts claims in exactly the classes the M1
measurement rated trusted (explicit PR references, full git SHAs,
full content hashes, run tallies) plus the provisional short-commit
class, and verifies every claim against the real evidence:

- full and short commit identifiers resolve in this repository's
  own git history (git rev-parse / cat-file, read-only);
- the content hash is recomputed from the cited file on disk;
- the test tally is recomputed by counting #[test] attributes in
  the crates;
- the prototype tallies are recomputed by running the Phase 1
  prototype checker in-process over its honest report and bundle;
- PR claims check against the bundled host records in
  web/data/bundle/records/github-records.json, which are real
  GitHub API responses fetched on 2026-11-22.

A claim whose evidence is not in the bundle comes out
unverifiable. That is the product's fail-closed rule, and the demo
report carries one such claim on purpose, plus one planted error
that must come out contradicted.

Output: web/data/run.json, written to a temp path, verified, then
moved into place. If the box write fault refuses the write, the
JSON is printed to stdout instead and the caller files it.
"""
import hashlib
import importlib.util
import json
import os
import re
import subprocess
import sys
from pathlib import Path

HOME = Path.home()
REPO = HOME / "workspace/crosscheck"
WEB = REPO / "web"
REPORT = WEB / "data/report.md"
RECORDS = WEB / "data/bundle/records/github-records.json"
OUT = WEB / "data/run.json"
PROTO_DIR = HOME / "workspace/retrial-phase1/test"

SHA256_RE = re.compile(r"\b[0-9a-f]{64}\b")
SHA40_RE = re.compile(r"\b[0-9a-f]{40}\b")
SHORTSHA_RE = re.compile(r"\b[0-9a-f]{7,12}\b")
COMMIT_CTX = re.compile(r"commit|head|base|sha|merge|reviewed|branch", re.I)
PR_RE = re.compile(r"\bPR\s*#(\d+)")
PASSED_RE = re.compile(r"(\d[\d,]*)\s+passed\b")
FAILED_RE = re.compile(r"(\d[\d,]*)\s+failed\b")
TESTS_RE = re.compile(r"(\d[\d,]*)\s+(tests?)\b")
EXIT_RE = re.compile(r"exit(?: code)?\s+(\d+)\b")
FILE_RE = re.compile(r"[\w./~-]+\.(?:md|rs|toml|json)\b")


def git(*args):
    p = subprocess.run(
        ["git", "-C", str(REPO), *args],
        capture_output=True, text=True)
    return p.returncode, p.stdout.strip()


def resolve_commit(sha):
    code, out = git("rev-parse", "--verify", "--quiet", f"{sha}^{{commit}}")
    if code != 0 or not out:
        return None
    full = out.splitlines()[0].strip()
    _, subject = git("log", "-1", "--format=%s", full)
    return full, subject


def prototype_tallies():
    spec = importlib.util.spec_from_file_location(
        "retrial_proto", PROTO_DIR / "retrial_proto.py")
    proto = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(proto)
    report = (PROTO_DIR / "report_honest.md").read_text()
    bundle = HOME / "workspace"
    passed = failed = 0
    worst = 0
    for line in report.splitlines():
        if not line.startswith("- CLAIM:"):
            continue
        m = proto.MARKER.search(line)
        status, _ = proto.check(m.group(1), bundle)
        if status == "ok":
            passed += 1
        elif status == "contradicted":
            failed += 1
            worst = max(worst, 1)
        elif status == "unverifiable":
            worst = max(worst, 3)
        elif status == "malformed":
            worst = max(worst, 2)
    return {"passed": passed, "failed": failed, "exit": worst}


def test_count():
    total = 0
    for rs in sorted((REPO / "crates").rglob("*.rs")):
        total += rs.read_text().count("#[test]")
    return total


def main():
    records = json.loads(RECORDS.read_text())
    lines = REPORT.read_text().splitlines()
    tallies = prototype_tallies()
    n_tests = test_count()
    claims = []

    def add(line_no, start, end, cls, trust, value, status, source, detail):
        claims.append({
            "id": f"c{len(claims) + 1}",
            "line": line_no,
            "start": start,
            "end": end,
            "class": cls,
            "trust": trust,
            "value": value,
            "status": status,
            "evidence": {"source": source, "detail": detail},
        })

    for i, line in enumerate(lines, start=1):
        for m in SHA256_RE.finditer(line):
            f = FILE_RE.search(line)
            if not f:
                continue
            target = REPO / f.group(0)
            actual = hashlib.sha256(target.read_bytes()).hexdigest()
            ok = actual == m.group(0)
            add(i, m.start(), m.end(), "hash256", "trusted", m.group(0),
                "verified" if ok else "contradicted",
                f.group(0),
                f"recomputed SHA-256 of the cited file: {actual[:16]}..."
                if ok else
                f"report says {m.group(0)[:16]}..., recomputed {actual[:16]}...")
        for m in SHA40_RE.finditer(line):
            r = resolve_commit(m.group(0))
            if r:
                add(i, m.start(), m.end(), "gitsha", "trusted", m.group(0),
                    "verified", "this repository's git history",
                    f"resolves to a commit: {r[1]}")
            else:
                add(i, m.start(), m.end(), "gitsha", "trusted", m.group(0),
                    "unverifiable", "this repository's git history",
                    "no such commit in the bundled repository")
        if COMMIT_CTX.search(line):
            for m in SHORTSHA_RE.finditer(line):
                if re.fullmatch(r"\d+", m.group(0)):
                    continue
                r = resolve_commit(m.group(0))
                if r:
                    add(i, m.start(), m.end(), "commit", "provisional",
                        m.group(0), "verified",
                        "this repository's git history",
                        f"unique prefix of {r[0][:12]}...: {r[1]}")
                else:
                    add(i, m.start(), m.end(), "commit", "provisional",
                        m.group(0), "unverifiable",
                        "this repository's git history",
                        "does not resolve in the bundled repository; "
                        "the claim's evidence is outside the bundle")
        for m in PR_RE.finditer(line):
            num = int(m.group(1))
            if "peira" in line:
                rec = records["pr568"]
                ok = num == rec["number"] and rec["merged"] and rec["state"] == "closed"
                add(i, m.start(), m.end(), "pr", "trusted", f"PR #{num}",
                    "verified" if ok else "contradicted",
                    "bundle record: GET /repos/david-engelmann/peira/pulls/568",
                    f"record shows state {rec['state']}, merged {str(rec['merged']).lower()}, "
                    f"merged at {rec['merged_at']}, merge commit "
                    f"{rec['merge_commit_sha'][:12]}...")
            else:
                rec = records["pulls"]
                absent = num not in rec["numbers"]
                absence_claim = "never" in line or "no pull" in line
                ok = absent if absence_claim else not absent
                add(i, m.start(), m.end(), "pr", "trusted", f"PR #{num}",
                    "verified" if ok else "contradicted",
                    "bundle record: GET /repos/david-engelmann/retrial/pulls?state=all",
                    f"record stands at count {rec['count']}; "
                    f"PR #{num} has never been opened on this repository"
                    if ok else "the bundled pulls record disagrees")
        for regex, kind in ((TESTS_RE, "tests"), (PASSED_RE, "passed"),
                            (FAILED_RE, "failed"), (EXIT_RE, "exit")):
            for m in regex.finditer(line):
                want = int(m.group(1).replace(",", ""))
                if kind == "tests":
                    actual = n_tests
                    source = "recomputation: #[test] count across crates/"
                    detail = f"recomputed {actual} tests in the workspace crates"
                else:
                    actual = tallies[kind]
                    source = "recomputation: Phase 1 prototype over its honest report"
                    detail = (f"prototype re-run in-process: {tallies['passed']} passed, "
                              f"{tallies['failed']} failed, exit {tallies['exit']}")
                add(i, m.start(), m.end(), "tally", "trusted", m.group(0),
                    "verified" if actual == want else "contradicted",
                    source, detail)

    claims.sort(key=lambda c: (c["line"], c["start"]))
    for n, c in enumerate(claims, start=1):
        c["id"] = f"c{n}"
    summary = {
        "claims": len(claims),
        "verified": sum(1 for c in claims
                        if c["status"] == "verified" and c["trust"] == "trusted"),
        "provisional": sum(1 for c in claims
                           if c["status"] == "verified" and c["trust"] == "provisional"),
        "contradicted": sum(1 for c in claims if c["status"] == "contradicted"),
        "unverifiable": sum(1 for c in claims if c["status"] == "unverifiable"),
    }
    run = {
        "generated_at": "2026-11-22",
        "generator": "web/scripts/generate_run.py",
        "title": "Crosscheck verdict viewer — a real report, checked claim by claim",
        "limit_line": "Crosscheck proves that cited spans exist, hashes match, "
                      "counts recompute, and cited commits resolve. It does not "
                      "prove a span supports the sentence it is attached to.",
        "report_file": "report.md",
        "claims": claims,
        "summary": summary,
    }
    text = json.dumps(run, indent=2)
    tmp = OUT.with_suffix(".json.tmp")
    try:
        tmp.write_text(text)
        if json.loads(tmp.read_text()) is None or tmp.stat().st_size == 0:
            raise OSError("write verification failed")
        os.replace(tmp, OUT)
        print(f"wrote {OUT} ({OUT.stat().st_size} bytes)")
    except PermissionError:
        print(text)
        return 0
    print(json.dumps(summary))
    return 0


if __name__ == "__main__":
    sys.exit(main())
