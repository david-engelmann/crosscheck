//! The retrial command line.
//!
//! Usage: retrial verify REPORT --bundle DIR [--json]
//!
//! Exit codes are the verdict contract in DECISIONS.md D5: 0 verified,
//! 1 contradicted, 2 malformed, 3 unverifiable, 64 usage, 66 input
//! unreadable.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use retrial_core::{Status, verify_report};

const LIMIT_LINE: &str = "Note: retrial proves that cited spans exist, hashes match, and counts \
recompute. It does not prove a span supports the sentence it is attached to.";

fn usage() -> ExitCode {
    eprintln!("usage: retrial verify REPORT --bundle DIR [--json]");
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

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) != Some("verify") {
        return usage();
    }
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

    let verdict = verify_report(&report_text, &bundle_dir);
    if json {
        let mut out = String::from("{\"claims\":[");
        for (n, r) in verdict.results.iter().enumerate() {
            if n > 0 {
                out.push(',');
            }
            out.push_str(&format!(
                "{{\"line\":{},\"status\":\"{}\",\"detail\":\"{}\"}}",
                r.line,
                r.status.label(),
                json_escape(&r.detail)
            ));
        }
        out.push_str(&format!(
            "],\"claims_checked\":{},\"exit_code\":{}}}",
            verdict.claims_checked(),
            verdict.exit_code()
        ));
        println!("{out}");
    } else {
        for r in &verdict.results {
            println!("line {}: {} {}", r.line, r.status.label(), r.detail);
        }
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
