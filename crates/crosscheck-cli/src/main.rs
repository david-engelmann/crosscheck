//! The crosscheck command line.
//!
//! Usage: crosscheck verify REPORT --bundle DIR [--json] [--attest]
//!        crosscheck extract REPORT --bundle DIR [--json] [--attest]
//!
//! The extract mode audits a report that carries no evidence
//! markers, under the bundle-answers rule of DECISIONS.md D16.
//!
//! With --attest, the run prints the verdict as an in-toto
//! Statement instead of the usual rendering (DECISIONS.md
//! D19): the same verdict, in attestation shape, unsigned,
//! deterministic. The exit code is the verdict's either way.
//!
//! Exit codes are the verdict contract in DECISIONS.md D5: 0 verified,
//! 1 contradicted, 2 malformed, 3 unverifiable, 64 usage, 66 input
//! unreadable.
//!
//! Git claims are checked when the bundle carries a repository in
//! its `repo` directory. The adapter below is the product's only
//! process surface: it invokes the auditor's git with a fixed
//! subcommand set and argument vectors, never a shell, and the only
//! report-derived values it passes are commit identifiers the core
//! has already validated as hex (INV-1, INV-8).

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use crosscheck_core::extract::{ExtractedResult, Trust, verify_extracted};
use crosscheck_core::sha256::sha256_hex;
use crosscheck_core::{
    ClaimResult, EvidenceValue, RecordEvidence, RepoFacts, Status, Tri, verify_report_repo,
};

/// RepoFacts supplied by the auditor's git, run against the
/// repository bundled at `dir`.
struct GitRepo {
    dir: PathBuf,
}

impl GitRepo {
    /// The repository's HEAD commit, for the attestation's
    /// bundle pin; None when it cannot be read.
    fn head(&self) -> Option<String> {
        let (code, out) = self.run(&["rev-parse", "HEAD"])?;
        if code == 0 {
            Some(out.trim().to_string())
        } else {
            None
        }
    }

    /// Run git with the given arguments. Returns the exit code and
    /// stdout, or None when git could not be run at all.
    fn run(&self, args: &[&str]) -> Option<(i32, String)> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.dir)
            .args(args)
            .output()
            .ok()?;
        Some((
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stdout).into_owned(),
        ))
    }
}

impl RepoFacts for GitRepo {
    fn resolves_to_commit(&self, sha: &str) -> Tri {
        let spec = format!("{sha}^{{commit}}");
        match self.run(&["rev-parse", "--verify", "--quiet", &spec]) {
            Some((0, _)) => Tri::Yes,
            Some(_) => Tri::No,
            None => Tri::Unknown,
        }
    }

    fn is_ancestor(&self, anc: &str, desc: &str) -> Tri {
        if self.resolves_to_commit(anc) != Tri::Yes || self.resolves_to_commit(desc) != Tri::Yes {
            return Tri::Unknown;
        }
        match self.run(&["merge-base", "--is-ancestor", anc, desc]) {
            Some((0, _)) => Tri::Yes,
            Some((1, _)) => Tri::No,
            _ => Tri::Unknown,
        }
    }

    fn diff_files(&self, a: &str, b: &str) -> Option<Vec<String>> {
        let (code, out) = self.run(&["diff", "--name-only", a, b])?;
        if code != 0 {
            return None;
        }
        Some(
            out.lines()
                .filter(|l| !l.is_empty())
                .map(str::to_string)
                .collect(),
        )
    }
}

const LIMIT_LINE: &str = "Note: crosscheck proves that cited spans exist, hashes match, counts \
recompute, and cited commits resolve. It does not prove a span supports the sentence it is attached to.";

const EXTRACT_LINE: &str = "Note: extraction checks the claim classes the measurements support, \
each at its measured standing. The marker convention remains the complete path, and a claim \
the bundle cannot answer is unverifiable, not false.";

fn usage() -> ExitCode {
    eprintln!("usage: crosscheck verify REPORT --bundle DIR [--json] [--attest]");
    eprintln!("       crosscheck extract REPORT --bundle DIR [--json] [--attest]");
    ExitCode::from(64)
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// One verdict row for output. The extract mode's rows carry
/// a class and a trust standing; the verify mode's do not.
struct Row<'a> {
    line: usize,
    class: Option<&'a str>,
    trust: Option<&'a str>,
    status: Status,
    detail: &'a str,
}

