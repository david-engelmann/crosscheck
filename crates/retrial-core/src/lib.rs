//! Retrial core: deterministic verification of agent work reports
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
    Err(format!("unrecognised evidence marker: {spec}"))
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

fn check(spec: &Spec, bundle: &Path) -> (Status, String) {
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
    }
}

/// Verify every claim in `report` against the bundle at `bundle`.
/// Claim lines are lines beginning with `- CLAIM:`. A claim line
/// without a parseable evidence marker is malformed, never skipped.
pub fn verify_report(report: &str, bundle: &Path) -> Verdict {
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
                    let (status, detail) = check(&spec, bundle);
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
                p.push(format!("retrial-core-test-{}-{n}", std::process::id()));
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
}
