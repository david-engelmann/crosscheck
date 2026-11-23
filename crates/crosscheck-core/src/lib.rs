//! Crosscheck core: deterministic verification of agent work reports
//! against an evidence bundle.
//!
//! A report is a Markdown document. A line that begins with
//! `- CLAIM:` is a claim, and it must end with exactly one evidence
//! marker of one of these forms:
//!
//! - `[evidence: exists PATH]`
//! - `[evidence: size PATH == N]`
//! - `[evidence: sha256 PATH == HEX]`
//! - `[evidence: quote "SPAN" in PATH]`
//! - `[evidence: count "PATTERN" in PATH == N]`
//! - `[evidence: commit SHA]`
//! - `[evidence: ancestor A of B]`
//! - `[evidence: diff A..B files == N]`
//! - `[evidence: diff A..B touches PATH]`
//! - `[evidence: run FACT in PATH == N]`
//! - `[evidence: pr N state == merged]` or `== open`
//! - `[evidence: ci SHA state == success]` or `== failure`
//!
//! A run fact is one of passed, failed, skipped, or exit, checked
//! against the tallies recorded in the cited log. The last tally
//! of that fact in the log is the record. A fact the log does not
//! record leaves the claim unverifiable; a tally that disagrees
//! contradicts it (DECISIONS.md D14). Pull request and CI claims
//! check against flat records the bundle carries under `records/`.
//! A missing record is unverifiable, never a pass.
//!
//! The git forms check against the repository the bundle carries,
//! reached through the [`RepoFacts`] interface; the core itself
//! spawns no processes and reads no repository directly. A bundle
//! with no repository makes every git claim unverifiable.
//!
//! Paths resolve inside the bundle directory only. The count pattern
//! is a literal string, with a leading `^` anchoring it to the start
//! of a line; there is no regular expression engine in the core.
//! Nothing a report names is executed, and no network is used. See
//! DECISIONS.md and docs/INVARIANTS.md in the repository for the
//! contract this code implements.

pub mod sha256;

use std::path::{Component, Path, PathBuf};

/// The mechanical check a claim asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spec {
    Exists { path: String },
    Size { path: String, want: u64 },
    Sha256 { path: String, want: String },
    Quote { span: String, path: String },
    Count { pattern: String, path: String, want: u64 },
    Commit { sha: String },
    Ancestor { anc: String, desc: String },
    DiffFiles { a: String, b: String, want: u64 },
    DiffTouches { a: String, b: String, path: String },
    RunFact { fact: RunFact, path: String, want: u64 },
    PrState { number: u64, want_merged: bool },
    CiState { sha: String, want: String },
}

/// One fact a run log can record, per DECISIONS.md D14.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunFact {
    Passed,
    Failed,
    Skipped,
    Exit,
}

impl RunFact {
    fn word(self) -> &'static str {
        match self {
            RunFact::Passed => "passed",
            RunFact::Failed => "failed",
            RunFact::Skipped => "skipped",
            RunFact::Exit => "exit",
        }
    }
}

/// A three-way fact from the bundled repository: the fact holds,
/// it does not hold, or it cannot be determined from the evidence
/// the bundle carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tri {
    Yes,
    No,
    Unknown,
}

/// Facts about the repository a bundle carries. The core asks only
/// these questions; how the answers are produced (the CLI uses the
/// auditor's git against the bundled repository) is an adapter
/// concern. Identifiers handed to an implementation have already
/// been validated as hex of bounded length by the core.
pub trait RepoFacts {
    /// Does `sha` resolve to exactly one commit object?
    fn resolves_to_commit(&self, sha: &str) -> Tri;
    /// Is `anc` an ancestor of `desc`? Unknown if either side does
    /// not resolve.
    fn is_ancestor(&self, anc: &str, desc: &str) -> Tri;
    /// The files the diff between `a` and `b` touches, as
    /// repository-relative paths. None if the diff cannot be
    /// determined (either side unresolvable, or no repository).
    fn diff_files(&self, a: &str, b: &str) -> Option<Vec<String>>;
}

/// How one claim came out against the evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok,
    Contradicted,
    Unverifiable,
    Malformed,
}

impl Status {
    pub fn label(self) -> &'static str {
        match self {
            Status::Ok => "OK",
            Status::Contradicted => "CONTRADICTED",
            Status::Unverifiable => "UNVERIFIABLE",
            Status::Malformed => "MALFORMED",
        }
    }
}

/// The outcome for one claim line.
#[derive(Debug, Clone)]
pub struct ClaimResult {
    pub line: usize,
    pub status: Status,
    pub detail: String,
}

/// The outcome for a whole report. The exit code follows the
/// precedence in DECISIONS.md D5: contradicted first, then
/// unverifiable, then malformed.
#[derive(Debug, Clone)]
pub struct Verdict {
    pub results: Vec<ClaimResult>,
}