fn print_json(rows: &[Row], count_key: &str, count: usize, exit: i32) {
    let mut out = String::from("{\"claims\":[");
    for (n, r) in rows.iter().enumerate() {
        if n > 0 {
            out.push(',');
        }
        out.push_str(&format!("{{\"line\":{}", r.line));
        if let Some(class) = r.class {
            out.push_str(&format!(",\"class\":\"{}\"", class));
        }
        if let Some(trust) = r.trust {
            out.push_str(&format!(",\"trust\":\"{}\"", trust));
        }
        out.push_str(&format!(
            ",\"status\":\"{}\",\"detail\":\"{}\"}}",
            r.status.label(),
            json_escape(r.detail)
        ));
    }
    out.push_str(&format!("],\"{count_key}\":{count},\"exit_code\":{exit}}}"));
    println!("{out}");
}

fn print_text(rows: &[Row]) {
    for r in rows {
        match (r.class, r.trust) {
            (Some(class), Some(trust)) => println!(
                "line {}: {} {} {} {}",
                r.line,
                class,
                trust,
                r.status.label(),
                r.detail
            ),
            _ => println!("line {}: {} {}", r.line, r.status.label(), r.detail),
        }
    }
}

const PREDICATE_TYPE: &str = "https://github.com/david-engelmann/crosscheck/attestation/verdict/v1";

/// One claim as the attestation carries it: the union of the
/// two modes' per-claim fields, each present only where the
/// mode has it.
struct StmtClaim<'a> {
    line: usize,
    spec: Option<&'a str>,
    class: Option<&'a str>,
    trust: Option<&'a str>,
    status: Status,
    detail: &'a str,
    evidence: Option<&'a RecordEvidence>,
}

fn evidence_json(ev: &RecordEvidence) -> String {
    let mut out = format!(
        "{{\"path\":\"{}\",\"sha256\":\"{}\",\"fields\":{{",
        json_escape(&ev.path),
        ev.sha256
    );
    for (n, (key, value)) in ev.fields.iter().enumerate() {
        if n > 0 {
            out.push(',');
        }
        out.push_str(&format!("\"{}\":", json_escape(key)));
        match value {
            EvidenceValue::Text(s) => out.push_str(&format!("\"{}\"", json_escape(s))),
            EvidenceValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            EvidenceValue::Number(s) => out.push_str(s),
        }
    }
    out.push_str("}}");
    out
}

fn status_counts(statuses: &[Status]) -> String {
    let count = |want: Status| statuses.iter().filter(|s| **s == want).count();
    format!(
        "{{\"verified\":{},\"contradicted\":{},\"unverifiable\":{},\"malformed\":{}}}",
        count(Status::Ok),
        count(Status::Contradicted),
        count(Status::Unverifiable),
        count(Status::Malformed)
    )
}

/// Render a verdict as an in-toto Statement (D19). Pure and
/// deterministic: the same inputs render byte-identically,
/// and the predicate carries no wall-clock time.
#[allow(clippy::too_many_arguments)]
fn build_statement(
    mode: &str,
    report_name: &str,
    report_sha256: &str,
    summary: &str,
    claims: &[StmtClaim],
    manifest_sha256: Option<&str>,
    git_head: Option<&str>,
    exit: i32,
) -> String {
    let mut out = String::from("{\"_type\":\"https://in-toto.io/Statement/v1\",\"subject\":[{");
    out.push_str(&format!(
        "\"name\":\"{}\",\"digest\":{{\"sha256\":\"{}\"}}",
        json_escape(report_name),
        report_sha256
    ));
    out.push_str(&format!(
        "}}],\"predicateType\":\"{PREDICATE_TYPE}\",\"predicate\":{{"
    ));
    out.push_str(&format!(
        "\"tool\":{{\"name\":\"crosscheck\",\"version\":\"{}\"}},\"mode\":\"{mode}\",\"exitCode\":{exit},\"summary\":{summary},\"claims\":[",
        env!("CARGO_PKG_VERSION")
    ));
    for (n, c) in claims.iter().enumerate() {
        if n > 0 {
            out.push(',');
        }
        out.push_str(&format!("{{\"line\":{}", c.line));
        if let Some(spec) = c.spec {
            out.push_str(&format!(",\"spec\":\"{}\"", json_escape(spec)));
        }
        if let Some(class) = c.class {
            out.push_str(&format!(",\"class\":\"{}\"", class));
        }
        if let Some(trust) = c.trust {
            out.push_str(&format!(",\"trust\":\"{}\"", trust));
        }
        out.push_str(&format!(
            ",\"status\":\"{}\",\"detail\":\"{}\"",
            c.status.label(),
            json_escape(c.detail)
        ));
        if let Some(ev) = c.evidence {
            out.push_str(&format!(",\"evidence\":{}", evidence_json(ev)));
        }
        out.push('}');
    }
    out.push(']');
    if manifest_sha256.is_some() || git_head.is_some() {
        out.push_str(",\"bundle\":{");
        let mut first = true;
        if let Some(m) = manifest_sha256 {
            out.push_str(&format!("\"manifestSha256\":\"{m}\""));
            first = false;
        }
        if let Some(h) = git_head {
            if !first {
                out.push(',');
            }
            out.push_str(&format!("\"gitHead\":\"{h}\""));
        }
        out.push('}');
    }
    out.push_str(&format!(",\"limit\":\"{}\"}}}}", json_escape(LIMIT_LINE)));
    out
}

