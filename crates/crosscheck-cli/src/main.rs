//! The crosscheck command line.
//!
//! Usage: crosscheck verify REPORT --bundle DIR [--json]
//!        crosscheck extract REPORT --bundle DIR [--json]
//!
//! The extract mode audits a report that carries no evidence
//! markers, under the bundle-answers rule of DECISIONS.md D16.
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

use crosscheck_core::extract::{Trust, verify_extracted};
use crosscheck_core::{RepoFacts, Status, Tri, verify_report_repo};

/// RepoFacts supplied by the auditor's git, run against the
/// repository bundled at `dir`.
struct GitRepo {
    dir: PathBuf,
}

impl GitRepo {
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
    eprintln!("usage: crosscheck verify REPORT --bundle DIR [--json]");
    eprintln!("       crosscheck extract REPORT --bundle DIR [--json]");
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