impl Verdict {
    pub fn claims_checked(&self) -> usize {
        self.results.len()
    }

    pub fn exit_code(&self) -> i32 {
        if self.results.iter().any(|r| r.status == Status::Contradicted) {
            1
        } else if self.results.iter().any(|r| r.status == Status::Unverifiable) {
            3
        } else if self.results.iter().any(|r| r.status == Status::Malformed) {
            2
        } else {
            0
        }
    }
}

fn parse_spec(spec: &str) -> Result<Spec, String> {
    let spec = spec.trim();
    if let Some(path) = spec.strip_prefix("exists ") {
        return Ok(Spec::Exists { path: path.trim().to_string() });
    }
    if let Some(rest) = spec.strip_prefix("size ") {
        let (path, want) = split_eq(rest)?;
        let want = want.parse::<u64>().map_err(|_| "size is not a number".to_string())?;
        return Ok(Spec::Size { path, want });
    }
    if let Some(rest) = spec.strip_prefix("sha256 ") {
        let (path, want) = split_eq(rest)?;
        if want.len() != 64 || !want.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("sha256 value is not 64 hex characters".to_string());
        }
        return Ok(Spec::Sha256 { path, want: want.to_lowercase() });
    }
    if let Some(rest) = spec.strip_prefix("quote ") {
        let (span, path) = split_quoted(rest, " in ")?;
        return Ok(Spec::Quote { span, path });
    }
    if let Some(rest) = spec.strip_prefix("count ") {
        let (pattern, rest) = split_quoted_raw(rest)?;
        let rest = rest.strip_prefix(" in ").ok_or("count marker missing ' in '")?;
        let (path, want) = split_eq(rest)?;
        let want = want.parse::<u64>().map_err(|_| "count is not a number".to_string())?;
        return Ok(Spec::Count { pattern, path, want });
    }
    if let Some(rest) = spec.strip_prefix("commit ") {
        let sha = rest.trim().to_string();
        if !valid_sha(&sha) {
            return Err("commit identifier is not 7 to 40 hex characters".to_string());
        }
        return Ok(Spec::Commit { sha });
    }
    if let Some(rest) = spec.strip_prefix("ancestor ") {
        let (anc, desc) = rest
            .split_once(" of ")
            .ok_or("ancestor marker missing ' of '")?;
        let (anc, desc) = (anc.trim().to_string(), desc.trim().to_string());
        if !valid_sha(&anc) || !valid_sha(&desc) {
            return Err("ancestor identifiers are not 7 to 40 hex characters".to_string());
        }
        return Ok(Spec::Ancestor { anc, desc });
    }
    if let Some(rest) = spec.strip_prefix("diff ") {
        let (a, rest) = rest
            .split_once("..")
            .ok_or("diff marker missing '..'")?;
        let (b, tail) = rest
            .split_once(' ')
            .ok_or("diff marker missing its claim")?;
        let (a, b) = (a.trim().to_string(), b.trim().to_string());
        if !valid_sha(&a) || !valid_sha(&b) {
            return Err("diff identifiers are not 7 to 40 hex characters".to_string());
        }
        if let Some(num) = tail.strip_prefix("files == ") {
            let want = num
                .trim()
                .parse::<u64>()
                .map_err(|_| "diff file count is not a number".to_string())?;
            return Ok(Spec::DiffFiles { a, b, want });
        }
        if let Some(path) = tail.strip_prefix("touches ") {
            let path = path.trim().to_string();
            let p = Path::new(&path);
            if p.is_absolute()
                || p.components()
                    .any(|c| matches!(c, Component::ParentDir | Component::RootDir | Component::Prefix(_)))
            {
                return Err("diff path escapes the repository".to_string());
            }
            return Ok(Spec::DiffTouches { a, b, path });
        }
        return Err("diff marker is neither 'files == N' nor 'touches PATH'".to_string());
    }
    if let Some(rest) = spec.strip_prefix("run ") {
        let (fact_word, rest) = rest
            .split_once(' ')
            .ok_or("run marker missing its log")?;
        let fact = match fact_word {
            "passed" => RunFact::Passed,
            "failed" => RunFact::Failed,
            "skipped" => RunFact::Skipped,
            "exit" => RunFact::Exit,
            _ => return Err(format!("unknown run fact: {fact_word}")),
        };
        let rest = rest
            .strip_prefix("in ")
            .ok_or("run marker missing ' in '")?;
        let (path, want) = split_eq(rest)?;
        let want = want
            .parse::<u64>()
            .map_err(|_| "run fact value is not a number".to_string())?;
        return Ok(Spec::RunFact { fact, path, want });
    }
    if let Some(rest) = spec.strip_prefix("pr ") {
        let (num, want) = split_eq(rest)?;
        let number = num
            .strip_suffix(" state")
            .ok_or("pr marker missing ' state'")?
            .parse::<u64>()
            .map_err(|_| "pr number is not a number".to_string())?;
        let want_merged = match want.as_str() {
            "merged" => true,
            "open" => false,
            _ => return Err(format!("unknown pr state: {want}")),
        };
        return Ok(Spec::PrState { number, want_merged });
    }
    if let Some(rest) = spec.strip_prefix("ci ") {
        let (sha, want) = split_eq(rest)?;
        let sha = sha
            .strip_suffix(" state")
            .ok_or("ci marker missing ' state'")?
            .to_string();
        if !valid_sha(&sha) {
            return Err("ci identifier is not 7 to 40 hex characters".to_string());
        }
        if want != "success" && want != "failure" {
            return Err(format!("unknown ci state: {want}"));
        }
        return Ok(Spec::CiState { sha, want });
    }
    Err(format!("unrecognised evidence marker: {spec}"))
}

