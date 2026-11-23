//! Extraction mode: audit a report that carries no evidence
//! markers, per DECISIONS.md D16.
//!
//! The patterns below are a hand-written port of the pattern set
//! v1 whose false positive rates the M1 and Track A measurements
//! established and D15 ratified (banked as docs/m2/extract_v1.py).
//! The port changes no pattern. It covers the classes the mode
//! extracts: the merged PR class, short commit references, full
//! git SHAs, full content hashes, and run tallies. Counts and
//! sizes are not extracted (both classes are out), and the quote
//! pattern stays dropped. The measurement's first-file target
//! diagnostic is not ported; it played no part in any verdict.
//!
//! Checking follows the D16 rule: the bundle answers what it
//! can, in the most specific way available, and the rest is
//! unverifiable. A commit or full SHA checks against the bundled
//! repository. A PR reference checks against the bundled host
//! record of that number. A content hash checks against every
//! file the bundle carries; a match anywhere verifies it, and
//! no match is unverifiable, never contradicted, because the
//! extractor cannot know which file the author meant. A tally
//! checks against every log the bundle carries under the M3
//! last-tally rule. Every verified or contradicted verdict
//! names the evidence it matched. Provisional claims (tallies)
//! never move the exit code alone; the exit code follows D5
//! over trusted claims only.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::{RepoFacts, RunFact, Status, Tri, flat_field, flat_str, last_tally, sha256, FlatVal};

/// Whether a class's extracted claims can move the exit code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trust {
    Trusted,
    Provisional,
}