fn verify_stmt_claims(results: &[ClaimResult]) -> Vec<StmtClaim<'_>> {
    results
        .iter()
        .map(|r| StmtClaim {
            line: r.line,
            spec: r.spec.as_deref(),
            class: None,
            trust: None,
            status: r.status,
            detail: &r.detail,
            evidence: r.evidence.as_ref(),
        })
        .collect()
}

fn extract_stmt_claims(results: &[ExtractedResult]) -> Vec<StmtClaim<'_>> {
    results
        .iter()
        .map(|r| StmtClaim {
            line: r.line,
            spec: None,
            class: Some(r.class),
            trust: Some(r.trust.label()),
            status: r.status,
            detail: &r.detail,
            evidence: r.evidence.as_ref(),
        })
        .collect()
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().map(String::as_str);
    if mode != Some("verify") && mode != Some("extract") {
        return usage();
    }
    let extract_mode = mode == Some("extract");
    let mut report: Option<PathBuf> = None;
    let mut bundle: Option<PathBuf> = None;
    let mut json = false;
    let mut attest = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--bundle" => {
                i += 1;
                if i >= args.len() {
                    return usage();
                }
                bundle = Some(PathBuf::from(&args[i]));
            }
            "--json" => json = true,
            "--attest" => attest = true,
            other if !other.starts_with("--") && report.is_none() => {
                report = Some(PathBuf::from(other));
            }
            _ => return usage(),
        }
        i += 1;
    }
    let (Some(report_path), Some(bundle_dir)) = (report, bundle) else {
        return usage();
    };

    let report_text = match std::fs::read_to_string(&report_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot read report {}: {e}", report_path.display());
            return ExitCode::from(66);
        }
    };
    if !Path::new(&bundle_dir).is_dir() {
        eprintln!("bundle {} is not a directory", bundle_dir.display());
        return ExitCode::from(66);
    }

    let repo_dir = bundle_dir.join("repo");
    let repo = if repo_dir.is_dir()
        && (repo_dir.join(".git").exists() || repo_dir.join("HEAD").exists())
    {
        Some(GitRepo { dir: repo_dir })
    } else {
        None
    };
    let repo_ref = repo.as_ref().map(|r| r as &dyn RepoFacts);
    // The attestation's pins on the evidence set: the bundle
    // manifest's digest when the bundle carries one, and the
    // bundled repository's head when it carries a repository.
    let manifest_sha = std::fs::read(bundle_dir.join("manifest.json"))
        .ok()
        .map(|bytes| sha256_hex(&bytes));
    let git_head = repo.as_ref().and_then(|r| r.head());
    if extract_mode {
        // When the report file lives inside the bundle, it is
        // excluded from the evidence scans: a claim never
        // verifies against the report itself.
        let report_rel = report_path
            .canonicalize()
            .ok()
            .zip(bundle_dir.canonicalize().ok())
            .and_then(|(r, b)| r.strip_prefix(&b).map(Path::to_path_buf).ok());
        let extraction =
            verify_extracted(&report_text, &bundle_dir, repo_ref, report_rel.as_deref());
        if attest {
            let by_trust = |want: Trust| {
                extraction
                    .results
                    .iter()
                    .filter(|r| r.trust == want)
                    .map(|r| r.status)
                    .collect::<Vec<_>>()
            };
            let summary = format!(
                "{{\"trusted\":{},\"provisional\":{}}}",
                status_counts(&by_trust(Trust::Trusted)),
                status_counts(&by_trust(Trust::Provisional))
            );
            println!(
                "{}",
                build_statement(
                    "extract",
                    &report_path.display().to_string(),
                    &sha256_hex(report_text.as_bytes()),
                    &summary,
                    &extract_stmt_claims(&extraction.results),
                    manifest_sha.as_deref(),
                    git_head.as_deref(),
                    extraction.exit_code(),
                )
            );
            return ExitCode::from(extraction.exit_code() as u8);
        }
        let rows: Vec<Row> = extraction
            .results
            .iter()
            .map(|r| Row {
                line: r.line,
                class: Some(r.class),
                trust: Some(r.trust.label()),
                status: r.status,
                detail: &r.detail,
            })
            .collect();
        if json {
            print_json(
                &rows,
                "claims_extracted",
                extraction.claims_extracted(),
                extraction.exit_code(),
            );
        } else {
            print_text(&rows);
            let bad = extraction
                .results
                .iter()
                .filter(|r| r.trust == Trust::Trusted && r.status != Status::Ok)
                .count();
            println!(
                "claims extracted: {}, trusted not verified: {}, exit {}",
                extraction.claims_extracted(),
                bad,
                extraction.exit_code()
            );
            println!("{EXTRACT_LINE}");
            println!("{LIMIT_LINE}");
        }
        return ExitCode::from(extraction.exit_code() as u8);
    }
    let verdict = verify_report_repo(&report_text, &bundle_dir, repo_ref);
    if attest {
        let statuses: Vec<Status> = verdict.results.iter().map(|r| r.status).collect();
        println!(
            "{}",
            build_statement(
                "verify",
                &report_path.display().to_string(),
                &sha256_hex(report_text.as_bytes()),
                &status_counts(&statuses),
                &verify_stmt_claims(&verdict.results),
                manifest_sha.as_deref(),
                git_head.as_deref(),
                verdict.exit_code(),
            )
        );
        return ExitCode::from(verdict.exit_code() as u8);
    }
    let rows: Vec<Row> = verdict
        .results
        .iter()
        .map(|r| Row {
            line: r.line,
            class: None,
            trust: None,
            status: r.status,
            detail: &r.detail,
        })
        .collect();
    if json {
        print_json(
            &rows,
            "claims_checked",
            verdict.claims_checked(),
            verdict.exit_code(),
        );
    } else {
        print_text(&rows);
        let bad = verdict
            .results
            .iter()
            .filter(|r| r.status != Status::Ok)
            .count();
        println!(
            "claims checked: {}, not verified: {}, exit {}",
            verdict.claims_checked(),
            bad,
            verdict.exit_code()
        );
        println!("{LIMIT_LINE}");
    }
    ExitCode::from(verdict.exit_code() as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_claims() -> (Vec<StmtClaim<'static>>, RecordEvidence) {
        let ev = RecordEvidence {
            path: "records/pr-452.json".to_string(),
            sha256: "a".repeat(64),
            fields: vec![
                (
                    "number".to_string(),
                    EvidenceValue::Number("452".to_string()),
                ),
                (
                    "state".to_string(),
                    EvidenceValue::Text("closed".to_string()),
                ),
                ("merged".to_string(), EvidenceValue::Bool(true)),
            ],
        };
        let claims = vec![
            StmtClaim {
                line: 3,
                spec: Some("pr 452 state == merged"),
                class: None,
                trust: None,
                status: Status::Ok,
                detail: "record shows the pull request merged",
                evidence: None,
            },
            StmtClaim {
                line: 5,
                spec: None,
                class: Some("commit"),
                trust: Some("trusted"),
                status: Status::Unverifiable,
                detail: "does not resolve in the bundled repository",
                evidence: None,
            },
        ];
        (claims, ev)
    }

    #[test]
    fn the_same_verdict_exports_byte_identically() {
        let (claims, _) = sample_claims();
        let summary = status_counts(&[Status::Ok, Status::Unverifiable]);
        let first = build_statement(
            "verify",
            "report.md",
            &"b".repeat(64),
            &summary,
            &claims,
            Some(&"c".repeat(64)),
            Some("abc1234"),
            3,
        );
        let second = build_statement(
            "verify",
            "report.md",
            &"b".repeat(64),
            &summary,
            &claims,
            Some(&"c".repeat(64)),
            Some("abc1234"),
            3,
        );
        assert_eq!(first, second);
    }

    #[test]
    fn the_statement_carries_the_ruled_shape() {
        let (mut claims, ev) = sample_claims();
        claims[0].evidence = Some(&ev);
        let summary = status_counts(&[Status::Ok, Status::Unverifiable]);
        let out = build_statement(
            "verify",
            "report.md",
            &"b".repeat(64),
            &summary,
            &claims,
            None,
            None,
            3,
        );
        assert!(out.contains("\"_type\":\"https://in-toto.io/Statement/v1\""));
        assert!(out.contains(&format!("\"predicateType\":\"{PREDICATE_TYPE}\"")));
        assert!(out.contains("\"exitCode\":3"));
        assert!(out.contains("\"verified\":1"));
        assert!(out.contains("\"unverifiable\":1"));
        assert!(out.contains("\"spec\":\"pr 452 state == merged\""));
        assert!(out.contains("\"path\":\"records/pr-452.json\""));
        assert!(out.contains("\"number\":452"));
        assert!(out.contains("\"merged\":true"));
        // The ruled absence: no wall-clock time anywhere.
        assert!(!out.contains("timestamp"));
        assert!(!out.contains("\"time\""));
        // No bundle block when the bundle pins nothing.
        assert!(!out.contains("\"bundle\""));
    }
}