fn valid_sha(s: &str) -> bool {
    (7..=40).contains(&s.len()) && s.chars().all(|c| c.is_ascii_hexdigit())
}

fn split_eq(rest: &str) -> Result<(String, String), String> {
    let (a, b) = rest
        .split_once(" == ")
        .ok_or_else(|| "marker missing ' == '".to_string())?;
    Ok((a.trim().to_string(), b.trim().to_string()))
}

fn split_quoted(rest: &str, sep: &str) -> Result<(String, String), String> {
    let (quoted, tail) = split_quoted_raw(rest)?;
    let tail = tail
        .strip_prefix(sep)
        .ok_or_else(|| format!("marker missing '{sep}'"))?;
    Ok((quoted, tail.trim().to_string()))
}

fn split_quoted_raw(rest: &str) -> Result<(String, String), String> {
    let rest = rest.strip_prefix('"').ok_or("expected opening quote")?;
    let end = rest.find('"').ok_or("unterminated quoted string")?;
    Ok((rest[..end].to_string(), rest[end + 1..].to_string()))
}

fn contained(bundle: &Path, cited: &str) -> Result<PathBuf, String> {
    let rel = Path::new(cited);
    if rel.is_absolute()
        || rel
            .components()
            .any(|c| matches!(c, Component::ParentDir | Component::RootDir | Component::Prefix(_)))
    {
        return Err("cited path escapes the bundle".to_string());
    }
    Ok(bundle.join(rel))
}

fn read_cited(bundle: &Path, cited: &str) -> Result<Vec<u8>, (Status, String)> {
    let path = contained(bundle, cited).map_err(|e| (Status::Malformed, e))?;
    if !path.is_file() {
        return Err((Status::Unverifiable, "cited file missing".to_string()));
    }
    std::fs::read(&path).map_err(|e| (Status::Unverifiable, format!("cited file unreadable: {e}")))
}

fn check_git(spec: &Spec, repo: Option<&dyn RepoFacts>) -> (Status, String) {
    let Some(repo) = repo else {
        return (
            Status::Unverifiable,
            "bundle carries no repository".to_string(),
        );
    };
    match spec {
        Spec::Commit { sha } => match repo.resolves_to_commit(sha) {
            Tri::Yes => (Status::Ok, "commit resolves".to_string()),
            Tri::No | Tri::Unknown => (
                Status::Unverifiable,
                "cited commit does not resolve in the bundled repository".to_string(),
            ),
        },
        Spec::Ancestor { anc, desc } => match repo.is_ancestor(anc, desc) {
            Tri::Yes => (Status::Ok, "ancestry holds".to_string()),
            Tri::No => (
                Status::Contradicted,
                "cited commit is not an ancestor of the cited descendant".to_string(),
            ),
            Tri::Unknown => (
                Status::Unverifiable,
                "ancestry cannot be determined from the bundled repository".to_string(),
            ),
        },
        Spec::DiffFiles { a, b, want } => match repo.diff_files(a, b) {
            Some(files) => {
                let actual = files.len() as u64;
                if actual == *want {
                    (Status::Ok, format!("diff touches {actual} files"))
                } else {
                    (
                        Status::Contradicted,
                        format!("report says {want} files, diff touches {actual}"),
                    )
                }
            }
            None => (
                Status::Unverifiable,
                "diff cannot be determined from the bundled repository".to_string(),
            ),
        },
        Spec::DiffTouches { a, b, path } => match repo.diff_files(a, b) {
            Some(files) => {
                if files.iter().any(|f| f == path) {
                    (Status::Ok, "path is in the diff".to_string())
                } else {
                    (
                        Status::Contradicted,
                        "path is not among the files the diff touches".to_string(),
                    )
                }
            }
            None => (
                Status::Unverifiable,
                "diff cannot be determined from the bundled repository".to_string(),
            ),
        },
        _ => unreachable!("check_git called with a non-git spec"),
    }
}