impl Trust {
    pub fn label(self) -> &'static str {
        match self {
            Trust::Trusted => "trusted",
            Trust::Provisional => "provisional",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XKind {
    Pr(u64),
    Commit(String),
    GitSha(String),
    Hash256(String),
    TallyFact(RunFact, u64),
    /// A tally shape that is not one of the four run facts: a
    /// tests/checks/subtests total, a fraction, or an of-form.
    /// Carries its canonical signature; a bundled log verifies it
    /// only by recording the identical signature, and it is never
    /// contradicted, because different logs record different runs
    /// and no canonical record exists for a total.
    TallyShape(String),
}

/// One extracted claim, before or after checking.
#[derive(Debug, Clone)]
pub struct Extracted {
    pub line: usize,
    pub class: &'static str,
    pub trust: Trust,
    kind: XKind,
}

/// The outcome for one extracted claim.
#[derive(Debug, Clone)]
pub struct ExtractedResult {
    pub line: usize,
    pub class: &'static str,
    pub trust: Trust,
    pub status: Status,
    pub detail: String,
}

/// The outcome of an extraction-mode run.
#[derive(Debug, Clone)]
pub struct Extraction {
    pub results: Vec<ExtractedResult>,
}

impl Extraction {
    pub fn claims_extracted(&self) -> usize {
        self.results.len()
    }

    /// D5 precedence over trusted claims only. A provisional
    /// claim, whatever its status, never moves this code.
    pub fn exit_code(&self) -> i32 {
        let trusted: Vec<Status> = self
            .results
            .iter()
            .filter(|r| r.trust == Trust::Trusted)
            .map(|r| r.status)
            .collect();
        if trusted.contains(&Status::Contradicted) {
            1
        } else if trusted.contains(&Status::Unverifiable) {
            3
        } else if trusted.contains(&Status::Malformed) {
            2
        } else {
            0
        }
    }
}

fn is_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn is_lhex(b: u8) -> bool {
    b.is_ascii_digit() || (b'a'..=b'f').contains(&b)
}

/// The fused-token guard (v1 change 2 and 5, first clause): the
/// character before a number must not be a letter, digit, dot,
/// or underscore.
fn guard_ok(line: &[u8], start: usize) -> bool {
    start == 0 || !(line[start - 1].is_ascii_alphanumeric() || line[start - 1] == b'.' || line[start - 1] == b'_')
}

/// Parse a `\d[\d,]*` token at `i`. Returns the value and the end
/// index (one past the last digit or comma consumed).
fn parse_number(line: &[u8], i: usize) -> Option<(u64, usize)> {
    if i >= line.len() || !line[i].is_ascii_digit() {
        return None;
    }
    let mut j = i;
    while j < line.len() && (line[j].is_ascii_digit() || line[j] == b',') {
        j += 1;
    }
    let text: String = line[i..j].iter().filter(|c| **c != b',').map(|c| *c as char).collect();
    text.parse::<u64>().ok().map(|v| (v, j))
}

/// Parse a run of plain digits at `i` (the `\d+` of the PR and
/// exit patterns, which admit no commas). Returns the value and
/// the end index.
fn parse_digits(line: &[u8], i: usize) -> Option<(u64, usize)> {
    if i >= line.len() || !line[i].is_ascii_digit() {
        return None;
    }
    let mut j = i;
    while j < line.len() && line[j].is_ascii_digit() {
        j += 1;
    }
    let text: String = line[i..j].iter().map(|c| *c as char).collect();
    text.parse::<u64>().ok().map(|v| (v, j))
}

/// Skip `\s+` (spaces and tabs) at `i`; returns the new index, or
/// None when there is no whitespace.
fn skip_ws(line: &[u8], i: usize) -> Option<usize> {
    let mut j = i;
    while j < line.len() && (line[j] == b' ' || line[j] == b'\t') {
        j += 1;
    }
    if j > i {
        Some(j)
    } else {
        None
    }
}

/// Does `line` at `i` start with `word`, followed by a word
/// boundary?
fn word_at(line: &[u8], i: usize, word: &str) -> bool {
    let w = word.as_bytes();
    if i + w.len() > line.len() || &line[i..i + w.len()] != w {
        return false;
    }
    let after = i + w.len();
    after >= line.len() || !is_word(line[after])
}

/// The v1 tally guard, second clause: the token immediately
/// before the number is a commit-like hex token or a single
/// letter.
fn tally_blocked(line: &str, start: usize) -> bool {
    let before = line[..start].trim_end();
    if before.is_empty() {
        return false;
    }
    let token = before
        .split_whitespace()
        .last()
        .unwrap_or("")
        .trim_matches(|c| matches!(c, '`' | '*' | '(' | '|'));
    let bytes = token.as_bytes();
    if bytes.len() == 1 && bytes[0].is_ascii_alphabetic() {
        return true;
    }
    (7..=40).contains(&bytes.len())
        && bytes.iter().all(|b| is_lhex(*b))
        && bytes.iter().any(|b| b.is_ascii_alphabetic())
}

const COMMIT_WORDS: [&str; 7] = ["commit", "head", "base", "sha", "merge", "reviewed", "branch"];

fn extract_line(line_no: usize, line: &str, out: &mut Vec<Extracted>, seen: &mut HashSet<(usize, &'static str, String)>) {
    let bytes = line.as_bytes();
    let mut add = |cls: &'static str, trust: Trust, kind: XKind, key: String, out: &mut Vec<Extracted>| {
        if seen.insert((line_no, cls, key)) {
            out.push(Extracted { line: line_no, class: cls, trust, kind });
        }
    };

    // Hex runs: full hashes, full SHAs, and (context permitting)
    // short commit references. A run is a maximal run of word
    // characters; the shape matches only when the whole run is
    // lowercase hex of the exact length, mirroring the word
    // boundaries of the measured patterns.
    let lower = line.to_lowercase();
    let has_ctx = COMMIT_WORDS.iter().any(|w| lower.contains(w));
    let mut i = 0;
    while i < bytes.len() {
        if !is_word(bytes[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len() && is_word(bytes[i]) {
            i += 1;
        }
        let run = &bytes[start..i];
        if run.iter().all(|b| is_lhex(*b)) {
            match run.len() {
                64 => {
                    let v = line[start..i].to_string();
                    add("hash256", Trust::Trusted, XKind::Hash256(v.clone()), v, out);
                }
                40 => {
                    let v = line[start..i].to_string();
                    add("gitsha", Trust::Trusted, XKind::GitSha(v.clone()), v, out);
                }
                7..=12 if has_ctx && !run.iter().all(|b| b.is_ascii_digit()) => {
                    let v = line[start..i].to_string();
                    add("commit", Trust::Trusted, XKind::Commit(v.clone()), v, out);
                }
                _ => {}
            }
        }
    }

    // Tallies: one left-to-right consuming pass per shape, as the
    // measured extractor ran one finditer pass per pattern.
    for (word, fact) in [
        ("passed", RunFact::Passed),
        ("failed", RunFact::Failed),
        ("skipped", RunFact::Skipped),
    ] {
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i].is_ascii_digit() && guard_ok(bytes, i) {
                if let Some((value, end)) = parse_number(bytes, i) {
                    if let Some(w) = skip_ws(bytes, end) {
                        if word_at(bytes, w, word) {
                            if !tally_blocked(line, i) {
                                add(
                                    "tally",
                                    Trust::Provisional,
                                    XKind::TallyFact(fact, value),
                                    format!("{} {value}", fact.word()),
                                    out,
                                );
                            }
                            i = w + word.len();
                            continue;
                        }
                    }
                }
            }
            i += 1;
        }
    }
    // Totals: N tests / checks / subtests.
    {
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i].is_ascii_digit() && guard_ok(bytes, i) {
                if let Some((value, end)) = parse_number(bytes, i) {
                    if let Some(w) = skip_ws(bytes, end) {
                        for (word, fam) in [
                            ("subtests", "subtest"),
                            ("tests", "test"),
                            ("test", "test"),
                            ("checks", "check"),
                            ("check", "check"),
                        ] {
                            if word_at(bytes, w, word) {
                                if !tally_blocked(line, i) {
                                    add(
                                        "tally",
                                        Trust::Provisional,
                                        XKind::TallyShape(format!("total:{value}:{fam}")),
                                        format!("total {value} {fam}"),
                                        out,
                                    );
                                }
                                i = w + word.len();
                                break;
                            }
                        }
                        if i > w {
                            continue;
                        }
                    }
                }
            }
            i += 1;
        }
    }
    // Exit tallies: "exit 0" or "exit code 0", unguarded in v1.
    {
        let mut i = 0;
        while i + 4 <= bytes.len() {
            if &bytes[i..i + 4] == b"exit" {
                let mut j = i + 4;
                if bytes[j..].starts_with(b" code") {
                    j += 5;
                }
                if let Some(d) = skip_ws(bytes, j) {
                    if let Some((value, end)) = parse_digits(bytes, d) {
                        if end >= bytes.len() || !is_word(bytes[end]) {
                            add(
                                "tally",
                                Trust::Provisional,
                                XKind::TallyFact(RunFact::Exit, value),
                                format!("exit {value}"),
                                out,
                            );
                            i = end;
                            continue;
                        }
                    }
                }
            }
            i += 1;
        }
    }
    // Fractions: A/B gates, checks, tests, or calls.
    {
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i].is_ascii_digit() {
                if let Some((a, e1)) = parse_number(bytes, i) {
                    let mut j = e1;
                    while j < bytes.len() && bytes[j] == b' ' {
                        j += 1;
                    }
                    if j < bytes.len() && bytes[j] == b'/' {
                        let mut k = j + 1;
                        while k < bytes.len() && bytes[k] == b' ' {
                            k += 1;
                        }
                        if let Some((b, e2)) = parse_number(bytes, k) {
                            if let Some(w) = skip_ws(bytes, e2) {
                                for (word, fam) in [
                                    ("gates", "gate"),
                                    ("gate", "gate"),
                                    ("checks", "check"),
                                    ("check", "check"),
                                    ("tests", "test"),
                                    ("test", "test"),
                                    ("calls", "call"),
                                    ("call", "call"),
                                ] {
                                    if word_at(bytes, w, word) {
                                        add(
                                            "tally",
                                            Trust::Provisional,
                                            XKind::TallyShape(format!("frac:{a}:{b}:{fam}")),
                                            format!("frac {a}/{b} {fam}"),
                                            out,
                                        );
                                        i = w + word.len();
                                        break;
                                    }
                                }
                                if i > e2 {
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }
    // Of-forms: A of B.
    {
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i].is_ascii_digit() {
                if let Some((a, e1)) = parse_number(bytes, i) {
                    if let Some(w) = skip_ws(bytes, e1) {
                        if word_at(bytes, w, "of") {
                            if let Some(d) = skip_ws(bytes, w + 2) {
                                if let Some((b, e2)) = parse_number(bytes, d) {
                                    if e2 >= bytes.len() || !is_word(bytes[e2]) {
                                        add(
                                            "tally",
                                            Trust::Provisional,
                                            XKind::TallyShape(format!("of:{a}:{b}")),
                                            format!("of {a} of {b}"),
                                            out,
                                        );
                                        i = e2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }

    // PR references: explicit shapes first, then bare numbers
    // not already claimed explicitly on this line.
    let mut explicit: HashSet<u64> = HashSet::new();
    {
        let mut i = 0;
        while i + 1 < bytes.len() {
            if &bytes[i..i + 2] == b"PR" && (i == 0 || !is_word(bytes[i - 1])) {
                if let Some(h) = skip_ws(bytes, i + 2) {
                    if h < bytes.len() && bytes[h] == b'#' {
                        if let Some((n, _)) = parse_digits(bytes, h + 1) {
                            explicit.insert(n);
                            add("pr", Trust::Trusted, XKind::Pr(n), format!("PR {n}"), out);
                        }
                    }
                }
            }
            i += 1;
        }
    }
    {
        let mut i = 0;
        while i + 5 <= bytes.len() {
            if &bytes[i..i + 5] == b"pull/" {
                if let Some((n, _)) = parse_digits(bytes, i + 5) {
                    explicit.insert(n);
                    add("pr", Trust::Trusted, XKind::Pr(n), format!("PR {n}"), out);
                }
            }
            i += 1;
        }
    }
    for i in 0..bytes.len() {
        if bytes[i] != b'#' {
            continue;
        }
        let Some((n, _)) = parse_digits(bytes, i + 1) else {
            continue;
        };
        // Repository form: a path-like run containing a slash
        // ends at this hash mark.
        let mut s = i;
        while s > 0
            && (bytes[s - 1].is_ascii_alphanumeric()
                || matches!(bytes[s - 1], b'.' | b'_' | b'-' | b'/'))
        {
            s -= 1;
        }
        if line[s..i].contains('/') && s < i {
            explicit.insert(n);
            add("pr", Trust::Trusted, XKind::Pr(n), format!("PR {n}"), out);
        }
    }
    for i in 0..bytes.len() {
        if bytes[i] != b'#' {
            continue;
        }
        if i > 0 && (is_word(bytes[i - 1]) || bytes[i - 1] == b'#') {
            continue;
        }
        let Some((n, end)) = parse_digits(bytes, i + 1) else {
            continue;
        };
        if end - (i + 1) < 2 {
            continue;
        }
        if end < bytes.len() && is_word(bytes[end]) {
            continue;
        }
        if explicit.contains(&n) {
            continue;
        }
        add("pr", Trust::Trusted, XKind::Pr(n), format!("PR {n}"), out);
    }
}

/// Extract every claim the ratified pattern set finds in
/// `report`, in line order. Pure: reads nothing, checks nothing.
pub fn extract_claims(report: &str) -> Vec<Extracted> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    for (idx, line) in report.lines().enumerate() {
        extract_line(idx + 1, line, &mut out, &mut seen);
    }
    out
}

/// Every file the bundle carries, as bundle-relative paths in
/// sorted order, excluding the bundled repository subtree.
fn bundle_files(bundle: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![bundle.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.is_file() {
                if let Ok(rel) = path.strip_prefix(bundle) {
                    let in_repo = rel
                        .components()
                        .next()
                        .map(|c| c.as_os_str().to_string_lossy() == "repo")
                        .unwrap_or(false);
                    if !in_repo {
                        files.push(rel.to_path_buf());
                    }
                }
            }
        }
    }
    files.sort();
    files
}

/// Check every extracted claim in `report` against the bundle at
/// `bundle`. `report_rel` is the report's own path relative to
/// the bundle when the report file lives inside it; that file is
/// excluded from evidence scans so a claim never verifies
/// against the report itself.
pub fn verify_extracted(
    report: &str,
    bundle: &Path,
    repo: Option<&dyn RepoFacts>,
    report_rel: Option<&Path>,
) -> Extraction {
    let claims = extract_claims(report);
    let files = bundle_files(bundle);
    let files: Vec<PathBuf> = files
        .into_iter()
        .filter(|f| Some(f.as_path()) != report_rel)
        .collect();

    // Bundle-wide evidence, gathered once: content hashes by
    // value, and the text of every file that could be a log.
    let mut hash_match: HashMap<String, String> = HashMap::new();
    let mut log_texts: Vec<(String, String)> = Vec::new();
    let need_hash = claims.iter().any(|c| matches!(c.kind, XKind::Hash256(_)));
    let need_logs = claims.iter().any(|c| matches!(c.kind, XKind::TallyFact(..) | XKind::TallyShape(_)));
    if need_hash || need_logs {
        for rel in &files {
            let Ok(bytes) = std::fs::read(bundle.join(rel)) else {
                continue;
            };
            let name = rel.display().to_string();
            if need_hash {
                let hex = sha256::sha256_hex(&bytes);
                hash_match.entry(hex).or_insert_with(|| name.clone());
            }
            if need_logs {
                log_texts.push((name, String::from_utf8_lossy(&bytes).into_owned()));
            }
        }
    }
    // Tally shape signatures by the first log that records them.
    let mut shape_home: HashMap<String, String> = HashMap::new();
    if claims.iter().any(|c| matches!(c.kind, XKind::TallyShape(_))) {
        for (name, text) in &log_texts {
            for claim in extract_claims(text) {
                if let XKind::TallyShape(sig) = &claim.kind {
                    shape_home.entry(sig.clone()).or_insert_with(|| name.clone());
                }
            }
        }
    }

    let mut results = Vec::new();
    for claim in &claims {
        let (status, detail) = match &claim.kind {
            XKind::Commit(sha) | XKind::GitSha(sha) => match repo {
                None => (
                    Status::Unverifiable,
                    "bundle carries no repository".to_string(),
                ),
                Some(r) => match r.resolves_to_commit(sha) {
                    Tri::Yes => (
                        Status::Ok,
                        "resolves in the bundled repository (repo/)".to_string(),
                    ),
                    Tri::No | Tri::Unknown => (
                        Status::Unverifiable,
                        "does not resolve in the bundled repository".to_string(),
                    ),
                },
            },
            XKind::Hash256(hex) => match hash_match.get(hex) {
                Some(name) => (Status::Ok, format!("sha256 matches bundle file {name}")),
                None => (
                    Status::Unverifiable,
                    "no file in the bundle hashes to this value".to_string(),
                ),
            },
            XKind::Pr(number) => {
                let cited = format!("records/pr-{number}.json");
                match crate::read_cited(bundle, &cited) {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        let rec_num = match flat_field(&text, "number") {
                            Some(FlatVal::Num(n)) => n.parse::<u64>().ok(),
                            _ => None,
                        };
                        let state = flat_str(&text, "state");
                        let merged = match flat_field(&text, "merged") {
                            Some(FlatVal::Bool(b)) => Some(b),
                            _ => None,
                        };
                        match (rec_num, state, merged) {
                            (Some(n), Some(state), Some(merged)) if n == *number => (
                                Status::Ok,
                                format!(
                                    "bundled record {cited} shows state {state}, merged {merged}"
                                ),
                            ),
                            (Some(n), _, _) => (
                                Status::Unverifiable,
                                format!("bundled record {cited} describes PR {n}"),
                            ),
                            _ => (
                                Status::Unverifiable,
                                format!("bundled record {cited} lacks the fields this claim needs"),
                            ),
                        }
                    }
                    Err(_) => (
                        Status::Unverifiable,
                        format!("bundle carries no record for PR {number}"),
                    ),
                }
            }
            XKind::TallyFact(fact, want) => {
                let mut agreeing: Option<&str> = None;
                let mut disagreeing: Option<(&str, u64)> = None;
                for (name, text) in &log_texts {
                    match last_tally(text, *fact) {
                        Some(v) if v == *want => {
                            agreeing = Some(name.as_str());
                            break;
                        }
                        Some(v) => {
                            if disagreeing.is_none() {
                                disagreeing = Some((name.as_str(), v));
                            }
                        }
                        None => {}
                    }
                }
                match (agreeing, disagreeing) {
                    (Some(name), _) => (
                        Status::Ok,
                        format!("bundle log {name} records {want} {}", fact.word()),
                    ),
                    (None, Some((name, actual))) => (
                        Status::Contradicted,
                        format!(
                            "bundle log {name} records {actual} {}, the report says {want}",
                            fact.word()
                        ),
                    ),
                    (None, None) => (
                        Status::Unverifiable,
                        format!("no bundled log records a {} tally", fact.word()),
                    ),
                }
            }
            XKind::TallyShape(sig) => match shape_home.get(sig) {
                Some(name) => (
                    Status::Ok,
                    format!("bundle log {name} records the same tally"),
                ),
                None => (
                    Status::Unverifiable,
                    "no bundled log records this tally".to_string(),
                ),
            },
        };
        results.push(ExtractedResult {
            line: claim.line,
            class: claim.class,
            trust: claim.trust,
            status,
            detail,
        });
    }
    Extraction { results }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::io::Write;

    struct StubRepo {
        commits: HashSet<String>,
    }

    impl RepoFacts for StubRepo {
        fn resolves_to_commit(&self, sha: &str) -> Tri {
            if self.commits.iter().any(|c| c.starts_with(sha)) {
                Tri::Yes
            } else {
                Tri::No
            }
        }
        fn is_ancestor(&self, _anc: &str, _desc: &str) -> Tri {
            Tri::Unknown
        }
        fn diff_files(&self, _a: &str, _b: &str) -> Option<Vec<String>> {
            None
        }
    }

    fn bundle_with(files: &[(&str, &[u8])]) -> Dir {
        let dir = Dir::new();
        for (name, bytes) in files {
            let path = dir.path().join(name);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            let mut f = std::fs::File::create(&path).unwrap();
            f.write_all(bytes).unwrap();
        }
        dir
    }

    struct Dir(PathBuf);

    impl Dir {
        fn new() -> Self {
            let base = std::env::temp_dir().join(format!(
                "crosscheck-extract-test-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir_all(&base).unwrap();
            Dir(base)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for Dir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn kinds(report: &str) -> Vec<XKind> {
        extract_claims(report).into_iter().map(|c| c.kind).collect()
    }

    #[test]
    fn bare_pr_suppressed_when_explicit_on_the_line() {
        let ks = kinds("PR #450 landed and #450 is cited again\n#451 stands alone");
        assert_eq!(
            ks,
            vec![XKind::Pr(450), XKind::Pr(451)],
            "the bare repeat of an explicit number adds no claim"
        );
    }

    #[test]
    fn repo_form_pr_extracts() {
        let ks = kinds("see david-engelmann/peira#309 for the ruling");
        assert_eq!(ks, vec![XKind::Pr(309)]);
    }

    #[test]
    fn tally_guard_blocks_fused_label_and_hex_prefix() {
        assert!(kinds("G5 passed").is_empty(), "a fused gate label is not a tally");
        assert!(
            kinds("a1cfc044 15 passed").is_empty(),
            "a tally number right after a hex token is not a tally"
        );
        assert_eq!(
            kinds("the suite ran clean: 15 passed"),
            vec![XKind::TallyFact(RunFact::Passed, 15)]
        );
    }

    #[test]
    fn of_form_decimal_fragment_extracts_as_v1_did() {
        // The v1 pattern set has no guard on the of-form; the
        // port must reproduce the measured behavior, not the v2
        // fix. "0.01 of 1" extracted "01 of 1" in v1.
        let ks = kinds("the flip rate was 0.01 of 1 in the band");
        assert_eq!(ks, vec![XKind::TallyShape("of:1:1".to_string())]);
    }

    #[test]
    fn commit_needs_context_and_letters() {
        assert!(kinds("deadbeef was the value").is_empty());
        assert_eq!(
            kinds("the head is deadbeef today"),
            vec![XKind::Commit("deadbeef".to_string())]
        );
        assert!(
            kinds("1234567 commits landed").is_empty(),
            "a pure digit token is never a commit reference"
        );
    }

    #[test]
    fn hex_lengths_route_to_the_right_class() {
        let h40 = "a".repeat(40);
        let h64 = "b".repeat(64);
        let h41 = "c".repeat(41);
        let ks = kinds(&format!("{h40} and {h64} and {h41}"));
        assert_eq!(
            ks,
            vec![
                XKind::GitSha(h40),
                XKind::Hash256(h64),
            ],
            "41 hex characters is neither a SHA nor a hash"
        );
    }

    #[test]
    fn fraction_and_its_total_both_extract() {
        // The v1 behavior that produced the measured FP4: the
        // fraction and the denominator total are separate claims.
        let ks = kinds("32/38 tests green");
        assert_eq!(
            ks,
            vec![
                XKind::TallyShape("total:38:test".to_string()),
                XKind::TallyShape("frac:32:38:test".to_string()),
            ]
        );
    }

    #[test]
    fn hash_verifies_against_any_bundle_file_and_names_it() {
        let content = b"the evidence bytes";
        let hex = sha256::sha256_hex(content);
        let dir = bundle_with(&[("docs/note.txt", content)]);
        let report = format!("the digest is {hex}");
        let out = verify_extracted(&report, dir.path(), None, None);
        assert_eq!(out.results.len(), 1);
        assert_eq!(out.results[0].status, Status::Ok);
        assert!(
            out.results[0].detail.contains("docs/note.txt"),
            "the verdict names the file it matched: {}",
            out.results[0].detail
        );
        let other = format!("the digest is {}", "0".repeat(64));
        let out = verify_extracted(&other, dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
        assert_eq!(out.exit_code(), 3);
    }

    #[test]
    fn pr_verifies_against_bundled_record_and_names_it() {
        let dir = bundle_with(&[(
            "records/pr-450.json",
            br#"{"number": 450, "state": "closed", "merged": true}"#,
        )]);
        let out = verify_extracted("PR #450 merged last night", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Ok);
        assert!(out.results[0].detail.contains("records/pr-450.json"));
        let out = verify_extracted("PR #451 merged last night", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
    }

    #[test]
    fn commit_verifies_through_the_repo_facts() {
        let repo = StubRepo {
            commits: ["abc1234ffff".to_string()].into_iter().collect(),
        };
        let dir = bundle_with(&[]);
        let out = verify_extracted(
            "the head abc1234 is pushed",
            dir.path(),
            Some(&repo),
            None,
        );
        assert_eq!(out.results[0].status, Status::Ok);
        let out = verify_extracted(
            "the head def5678 is pushed",
            dir.path(),
            Some(&repo),
            None,
        );
        assert_eq!(out.results[0].status, Status::Unverifiable);
    }

    #[test]
    fn tally_checks_against_bundled_logs_and_names_the_log() {
        let dir = bundle_with(&[("run.log", b"running\n15 passed\n0 failed\n")]);
        let out = verify_extracted("the suite saw 15 passed", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Ok);
        assert_eq!(out.results[0].trust, Trust::Provisional);
        assert!(out.results[0].detail.contains("run.log"));
        let out = verify_extracted("the suite saw 14 passed", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Contradicted);
        assert!(out.results[0].detail.contains("run.log"));
    }

    #[test]
    fn provisional_contradiction_never_moves_the_exit_code() {
        let content = b"evidence";
        let hex = sha256::sha256_hex(content);
        let dir = bundle_with(&[("run.log", b"15 passed\n"), ("e.bin", content)]);
        let report = format!("digest {hex}; the suite saw 14 passed");
        let out = verify_extracted(&report, dir.path(), None, None);
        assert_eq!(out.exit_code(), 0, "the only failure is provisional");
        let report = format!("digest {}; the suite saw 15 passed", "0".repeat(64));
        let out = verify_extracted(&report, dir.path(), None, None);
        assert_eq!(out.exit_code(), 3, "a trusted unverifiable claim sets the code");
    }

    #[test]
    fn the_report_itself_is_not_evidence() {
        let dir = bundle_with(&[("copy.md", b"the suite saw 15 passed")]);
        let report = "the suite saw 15 passed";
        let out = verify_extracted(report, dir.path(), None, Some(Path::new("copy.md")));
        assert_eq!(
            out.results[0].status,
            Status::Unverifiable,
            "a claim must not verify against the report's own text"
        );
        let out = verify_extracted(report, dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Ok);
    }
}