/// The last tally of `fact` recorded in a run log, per D14. A
/// tally is a number token immediately followed by the fact word
/// (for exit, an `exit` or `exited` token followed by a number,
/// optionally through the word `code`). Logs append, so the last
/// match is the run's record.
fn last_tally(text: &str, fact: RunFact) -> Option<u64> {
    let mut found = None;
    for line in text.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        for (i, tok) in tokens.iter().enumerate() {
            if fact == RunFact::Exit {
                if *tok == "exit" || *tok == "exited" {
                    if let Some(v) = tokens.get(i + 1).and_then(|t| t.parse::<u64>().ok()) {
                        found = Some(v);
                    } else if tokens.get(i + 1).map(|t| t.trim_end_matches(':')) == Some("code") {
                        if let Some(v) = tokens.get(i + 2).and_then(|t| t.parse::<u64>().ok()) {
                            found = Some(v);
                        }
                    }
                }
            } else if let Ok(v) = tok.parse::<u64>() {
                if let Some(next) = tokens.get(i + 1) {
                    let word = next.trim_end_matches(|c| matches!(c, ';' | ',' | '.' | ':'));
                    if word == fact.word() {
                        found = Some(v);
                    }
                }
            }
        }
    }
    found
}

/// One value in a flat host record, per D14. Records are single
/// JSON objects with flat fields; this reader is deliberately
/// only that, and a nested payload must be trimmed to a flat
/// record when the bundle is assembled.
#[derive(Debug, PartialEq)]
enum FlatVal {
    Str(String),
    Bool(bool),
    Num(String),
}

fn flat_field(text: &str, key: &str) -> Option<FlatVal> {
    let needle = format!("\"{key}\"");
    let at = text.find(&needle)? + needle.len();
    let rest = text[at..].trim_start().strip_prefix(':')?.trim_start();
    if let Some(s) = rest.strip_prefix('"') {
        let mut out = String::new();
        let mut chars = s.chars();
        loop {
            match chars.next()? {
                '"' => break,
                '\\' => {
                    let e = chars.next()?;
                    out.push(match e {
                        'n' => '\n',
                        't' => '\t',
                        _ => e,
                    });
                }
                c => out.push(c),
            }
        }
        return Some(FlatVal::Str(out));
    }
    let end = rest.find(|c| c == ',' || c == '}').unwrap_or(rest.len());
    let tok = rest[..end].trim();
    match tok {
        "true" => Some(FlatVal::Bool(true)),
        "false" => Some(FlatVal::Bool(false)),
        _ if tok.parse::<i64>().is_ok() => Some(FlatVal::Num(tok.to_string())),
        _ => None,
    }
}

fn flat_str(text: &str, key: &str) -> Option<String> {
    match flat_field(text, key) {
        Some(FlatVal::Str(s)) => Some(s),
        _ => None,
    }
}

fn check_pr_record(bytes: &[u8], number: u64, want_merged: bool) -> (Status, String) {
    let text = String::from_utf8_lossy(bytes);
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
        (Some(n), Some(state), Some(merged)) => {
            if n != number {
                return (
                    Status::Unverifiable,
                    format!("bundled record describes PR {n}, not PR {number}"),
                );
            }
            let holds = if want_merged {
                state == "closed" && merged
            } else {
                state == "open"
            };
            if holds {
                (
                    Status::Ok,
                    format!("record shows the pull request {}", if want_merged { "merged" } else { "open" }),
                )
            } else {
                (
                    Status::Contradicted,
                    format!("record shows state {state}, merged {merged}"),
                )
            }
        }
        _ => (
            Status::Unverifiable,
            "bundled record lacks the fields this claim needs".to_string(),
        ),
    }
}

fn check_ci_record(bytes: &[u8], sha: &str, want: &str) -> (Status, String) {
    let text = String::from_utf8_lossy(bytes);
    let head = flat_str(&text, "head_sha");
    let status = flat_str(&text, "status");
    let conclusion = flat_str(&text, "conclusion");
    match (head, status, conclusion) {
        (Some(head), Some(status), Some(conclusion)) => {
            if !head.to_lowercase().starts_with(&sha.to_lowercase()) {
                return (
                    Status::Unverifiable,
                    "bundled record describes a different head".to_string(),
                );
            }
            if status != "completed" {
                return (
                    Status::Unverifiable,
                    "the recorded run has not completed".to_string(),
                );
            }
            if conclusion == want {
                (Status::Ok, format!("record shows the run {conclusion}"))
            } else {
                (
                    Status::Contradicted,
                    format!("record shows the run {conclusion}"),
                )
            }
        }
        _ => (
            Status::Unverifiable,
            "bundled record lacks the fields this claim needs".to_string(),
        ),
    }
}

fn check(spec: &Spec, bundle: &Path, repo: Option<&dyn RepoFacts>) -> (Status, String) {
    match spec {
        Spec::Exists { path } => match contained(bundle, path) {
            Ok(p) if p.is_file() => (Status::Ok, "exists".to_string()),
            Ok(_) => (Status::Unverifiable, "cited file missing".to_string()),
            Err(e) => (Status::Malformed, e),
        },
        Spec::Size { path, want } => match read_cited(bundle, path) {
            Ok(bytes) => {
                let actual = bytes.len() as u64;
                if actual == *want {
                    (Status::Ok, format!("size {actual}"))
                } else {
                    (
                        Status::Contradicted,
                        format!("report says {want} bytes, bundle says {actual}"),
                    )
                }
            }
            Err(e) => e,
        },
        Spec::Sha256 { path, want } => match read_cited(bundle, path) {
            Ok(bytes) => {
                let actual = sha256::sha256_hex(&bytes);
                if actual == *want {
                    (Status::Ok, "sha256 matches".to_string())
                } else {
                    (
                        Status::Contradicted,
                        format!(
                            "report says {}..., bundle says {}...",
                            &want[..12],
                            &actual[..12]
                        ),
                    )
                }
            }
            Err(e) => e,
        },
        Spec::Quote { span, path } => match read_cited(bundle, path) {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                if text.contains(span.as_str()) {
                    (Status::Ok, "span found verbatim".to_string())
                } else {
                    (
                        Status::Contradicted,
                        "quoted span not found verbatim in cited file".to_string(),
                    )
                }
            }
            Err(e) => e,
        },
        Spec::Count { pattern, path, want } => match read_cited(bundle, path) {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                let actual = if let Some(prefix) = pattern.strip_prefix('^') {
                    text.lines().filter(|l| l.starts_with(prefix)).count() as u64
                } else {
                    text.lines().filter(|l| l.contains(pattern.as_str())).count() as u64
                };
                if actual == *want {
                    (Status::Ok, format!("count {actual}"))
                } else {
                    (
                        Status::Contradicted,
                        format!("report says {want}, recomputed {actual}"),
                    )
                }
            }
            Err(e) => e,
        },
        Spec::RunFact { fact, path, want } => match read_cited(bundle, path) {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                match last_tally(&text, *fact) {
                    Some(actual) if actual == *want => (
                        Status::Ok,
                        format!("{} recorded as {actual}", fact.word()),
                    ),
                    Some(actual) => (
                        Status::Contradicted,
                        format!(
                            "report says {} {}, cited log records {actual}",
                            want,
                            fact.word()
                        ),
                    ),
                    None => (
                        Status::Unverifiable,
                        format!("cited log records no {} tally", fact.word()),
                    ),
                }
            }
            Err(e) => e,
        },
        Spec::PrState { number, want_merged } => {
            let cited = format!("records/pr-{number}.json");
            match read_cited(bundle, &cited) {
                Ok(bytes) => check_pr_record(&bytes, *number, *want_merged),
                Err(e) => e,
            }
        }
        Spec::CiState { sha, want } => {
            let cited = format!("records/ci-{sha}.json");
            match read_cited(bundle, &cited) {
                Ok(bytes) => check_ci_record(&bytes, sha, want),
                Err(e) => e,
            }
        }
        Spec::Commit { .. }
        | Spec::Ancestor { .. }
        | Spec::DiffFiles { .. }
        | Spec::DiffTouches { .. } => check_git(spec, repo),
    }
}

/// Verify every claim in `report` against the bundle at `bundle`,
/// with no repository: git claims come out unverifiable. Claim
/// lines are lines beginning with `- CLAIM:`. A claim line without
/// a parseable evidence marker is malformed, never skipped.
pub fn verify_report(report: &str, bundle: &Path) -> Verdict {
    verify_report_repo(report, bundle, None)
}

/// Verify every claim in `report` against the bundle at `bundle`,
/// with git claims checked against `repo` when the bundle carries
/// one.
pub fn verify_report_repo(
    report: &str,
    bundle: &Path,
    repo: Option<&dyn RepoFacts>,
) -> Verdict {
    let mut results = Vec::new();
    for (idx, line) in report.lines().enumerate() {
        if !line.starts_with("- CLAIM:") {
            continue;
        }
        let line_no = idx + 1;
        let marker = line.rfind("[evidence:").and_then(|start| {
            line[start..]
                .strip_prefix("[evidence:")
                .and_then(|s| s.strip_suffix(']'))
        });
        let result = match marker {
            None => ClaimResult {
                line: line_no,
                status: Status::Malformed,
                detail: "claim carries no evidence marker".to_string(),
            },
            Some(spec_text) => match parse_spec(spec_text) {
                Ok(spec) => {
                    let (status, detail) = check(&spec, bundle, repo);
                    ClaimResult { line: line_no, status, detail }
                }
                Err(e) => ClaimResult { line: line_no, status: Status::Malformed, detail: e },
            },
        };
        results.push(result);
    }
    Verdict { results }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn bundle_with(files: &[(&str, &[u8])]) -> tempfile_like::Dir {
        let dir = tempfile_like::Dir::new();
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

    // A tiny stand-in for a tempdir so the core test suite needs no
    // external crate. Directories are created under the system temp
    // dir with a process-unique suffix and removed on drop.
    mod tempfile_like {
        use std::path::PathBuf;

        pub struct Dir(PathBuf);

        impl Dir {
            pub fn new() -> Self {
                use std::sync::atomic::{AtomicU64, Ordering};
                static NEXT: AtomicU64 = AtomicU64::new(0);
                let n = NEXT.fetch_add(1, Ordering::Relaxed);
                let mut p = std::env::temp_dir();
                p.push(format!("crosscheck-core-test-{}-{n}", std::process::id()));
                let _ = std::fs::remove_dir_all(&p);
                std::fs::create_dir_all(&p).unwrap();
                Dir(p)
            }

            pub fn path(&self) -> &std::path::Path {
                &self.0
            }
        }

        impl Drop for Dir {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
    }

    #[test]
    fn honest_claims_all_verify() {
        let dir = bundle_with(&[("doc.md", b"# Title\n\nbody text here\n")]);
        let report = "- CLAIM: it exists [evidence: exists doc.md]\n\
                      - CLAIM: size [evidence: size doc.md == 24]\n\
                      - CLAIM: quote [evidence: quote \"body text\" in doc.md]\n\
                      - CLAIM: count [evidence: count \"^# \" in doc.md == 1]\n";
        let verdict = verify_report(report, dir.path());
        assert_eq!(verdict.exit_code(), 0, "results: {:?}", verdict.results);
        assert_eq!(verdict.claims_checked(), 4);
    }

    #[test]
    fn wrong_size_is_contradicted() {
        let dir = bundle_with(&[("doc.md", b"abc")]);
        let report = "- CLAIM: size [evidence: size doc.md == 99]\n";
        let verdict = verify_report(report, dir.path());
        assert_eq!(verdict.exit_code(), 1);
        assert_eq!(verdict.results[0].status, Status::Contradicted);
    }

    #[test]
    fn missing_file_is_unverifiable() {
        let dir = bundle_with(&[]);
        let report = "- CLAIM: size [evidence: size gone.md == 3]\n";
        let verdict = verify_report(report, dir.path());
        assert_eq!(verdict.exit_code(), 3);
    }

    #[test]
    fn uncited_claim_is_malformed() {
        let dir = bundle_with(&[]);
        let report = "- CLAIM: the gate ran 205 tests\n";
        let verdict = verify_report(report, dir.path());
        assert_eq!(verdict.exit_code(), 2);
    }

    #[test]
    fn escaping_path_is_malformed() {
        let dir = bundle_with(&[]);
        let report = "- CLAIM: x [evidence: exists ../outside.md]\n";
        let verdict = verify_report(report, dir.path());
        assert_eq!(verdict.results[0].status, Status::Malformed);
    }

    #[test]
    fn contradicted_outranks_unverifiable() {
        let dir = bundle_with(&[("doc.md", b"abc")]);
        let report = "- CLAIM: a [evidence: size gone.md == 1]\n\
                      - CLAIM: b [evidence: size doc.md == 99]\n";
        let verdict = verify_report(report, dir.path());
        assert_eq!(verdict.exit_code(), 1);
    }

    // A synthetic commit graph for the git claim classes:
    // 1111111a <- 2222222b <- 3333333c, and 1111111a <- 4444444d.
    struct StubRepo;

    impl StubRepo {
        fn resolve(&self, sha: &str) -> Option<String> {
            let keys = ["1111111a", "2222222b", "3333333c", "4444444d"];
            let found: Vec<&&str> = keys.iter().filter(|k| k.starts_with(sha)).collect();
            if found.len() == 1 {
                Some(found[0].to_string())
            } else {
                None
            }
        }
    }

    impl RepoFacts for StubRepo {
        fn resolves_to_commit(&self, sha: &str) -> Tri {
            if self.resolve(sha).is_some() {
                Tri::Yes
            } else {
                Tri::No
            }
        }

        fn is_ancestor(&self, anc: &str, desc: &str) -> Tri {
            let parents: std::collections::HashMap<&str, Vec<&str>> =
                std::collections::HashMap::from([
                    ("1111111a", vec![]),
                    ("2222222b", vec!["1111111a"]),
                    ("3333333c", vec!["2222222b"]),
                    ("4444444d", vec!["1111111a"]),
                ]);
            let (Some(anc), Some(desc)) = (self.resolve(anc), self.resolve(desc)) else {
                return Tri::Unknown;
            };
            let mut stack = vec![desc];
            let mut seen = std::collections::HashSet::new();
            while let Some(cur) = stack.pop() {
                if cur == anc {
                    return Tri::Yes;
                }
                if seen.insert(cur.clone()) {
                    if let Some(ps) = parents.get(cur.as_str()) {
                        stack.extend(ps.iter().map(|p| p.to_string()));
                    }
                }
            }
            Tri::No
        }

        fn diff_files(&self, a: &str, b: &str) -> Option<Vec<String>> {
            let (Some(a), Some(b)) = (self.resolve(a), self.resolve(b)) else {
                return None;
            };
            if a == "1111111a" && b == "3333333c" {
                Some(vec!["src/lib.rs".to_string(), "README.md".to_string()])
            } else {
                Some(Vec::new())
            }
        }
    }

    fn git_verdict(report: &str) -> Verdict {
        let dir = bundle_with(&[]);
        verify_report_repo(report, dir.path(), Some(&StubRepo))
    }

    #[test]
    fn commit_claim_resolves_full_and_short() {
        let v = git_verdict("- CLAIM: landed [evidence: commit 3333333c]\n");
        assert_eq!(v.exit_code(), 0, "results: {:?}", v.results);
        let v = git_verdict("- CLAIM: landed [evidence: commit 3333333]\n");
        assert_eq!(v.exit_code(), 0, "results: {:?}", v.results);
    }

    #[test]
    fn commit_claim_missing_is_unverifiable() {
        let v = git_verdict("- CLAIM: landed [evidence: commit 9999999e]\n");
        assert_eq!(v.exit_code(), 3);
        assert_eq!(v.results[0].status, Status::Unverifiable);
    }

    #[test]
    fn git_claim_without_repo_is_unverifiable() {
        let dir = bundle_with(&[]);
        let v = verify_report("- CLAIM: landed [evidence: commit 3333333c]\n", dir.path());
        assert_eq!(v.exit_code(), 3);
    }

    #[test]
    fn ancestry_holds_and_fails() {
        let v = git_verdict("- CLAIM: based [evidence: ancestor 1111111a of 3333333c]\n");
        assert_eq!(v.exit_code(), 0, "results: {:?}", v.results);
        let v = git_verdict("- CLAIM: based [evidence: ancestor 3333333c of 1111111a]\n");
        assert_eq!(v.exit_code(), 1);
        assert_eq!(v.results[0].status, Status::Contradicted);
        let v = git_verdict("- CLAIM: based [evidence: ancestor 4444444d of 3333333c]\n");
        assert_eq!(v.exit_code(), 1);
    }

    #[test]
    fn diff_scope_count_and_touches() {
        let v = git_verdict("- CLAIM: scope [evidence: diff 1111111a..3333333c files == 2]\n");
        assert_eq!(v.exit_code(), 0, "results: {:?}", v.results);
        let v = git_verdict("- CLAIM: scope [evidence: diff 1111111a..3333333c files == 5]\n");
        assert_eq!(v.exit_code(), 1);
        let v = git_verdict("- CLAIM: scope [evidence: diff 1111111a..3333333c touches README.md]\n");
        assert_eq!(v.exit_code(), 0, "results: {:?}", v.results);
        let v = git_verdict("- CLAIM: scope [evidence: diff 1111111a..3333333c touches src/other.rs]\n");
        assert_eq!(v.exit_code(), 1);
        assert_eq!(v.results[0].status, Status::Contradicted);
    }

    #[test]
    fn malformed_git_markers_are_malformed() {
        let v = git_verdict("- CLAIM: x [evidence: commit not-hex!]\n");
        assert_eq!(v.results[0].status, Status::Malformed);
        let v = git_verdict("- CLAIM: x [evidence: diff 1111111a..3333333c touches ../escape]\n");
        assert_eq!(v.results[0].status, Status::Malformed);
        let v = git_verdict("- CLAIM: x [evidence: ancestor 1111111a 3333333c]\n");
        assert_eq!(v.results[0].status, Status::Malformed);
    }

    const RUN_LOG: &[u8] = b"running 15 tests\n\
                             test result: ok. 15 passed; 0 failed; 0 skipped\n\
                             process finished, exit code 0\n";

    #[test]
    fn run_facts_verify_against_the_cited_log() {
        let dir = bundle_with(&[("gate.log", RUN_LOG)]);
        let report = "- CLAIM: suite [evidence: run passed in gate.log == 15]\n\
                      - CLAIM: suite [evidence: run failed in gate.log == 0]\n\
                      - CLAIM: suite [evidence: run skipped in gate.log == 0]\n\
                      - CLAIM: suite [evidence: run exit in gate.log == 0]\n";
        let v = verify_report(report, dir.path());
        assert_eq!(v.exit_code(), 0, "results: {:?}", v.results);
        assert_eq!(v.claims_checked(), 4);
    }

    #[test]
    fn run_fact_that_disagrees_is_contradicted() {
        let dir = bundle_with(&[("gate.log", RUN_LOG)]);
        let v = verify_report("- CLAIM: suite [evidence: run passed in gate.log == 16]\n", dir.path());
        assert_eq!(v.exit_code(), 1);
        assert_eq!(v.results[0].status, Status::Contradicted);
    }

    #[test]
    fn run_fact_the_log_does_not_record_is_unverifiable() {
        let dir = bundle_with(&[("gate.log", b"15 passed and nothing else\n")]);
        let v = verify_report("- CLAIM: suite [evidence: run failed in gate.log == 0]\n", dir.path());
        assert_eq!(v.exit_code(), 3);
        assert_eq!(v.results[0].status, Status::Unverifiable);
        let v = verify_report("- CLAIM: suite [evidence: run passed in gone.log == 15]\n", dir.path());
        assert_eq!(v.exit_code(), 3);
    }

    #[test]
    fn last_tally_in_the_log_is_the_record() {
        let log = b"first attempt: 10 passed; 5 failed\n\
                    second attempt: 15 passed; 0 failed\n";
        let dir = bundle_with(&[("gate.log", log)]);
        let v = verify_report("- CLAIM: suite [evidence: run passed in gate.log == 15]\n", dir.path());
        assert_eq!(v.exit_code(), 0, "results: {:?}", v.results);
        let v = verify_report("- CLAIM: suite [evidence: run passed in gate.log == 10]\n", dir.path());
        assert_eq!(v.exit_code(), 1);
    }

    const PR_MERGED: &[u8] = br#"{"number": 568, "state": "closed", "merged": true, "title": "Publish the study"}"#;
    const PR_OPEN: &[u8] = br#"{"number": 12, "state": "open", "merged": false}"#;
    const CI_DONE: &[u8] = br#"{"head_sha": "2bb685b82cf8f723c57cfdd2da6946460e50fbfa", "status": "completed", "conclusion": "success"}"#;

    #[test]
    fn pr_state_checks_against_the_bundled_record() {
        let dir = bundle_with(&[
            ("records/pr-568.json", PR_MERGED),
            ("records/pr-12.json", PR_OPEN),
        ]);
        let v = verify_report("- CLAIM: landed [evidence: pr 568 state == merged]\n", dir.path());
        assert_eq!(v.exit_code(), 0, "results: {:?}", v.results);
        let v = verify_report("- CLAIM: pending [evidence: pr 12 state == open]\n", dir.path());
        assert_eq!(v.exit_code(), 0, "results: {:?}", v.results);
        let v = verify_report("- CLAIM: landed [evidence: pr 12 state == merged]\n", dir.path());
        assert_eq!(v.exit_code(), 1);
        assert_eq!(v.results[0].status, Status::Contradicted);
        let v = verify_report("- CLAIM: landed [evidence: pr 568 state == open]\n", dir.path());
        assert_eq!(v.exit_code(), 1);
    }

    #[test]
    fn pr_claim_without_a_record_is_unverifiable() {
        let dir = bundle_with(&[]);
        let v = verify_report("- CLAIM: landed [evidence: pr 568 state == merged]\n", dir.path());
        assert_eq!(v.exit_code(), 3);
        // A record whose number names a different pull request
        // does not cover the claim.
        let dir = bundle_with(&[("records/pr-568.json", PR_OPEN)]);
        let v = verify_report("- CLAIM: landed [evidence: pr 568 state == merged]\n", dir.path());
        assert_eq!(v.exit_code(), 3);
        assert_eq!(v.results[0].status, Status::Unverifiable);
    }

    #[test]
    fn unusable_record_leaves_the_claim_unverifiable() {
        let dir = bundle_with(&[("records/pr-568.json", b"not a record at all")]);
        let v = verify_report("- CLAIM: landed [evidence: pr 568 state == merged]\n", dir.path());
        assert_eq!(v.exit_code(), 3);
        assert_eq!(v.results[0].status, Status::Unverifiable);
    }

    #[test]
    fn ci_state_checks_against_the_bundled_record() {
        let dir = bundle_with(&[("records/ci-2bb685b8.json", CI_DONE)]);
        let v = verify_report("- CLAIM: gate [evidence: ci 2bb685b8 state == success]\n", dir.path());
        assert_eq!(v.exit_code(), 0, "results: {:?}", v.results);
        let v = verify_report("- CLAIM: gate [evidence: ci 2bb685b8 state == failure]\n", dir.path());
        assert_eq!(v.exit_code(), 1);
        // A record whose head does not match the claimed head does
        // not cover the claim, even filed under the claimed name.
        let dir = bundle_with(&[("records/ci-1111111a.json", CI_DONE)]);
        let v = verify_report("- CLAIM: gate [evidence: ci 1111111a state == success]\n", dir.path());
        assert_eq!(v.exit_code(), 3);
    }

    #[test]
    fn ci_claim_for_an_unfinished_run_is_unverifiable() {
        let rec = br#"{"head_sha": "2bb685b8", "status": "in_progress", "conclusion": null}"#;
        let dir = bundle_with(&[("records/ci-2bb685b8.json", rec)]);
        let v = verify_report("- CLAIM: gate [evidence: ci 2bb685b8 state == success]\n", dir.path());
        assert_eq!(v.exit_code(), 3);
        assert_eq!(v.results[0].status, Status::Unverifiable);
    }
}
