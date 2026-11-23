//! Extraction mode: audit a report that carries no evidence
//! markers, per DECISIONS.md D16, with the trust standings
//! amended by D17 and the checking semantics stated in D18.
//!
//! The patterns are a hand-written port of the measured
//! extractor. The tally and count patterns are pattern set v2
//! (banked as docs/m3/extract_v2.py), whose rates D17 ratified:
//! tallies trusted at 0.3 percent, counts provisional at 2.8
//! percent. The remaining patterns are unchanged between v1
//! and v2. Sizes stay out, and the quote pattern stays
//! dropped. The measurement's first-file target is ported for
//! the count class only, because D18 makes it part of one
//! verdict (a line count against a named file).
//!
//! Trust standings after D17: explicit PR references, short
//! commits, full SHAs, content hashes, and run tallies are
//! trusted. Bare PR references and counts are provisional and
//! never move the exit code alone. The exit code follows D5
//! over trusted claims only.
//!
//! Checking follows the D16 rule: the bundle answers what it
//! can, in the most specific way available, and the rest is
//! unverifiable. A commit or full SHA checks against the
//! bundled repository. A PR reference checks against the
//! bundled host record of that number. A content hash checks
//! against every file the bundle carries; a match anywhere
//! verifies it, and no match is unverifiable, never
//! contradicted, because the extractor cannot know which file
//! the author meant. A tally checks against every log the
//! bundle carries, meaning its text files outside the
//! repository subtree: the four run facts under the M3
//! last-tally rule, and totals, fractions, and of-forms
//! against the comparable shape each log records last (D18). A count
//! checks only when it counts lines in a file the report
//! names and the bundle carries; every other count is
//! unverifiable, because the bundle cannot answer what was
//! counted. Every verified or contradicted verdict names the
//! evidence it matched.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::{
    PR_RECORD_KEYS, RecordEvidence, RepoFacts, RunFact, Status, Tri, last_tally, parse_pr_record,
    read_cited, record_evidence, sha256,
};

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

/// A tally shape that is not one of the four run facts: a
/// tests/checks/subtests total, a fraction, or an of-form.
/// Values are normalized digit strings (no commas, no leading
/// zeros), so report-side and log-side shapes compare equal
/// when they record the same numbers.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Shape {
    Total {
        fam: &'static str,
        value: String,
    },
    Frac {
        a: String,
        b: String,
        fam: &'static str,
    },
    Of {
        a: String,
        b: String,
    },
}

impl Shape {
    /// The dedupe and display signature.
    fn signature(&self) -> String {
        match self {
            Shape::Total { fam, value } => format!("total:{value}:{fam}"),
            Shape::Frac { a, b, fam } => format!("frac:{a}:{b}:{fam}"),
            Shape::Of { a, b } => format!("of:{a}:{b}"),
        }
    }

    /// The recorded answer this shape carries, when `other`
    /// records the same fact: the same family for a total, the
    /// same denominator and family for a fraction, the same
    /// whole for an of-form. None when the two shapes record
    /// different facts and cannot disagree.
    fn comparable_answer(&self, other: &Shape) -> Option<String> {
        match (self, other) {
            (Shape::Total { fam: f1, .. }, Shape::Total { fam: f2, value }) if f1 == f2 => {
                Some(value.clone())
            }
            (Shape::Frac { b: b1, fam: f1, .. }, Shape::Frac { a, b: b2, fam: f2 })
                if b1 == b2 && f1 == f2 =>
            {
                Some(a.clone())
            }
            (Shape::Of { b: b1, .. }, Shape::Of { a, b: b2 }) if b1 == b2 => Some(a.clone()),
            _ => None,
        }
    }

    /// This shape's own answer, in the same terms.
    fn answer(&self) -> &str {
        match self {
            Shape::Total { value, .. } => value,
            Shape::Frac { a, .. } => a,
            Shape::Of { a, .. } => a,
        }
    }

    /// A short human form for verdict details.
    fn describe(&self) -> String {
        match self {
            Shape::Total { fam, value } => format!("{value} {fam}s"),
            Shape::Frac { a, b, fam } => format!("{a}/{b} {fam}s"),
            Shape::Of { a, b } => format!("{a} of {b}"),
        }
    }
}

/// A count claim: a number, the noun it counts, and the first
/// file the report's line names, if any.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CountClaim {
    digits: String,
    noun: String,
    target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XKind {
    /// The PR number's digits exactly as written. The record
    /// lookup normalizes them; keeping the literal form keeps
    /// numbers of any length extractable, as in the measured
    /// extractor.
    Pr(String),
    Commit(String),
    GitSha(String),
    Hash256(String),
    /// A run fact and its claimed value as normalized digits.
    /// String form keeps very long numbers from vanishing at
    /// a parse bound the measured extractor does not have.
    TallyFact(RunFact, String),
    TallyShape(Shape),
    Count(CountClaim),
}

/// One extracted claim, before checking.
#[derive(Debug, Clone)]
pub struct Extracted {
    pub line: usize,
    pub class: &'static str,
    pub trust: Trust,
    /// The claim as the measured extractor states it (its
    /// dedupe identity within the line).
    pub value: String,
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
    /// The bundled record the verdict rested on, when the
    /// claim is a pull request reference and the record was
    /// read; carried for the attestation export.
    pub evidence: Option<RecordEvidence>,
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

/// The fused-token guard, pattern set v2: the character before
/// a number must not be a letter, digit, dot, underscore, hash,
/// hyphen, tilde, or equals sign.
fn guard_ok(line: &[u8], start: usize) -> bool {
    start == 0
        || !(line[start - 1].is_ascii_alphanumeric()
            || matches!(line[start - 1], b'.' | b'_' | b'#' | b'-' | b'~' | b'='))
}

/// Normalize a digit token as written (commas admitted) to the
/// digit string it names: commas removed, leading zeros
/// removed, zero itself kept.
fn normalize_digits(raw: &str) -> String {
    let digits: String = raw.chars().filter(|c| *c != ',').collect();
    let trimmed = digits.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Scan a `\d[\d,]*` token at `i`. Returns the normalized
/// digits and the end index (one past the last digit or comma
/// consumed). Any length is accepted; comparison happens on
/// the digit string.
fn scan_number(line: &[u8], i: usize) -> Option<(String, usize)> {
    if i >= line.len() || !line[i].is_ascii_digit() {
        return None;
    }
    let mut j = i;
    while j < line.len() && (line[j].is_ascii_digit() || line[j] == b',') {
        j += 1;
    }
    let raw = std::str::from_utf8(&line[i..j]).ok()?;
    Some((normalize_digits(raw), j))
}

/// Scan a run of plain digits at `i` (the `\d+` of the PR,
/// fraction, and exit patterns, which admit no commas).
/// Returns the digits exactly as written and the end index.
fn scan_digits(line: &[u8], i: usize) -> Option<(String, usize)> {
    if i >= line.len() || !line[i].is_ascii_digit() {
        return None;
    }
    let mut j = i;
    while j < line.len() && line[j].is_ascii_digit() {
        j += 1;
    }
    Some((line[i..j].iter().map(|c| *c as char).collect(), j))
}

/// Skip `\s*` (spaces and tabs) at `i`; returns the index one
/// past the last space or tab, which may equal `i`.
fn skip_ws_any(line: &[u8], i: usize) -> usize {
    let mut j = i;
    while j < line.len() && (line[j] == b' ' || line[j] == b'\t') {
        j += 1;
    }
    j
}

/// Skip `\s+` (spaces and tabs) at `i`; returns the new index,
/// or None when there is no whitespace.
fn skip_ws(line: &[u8], i: usize) -> Option<usize> {
    let j = skip_ws_any(line, i);
    if j > i { Some(j) } else { None }
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

/// The token immediately before `start`, in the measured
/// extractor's terms: the last whitespace-separated token of
/// the preceding text, stripped of backtick, star, open
/// paren, pipe, and quote marks.
fn prev_token(line: &str, start: usize) -> &str {
    let before = line[..start].trim_end();
    if before.is_empty() {
        return "";
    }
    before
        .split_whitespace()
        .last()
        .unwrap_or("")
        .trim_matches(|c| matches!(c, '`' | '*' | '(' | '|' | '"'))
}

/// The tally guard's second clause: the token immediately
/// before the number is a commit-like hex token or a single
/// letter.
fn tally_blocked(line: &str, start: usize) -> bool {
    let token = prev_token(line, start);
    let bytes = token.as_bytes();
    if bytes.len() == 1 && bytes[0].is_ascii_alphabetic() {
        return true;
    }
    (7..=40).contains(&bytes.len())
        && bytes.iter().all(|b| is_lhex(*b))
        && bytes.iter().any(|b| b.is_ascii_alphabetic())
}

/// The of-form's series suppression (pattern set v2): the
/// word before the number names a document series or a rank,
/// not a run.
fn of_blocked(line: &str, start: usize) -> bool {
    let token = prev_token(line, start).to_lowercase();
    matches!(
        token.as_str(),
        "review" | "rank" | "checkpoint" | "section" | "part" | "step" | "phase"
    )
}

const COMMIT_WORDS: [&str; 7] = [
    "commit", "head", "base", "sha", "merge", "reviewed", "branch",
];

const FILE_EXTS: [&str; 11] = [
    "md", "py", "rs", "toml", "json", "jsonl", "txt", "log", "sh", "yaml", "yml",
];

fn is_file_char(b: u8) -> bool {
    is_word(b) || matches!(b, b'.' | b'/' | b'~' | b'-')
}

/// The first file-like token on the line, in the measured
/// extractor's terms (its FILE_RE): a run of word, dot, slash,
/// tilde, and hyphen characters carrying a known extension.
/// Ported for the count class, whose line-count verdict names
/// its target through it (D18).
fn first_file(line: &str) -> Option<String> {
    let bytes = line.as_bytes();
    let mut start = 0;
    while start < bytes.len() {
        if !is_file_char(bytes[start]) {
            start += 1;
            continue;
        }
        let mut end = start;
        while end < bytes.len() && is_file_char(bytes[end]) {
            end += 1;
        }
        // The pattern's run is greedy and backtracks from the
        // last dot: try each dot in the run, last first.
        let mut dots: Vec<usize> = (start..end).filter(|&p| bytes[p] == b'.').collect();
        dots.reverse();
        for d in dots {
            let rest = &line[d + 1..end];
            for ext in FILE_EXTS {
                if let Some(after) = rest.strip_prefix(ext) {
                    // The regex's \b after the extension: the
                    // next character must not be a word char.
                    if after.is_empty() || !is_word(after.as_bytes()[0]) {
                        return Some(line[start..d + 1 + ext.len()].to_string());
                    }
                }
            }
        }
        start = end;
    }
    None
}

/// Left-to-right scan over the number tokens of a line, the
/// skeleton the guarded tally and count passes share. At each
/// digit whose preceding character passes the fused-token
/// guard, `on_match` receives the number's start, its
/// normalized digits, and its end; it returns the index to
/// resume from when it consumes the match, or None to advance
/// by one character.
fn scan_numbers(line: &[u8], on_match: &mut dyn FnMut(usize, &str, usize) -> Option<usize>) {
    let mut i = 0;
    while i < line.len() {
        if line[i].is_ascii_digit() && guard_ok(line, i) {
            if let Some((digits, end)) = scan_number(line, i) {
                if let Some(resume) = on_match(i, &digits, end) {
                    i = resume;
                    continue;
                }
            }
        }
        i += 1;
    }
}

fn extract_line(
    line_no: usize,
    line: &str,
    out: &mut Vec<Extracted>,
    seen: &mut HashSet<(usize, &'static str, String)>,
) {
    let bytes = line.as_bytes();
    let mut add =
        |cls: &'static str, trust: Trust, kind: XKind, value: String, out: &mut Vec<Extracted>| {
            if seen.insert((line_no, cls, value.clone())) {
                out.push(Extracted {
                    line: line_no,
                    class: cls,
                    trust,
                    value,
                    kind,
                });
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

    // Run-fact tallies: one left-to-right pass per word, as the
    // measured extractor ran one finditer pass per pattern.
    for (word, fact) in [
        ("passed", RunFact::Passed),
        ("failed", RunFact::Failed),
        ("skipped", RunFact::Skipped),
    ] {
        scan_numbers(bytes, &mut |start, digits, end| {
            let w = skip_ws(bytes, end)?;
            if !word_at(bytes, w, word) {
                return None;
            }
            if !tally_blocked(line, start) {
                add(
                    "tally",
                    Trust::Trusted,
                    XKind::TallyFact(fact, digits.to_string()),
                    format!("{} {}", fact.word(), &line[start..w + word.len()]),
                    out,
                );
            }
            Some(w + word.len())
        });
    }
    // Totals: N tests / checks / subtests.
    scan_numbers(bytes, &mut |start, digits, end| {
        let w = skip_ws(bytes, end)?;
        for (word, fam) in [
            ("subtests", "subtest"),
            ("tests", "test"),
            ("test", "test"),
            ("checks", "check"),
            ("check", "check"),
        ] {
            if word_at(bytes, w, word) {
                if !tally_blocked(line, start) {
                    add(
                        "tally",
                        Trust::Trusted,
                        XKind::TallyShape(Shape::Total {
                            fam,
                            value: digits.to_string(),
                        }),
                        format!("tests {}", &line[start..w + word.len()]),
                        out,
                    );
                }
                return Some(w + word.len());
            }
        }
        None
    });
    // Exit tallies: "exit 0" or "exit code 0", unguarded in the
    // measured set.
    {
        let mut i = 0;
        while i + 4 <= bytes.len() {
            if &bytes[i..i + 4] == b"exit" {
                let mut j = i + 4;
                if bytes[j..].starts_with(b" code") {
                    j += 5;
                }
                if let Some(d) = skip_ws(bytes, j) {
                    if let Some((digits, end)) = scan_digits(bytes, d) {
                        if end >= bytes.len() || !is_word(bytes[end]) {
                            add(
                                "tally",
                                Trust::Trusted,
                                XKind::TallyFact(RunFact::Exit, normalize_digits(&digits)),
                                format!("exit {}", &line[i..end]),
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
    // Fractions: A/B gates, checks, tests, or calls. Plain
    // digits only, no commas, no guard, as measured.
    {
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i].is_ascii_digit() {
                if let Some((a, e1)) = scan_digits(bytes, i) {
                    let j = skip_ws_any(bytes, e1);
                    if j < bytes.len() && bytes[j] == b'/' {
                        let k = skip_ws_any(bytes, j + 1);
                        if let Some((b, e2)) = scan_digits(bytes, k) {
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
                                            Trust::Trusted,
                                            XKind::TallyShape(Shape::Frac {
                                                a: normalize_digits(&a),
                                                b: normalize_digits(&b),
                                                fam,
                                            }),
                                            line[i..w + word.len()].to_string(),
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
    // Of-forms: A of B, with the v2 fused-token guard and the
    // series suppression.
    scan_numbers(bytes, &mut |start, digits, end| {
        let w = skip_ws(bytes, end)?;
        if !word_at(bytes, w, "of") {
            return None;
        }
        let d = skip_ws(bytes, w + 2)?;
        let (b, e2) = scan_number(bytes, d)?;
        if e2 < bytes.len() && is_word(bytes[e2]) {
            return None;
        }
        if !of_blocked(line, start) {
            add(
                "tally",
                Trust::Trusted,
                XKind::TallyShape(Shape::Of {
                    a: digits.to_string(),
                    b,
                }),
                line[start..e2].to_string(),
                out,
            );
        }
        Some(e2)
    });

    // PR references: explicit shapes first, then bare numbers
    // not already claimed explicitly on this line. Explicit
    // forms are trusted; the bare form is provisional (D17).
    // Suppression compares the digits as written, as the
    // measured extractor compares them.
    let mut explicit: HashSet<String> = HashSet::new();
    {
        let mut i = 0;
        while i + 1 < bytes.len() {
            if &bytes[i..i + 2] == b"PR" && (i == 0 || !is_word(bytes[i - 1])) {
                let h = skip_ws_any(bytes, i + 2);
                if h < bytes.len() && bytes[h] == b'#' {
                    if let Some((digits, _)) = scan_digits(bytes, h + 1) {
                        explicit.insert(digits.clone());
                        add(
                            "pr",
                            Trust::Trusted,
                            XKind::Pr(digits.clone()),
                            format!("PR {digits}"),
                            out,
                        );
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
                if let Some((digits, _)) = scan_digits(bytes, i + 5) {
                    explicit.insert(digits.clone());
                    add(
                        "pr",
                        Trust::Trusted,
                        XKind::Pr(digits.clone()),
                        format!("PR {digits}"),
                        out,
                    );
                }
            }
            i += 1;
        }
    }
    for i in 0..bytes.len() {
        if bytes[i] != b'#' {
            continue;
        }
        let Some((digits, _)) = scan_digits(bytes, i + 1) else {
            continue;
        };
        // Repository form: owner/repo#N, where both sides of
        // the slash carry at least one name character. The
        // measured pattern admits no slash inside either side,
        // so the last slash of the run decides.
        let mut s = i;
        while s > 0
            && (bytes[s - 1].is_ascii_alphanumeric()
                || matches!(bytes[s - 1], b'.' | b'_' | b'-' | b'/'))
        {
            s -= 1;
        }
        let run = &line[s..i];
        if let Some(slash) = run.rfind('/') {
            let left = &run[..slash];
            let left_ok = !left.is_empty()
                && left
                    .rsplit('/')
                    .next()
                    .map(|seg| !seg.is_empty())
                    .unwrap_or(false);
            let right_ok = slash + 1 < run.len();
            if left_ok && right_ok {
                explicit.insert(digits.clone());
                add(
                    "pr",
                    Trust::Trusted,
                    XKind::Pr(digits.clone()),
                    format!("PR {digits}"),
                    out,
                );
            }
        }
    }
    for i in 0..bytes.len() {
        if bytes[i] != b'#' {
            continue;
        }
        if i > 0 && (is_word(bytes[i - 1]) || bytes[i - 1] == b'#') {
            continue;
        }
        let Some((digits, end)) = scan_digits(bytes, i + 1) else {
            continue;
        };
        if digits.len() < 2 {
            continue;
        }
        if end < bytes.len() && is_word(bytes[end]) {
            continue;
        }
        if explicit.contains(&digits) {
            continue;
        }
        add(
            "pr",
            Trust::Provisional,
            XKind::Pr(digits.clone()),
            format!("PR {digits}"),
            out,
        );
    }

    // Counts: a number and a count noun, guarded, provisional
    // (D17). The claim carries the first file the line names,
    // for the line-count verdict (D18).
    let target = first_file(line);
    scan_numbers(bytes, &mut |start, digits, end| {
        let w = skip_ws(bytes, end)?;
        for noun in [
            "files",
            "file",
            "findings",
            "finding",
            "rounds",
            "round",
            "repos",
            "repo",
            "crates",
            "crate",
            "modules",
            "module",
            "headings",
            "heading",
            "cases",
            "case",
            "commits",
            "commit",
            "lines",
            "line",
            "errors",
            "error",
            "warnings",
            "warning",
            "pairs",
            "pair",
            "instances",
            "instance",
            "defects",
            "defect",
            "suites",
            "suite",
            "families",
            "arms",
            "prompts",
            "documents",
            "document",
            "entries",
            "items",
            "comments",
            "comment",
            "nits",
            "violations",
            "violation",
        ] {
            if word_at(bytes, w, noun) {
                add(
                    "count",
                    Trust::Provisional,
                    XKind::Count(CountClaim {
                        digits: digits.to_string(),
                        noun: noun.to_string(),
                        target: target.clone(),
                    }),
                    line[start..w + noun.len()].to_string(),
                    out,
                );
                return Some(w + noun.len());
            }
        }
        None
    });
}

/// Extract every claim the ratified pattern set finds in
/// `report`, in line order; within a line, claims come out in
/// pattern-pass order (hex runs in position order), as the
/// measured extractor emitted them. The port is byte-oriented:
/// where the measured extractor's patterns are Unicode-aware,
/// the port reads ASCII, the alphabet of the fleet's reports
/// (D18).
/// Pure: reads nothing, checks nothing.
pub fn extract_claims(report: &str) -> Vec<Extracted> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    for (idx, line) in report.lines().enumerate() {
        extract_line(idx + 1, line, &mut out, &mut seen);
    }
    out
}

/// Every file the bundle carries, as bundle-relative paths in
/// sorted order. The walk is the evidence boundary, so it is
/// strict (D18): it never follows a symlink (a symlinked entry
/// is skipped whole, so evidence cannot arrive from outside
/// the bundle and a cyclic link cannot loop the walk), it
/// does not descend into any `.git` directory (repository
/// internals are not evidence files), and it bounds its depth
/// at 32 as a backstop. The bundled repository's working tree
/// is bundle evidence like any other file.
fn bundle_files(bundle: &Path) -> Vec<PathBuf> {
    const MAX_DEPTH: usize = 32;
    let mut files = Vec::new();
    let mut stack: Vec<(PathBuf, usize)> = vec![(bundle.to_path_buf(), 0)];
    while let Some((dir, depth)) = stack.pop() {
        if depth > MAX_DEPTH {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if file_type.is_symlink() {
                continue;
            }
            let path = entry.path();
            if file_type.is_dir() {
                if entry.file_name().to_string_lossy() != ".git" {
                    stack.push((path, depth + 1));
                }
            } else if file_type.is_file() {
                if let Ok(rel) = path.strip_prefix(bundle) {
                    files.push(rel.to_path_buf());
                }
            }
        }
    }
    files.sort();
    files
}

/// Does this file read as text? The log scan applies a plain
/// sniff: no NUL byte in the first 8 KiB. Binary files are
/// still hash evidence; they are not logs (D18).
fn looks_like_text(bytes: &[u8]) -> bool {
    !bytes[..bytes.len().min(8192)].contains(&0)
}

/// Check every extracted claim in `report` against the bundle at
/// `bundle`. `report_rel` is the report's own path relative to
/// the bundle when the report file lives inside it; that file is
/// excluded from evidence scans so a claim never verifies
/// against the report itself (D18).
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
    // value, the text of every file that could be a log, and
    // the tally shapes each log records, in order.
    let mut hash_match: HashMap<String, String> = HashMap::new();
    let mut log_texts: Vec<(String, String)> = Vec::new();
    let need_hash = claims.iter().any(|c| matches!(c.kind, XKind::Hash256(_)));
    let need_logs = claims
        .iter()
        .any(|c| matches!(c.kind, XKind::TallyFact(..) | XKind::TallyShape(_)));
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
            // The log corpus is the bundle's text files outside
            // the repository subtree: the repository is bundled
            // as git evidence, and its documents are not logs of
            // the work under audit (D18). Hash evidence, by
            // contrast, comes from every file.
            let in_repo = rel
                .components()
                .next()
                .map(|c| c.as_os_str().to_string_lossy() == "repo")
                .unwrap_or(false);
            if need_logs && !in_repo && looks_like_text(&bytes) {
                log_texts.push((name, String::from_utf8_lossy(&bytes).into_owned()));
            }
        }
    }
    let mut log_shapes: Vec<(String, Vec<Shape>)> = Vec::new();
    if claims
        .iter()
        .any(|c| matches!(c.kind, XKind::TallyShape(_)))
    {
        for (name, text) in &log_texts {
            let shapes: Vec<Shape> = extract_claims(text)
                .into_iter()
                .filter_map(|c| match c.kind {
                    XKind::TallyShape(s) => Some(s),
                    _ => None,
                })
                .collect();
            log_shapes.push((name.clone(), shapes));
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
            XKind::Pr(digits) => {
                let number = normalize_digits(digits);
                let cited = format!("records/pr-{number}.json");
                match read_cited(bundle, &cited) {
                    Ok(bytes) => match parse_pr_record(&bytes) {
                        Some((n, state, merged)) if n.to_string() == number => (
                            Status::Ok,
                            format!("bundled record {cited} shows state {state}, merged {merged}"),
                        ),
                        Some((n, _, _)) => (
                            Status::Unverifiable,
                            format!("bundled record {cited} describes PR {n}"),
                        ),
                        None => (
                            Status::Unverifiable,
                            format!("bundled record {cited} lacks the fields this claim needs"),
                        ),
                    },
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
                        Some(v) if v.to_string() == *want => {
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
            XKind::TallyShape(shape) => {
                // Each log's record of this fact is the last
                // comparable shape it carries (D18), the same
                // append discipline as the last-tally rule.
                let mut agreeing: Option<&str> = None;
                let mut disagreeing: Option<(&str, String)> = None;
                for (name, shapes) in &log_shapes {
                    let recorded = shapes.iter().rev().find_map(|s| shape.comparable_answer(s));
                    match recorded {
                        Some(ans) if ans == shape.answer() => {
                            agreeing = Some(name.as_str());
                            break;
                        }
                        Some(ans) => {
                            if disagreeing.is_none() {
                                disagreeing = Some((name.as_str(), ans));
                            }
                        }
                        None => {}
                    }
                }
                match (agreeing, disagreeing) {
                    (Some(name), _) => (
                        Status::Ok,
                        format!("bundle log {name} records {}", shape.describe()),
                    ),
                    (None, Some((name, actual))) => (
                        Status::Contradicted,
                        format!(
                            "bundle log {name} records a different tally (its last comparable record answers {actual}), the report says {}",
                            shape.describe()
                        ),
                    ),
                    (None, None) => (
                        Status::Unverifiable,
                        "no bundled log records this tally".to_string(),
                    ),
                }
            }
            XKind::Count(count) => {
                // The one count the bundle can answer: lines in
                // a named file it carries (D18).
                let is_lines = count.noun == "line" || count.noun == "lines";
                match (is_lines, &count.target) {
                    (true, Some(target)) => match read_cited(bundle, target) {
                        Ok(bytes) => {
                            let text = String::from_utf8_lossy(&bytes);
                            let actual = text.lines().count().to_string();
                            if actual == count.digits {
                                (
                                    Status::Ok,
                                    format!("bundle file {target} has {} lines", count.digits),
                                )
                            } else {
                                (
                                    Status::Contradicted,
                                    format!(
                                        "bundle file {target} has {actual} lines, the report says {}",
                                        count.digits
                                    ),
                                )
                            }
                        }
                        Err(_) => (
                            Status::Unverifiable,
                            format!("the bundle does not carry {target}"),
                        ),
                    },
                    (true, None) => (
                        Status::Unverifiable,
                        "the claim names no file whose lines could be counted".to_string(),
                    ),
                    (false, _) => (
                        Status::Unverifiable,
                        format!("the bundle cannot answer what was counted ({})", count.noun),
                    ),
                }
            }
        };
        // A pull request claim checked against a bundled
        // record carries the record's evidence entry, as in
        // the marker mode; the read changes no verdict.
        let evidence = match &claim.kind {
            XKind::Pr(digits) => {
                let cited = format!("records/pr-{}.json", normalize_digits(digits));
                read_cited(bundle, &cited)
                    .ok()
                    .map(|bytes| record_evidence(&cited, &bytes, &PR_RECORD_KEYS))
            }
            _ => None,
        };
        results.push(ExtractedResult {
            line: claim.line,
            class: claim.class,
            trust: claim.trust,
            status,
            detail,
            evidence,
        });
    }
    Extraction { results }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testdir::Dir;
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

    fn kinds(report: &str) -> Vec<XKind> {
        extract_claims(report).into_iter().map(|c| c.kind).collect()
    }

    fn trusts(report: &str) -> Vec<Trust> {
        extract_claims(report)
            .into_iter()
            .map(|c| c.trust)
            .collect()
    }

    #[test]
    fn bare_pr_suppressed_when_explicit_on_the_line() {
        let ks = kinds("PR #450 landed and #450 is cited again\n#451 stands alone");
        assert_eq!(
            ks,
            vec![XKind::Pr("450".to_string()), XKind::Pr("451".to_string())],
            "the bare repeat of an explicit number adds no claim"
        );
    }

    #[test]
    fn pr_forms_carry_the_d17_split() {
        assert_eq!(trusts("PR #450 merged"), vec![Trust::Trusted]);
        assert_eq!(trusts("see pull/450 for it"), vec![Trust::Trusted]);
        assert_eq!(
            trusts("see david-engelmann/peira#309 for the ruling"),
            vec![Trust::Trusted]
        );
        assert_eq!(trusts("the work in #450 landed"), vec![Trust::Provisional]);
    }

    #[test]
    fn pr_without_whitespace_extracts() {
        // The measured pattern is PR\s*# with \s* admitting zero
        // whitespace.
        assert_eq!(kinds("PR#450 merged"), vec![XKind::Pr("450".to_string())]);
    }

    #[test]
    fn repo_form_needs_a_name_on_both_sides_of_the_slash() {
        // "owner/#55" is not the repository form; the number is
        // a bare reference instead, provisional under D17.
        let claims = extract_claims("the note in owner/#55 explains");
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].kind, XKind::Pr("55".to_string()));
        assert_eq!(claims[0].trust, Trust::Provisional);
    }

    #[test]
    fn bare_pr_unverifiable_never_moves_the_exit_code() {
        let dir = bundle_with(&[]);
        let out = verify_extracted("the work in #450 landed", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
        assert_eq!(out.results[0].trust, Trust::Provisional);
        assert_eq!(out.exit_code(), 0, "no trusted claim stands unresolved");
    }

    #[test]
    fn tally_guard_blocks_fused_label_and_hex_prefix() {
        assert!(
            kinds("G5 passed").is_empty(),
            "a fused gate label is not a tally"
        );
        assert!(
            kinds("a1cfc044 15 passed").is_empty(),
            "a tally number right after a hex token is not a tally"
        );
        assert_eq!(
            kinds("the suite ran clean: 15 passed"),
            vec![XKind::TallyFact(RunFact::Passed, "15".to_string())]
        );
    }

    #[test]
    fn of_form_decimal_fragment_is_guarded_in_v2() {
        // Pattern set v2 gives the of-form the fused-token
        // guard: the "01 of 1" fragment inside "0.01 of 1" no
        // longer extracts. A clean of-form still does.
        assert!(
            kinds("the flip rate was 0.01 of 1 in the band")
                .iter()
                .all(|k| !matches!(k, XKind::TallyShape(Shape::Of { .. })))
        );
        assert_eq!(
            kinds("the audit covered 4 of 5 areas"),
            vec![XKind::TallyShape(Shape::Of {
                a: "4".to_string(),
                b: "5".to_string()
            })]
        );
    }

    #[test]
    fn of_form_series_words_are_suppressed_in_v2() {
        assert!(
            kinds("read checkpoint 1 of 3 before signing")
                .iter()
                .all(|k| !matches!(k, XKind::TallyShape(Shape::Of { .. })))
        );
    }

    #[test]
    fn commit_needs_context_and_letters() {
        assert!(kinds("deadbeef was the value").is_empty());
        assert_eq!(
            kinds("the head is deadbeef today"),
            vec![XKind::Commit("deadbeef".to_string())]
        );
        assert!(
            !kinds("1234567 commits landed")
                .iter()
                .any(|k| matches!(k, XKind::Commit(_))),
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
            vec![XKind::GitSha(h40), XKind::Hash256(h64)],
            "41 hex characters is neither a SHA nor a hash"
        );
    }

    #[test]
    fn fraction_and_its_total_both_extract() {
        // The measured behavior that produced the v1 FP4: the
        // fraction and the denominator total are separate claims.
        let ks = kinds("32/38 tests green");
        assert_eq!(
            ks,
            vec![
                XKind::TallyShape(Shape::Total {
                    fam: "test",
                    value: "38".to_string()
                }),
                XKind::TallyShape(Shape::Frac {
                    a: "32".to_string(),
                    b: "38".to_string(),
                    fam: "test"
                }),
            ]
        );
    }

    #[test]
    fn fraction_refuses_commas_as_measured() {
        // The fraction pattern is \d+ on both sides; a comma
        // number cannot form one. The denominator still reads
        // as a total.
        let ks = kinds("1,234/5,678 tests green");
        assert_eq!(
            ks,
            vec![XKind::TallyShape(Shape::Total {
                fam: "test",
                value: "5678".to_string()
            })]
        );
    }

    #[test]
    fn dedupe_identity_is_the_literal_text() {
        // The measured extractor dedupes on the matched text:
        // "1,234 passed" and "1234 passed" on one line are two
        // claims naming the same value.
        let claims = extract_claims("1,234 passed and later 1234 passed again");
        let facts: Vec<&Extracted> = claims.iter().filter(|c| c.class == "tally").collect();
        assert_eq!(facts.len(), 2);
        assert_ne!(facts[0].value, facts[1].value);
        assert_eq!(
            facts[0].kind,
            XKind::TallyFact(RunFact::Passed, "1234".to_string())
        );
        assert_eq!(facts[0].kind, facts[1].kind);
    }

    #[test]
    fn oversized_numbers_still_extract() {
        // The measured extractor's integers are unbounded; a
        // 25-digit tally number is a claim, not a parse error.
        let big = "9".repeat(25);
        let ks = kinds(&format!("the run saw {big} passed"));
        assert_eq!(ks, vec![XKind::TallyFact(RunFact::Passed, big)]);
    }

    #[test]
    fn exit_tallies_extract() {
        assert_eq!(
            kinds("the gate ended with exit code 0"),
            vec![XKind::TallyFact(RunFact::Exit, "0".to_string())]
        );
        assert_eq!(
            kinds("the smoke: exit 2, an environment defect"),
            vec![XKind::TallyFact(RunFact::Exit, "2".to_string())]
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
    fn hash_in_the_repo_working_tree_verifies() {
        // The bundle walk covers the bundled repository's
        // working tree (D18); only .git internals are excluded.
        let content = b"tracked source bytes";
        let hex = sha256::sha256_hex(content);
        let dir = bundle_with(&[("repo/src/lib.rs", content)]);
        let report = format!("the digest is {hex}");
        let out = verify_extracted(&report, dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Ok);
    }

    #[test]
    fn repo_documents_are_not_logs() {
        // A tally recorded inside the bundled repository's
        // documents does not contradict a claim: the repository
        // is git evidence, not a bundled log (D18).
        let dir = bundle_with(&[("repo/NOTES.md", b"the other suite saw 99 passed\n")]);
        let out = verify_extracted("the suite saw 15 passed", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
    }

    #[test]
    fn git_internals_are_not_evidence() {
        let content = b"a loose object";
        let hex = sha256::sha256_hex(content);
        let dir = bundle_with(&[(".git/objects/ab/cdef", content)]);
        let report = format!("the digest is {hex}");
        let out = verify_extracted(&report, dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_evidence_is_not_followed() {
        // A bundle assembled from untrusted material must not
        // pull evidence from outside itself, and a cyclic link
        // must not loop the walk.
        let outside = Dir::new();
        let secret = b"outside the bundle";
        std::fs::write(outside.path().join("secret.txt"), secret).unwrap();
        let hex = sha256::sha256_hex(secret);
        let dir = bundle_with(&[("note.txt", b"inside")]);
        std::os::unix::fs::symlink(outside.path(), dir.path().join("link")).unwrap();
        std::os::unix::fs::symlink(dir.path(), dir.path().join("loop")).unwrap();
        let report = format!("the digest is {hex}");
        let out = verify_extracted(&report, dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
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
    fn pr_record_mismatch_and_missing_fields_are_unverifiable() {
        let dir = bundle_with(&[(
            "records/pr-450.json",
            br#"{"number": 451, "state": "closed", "merged": true}"#,
        )]);
        let out = verify_extracted("PR #450 merged last night", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
        let dir = bundle_with(&[("records/pr-450.json", br#"{"number": 450}"#)]);
        let out = verify_extracted("PR #450 merged last night", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
    }

    #[test]
    fn commit_and_gitsha_verify_through_the_repo_facts() {
        let full = "a".repeat(40);
        let repo = StubRepo {
            commits: ["abc1234ffff".to_string(), full.clone()]
                .into_iter()
                .collect(),
        };
        let dir = bundle_with(&[]);
        let out = verify_extracted("the head abc1234 is pushed", dir.path(), Some(&repo), None);
        assert_eq!(out.results[0].status, Status::Ok);
        let out = verify_extracted(
            &format!("merged at {full} last night"),
            dir.path(),
            Some(&repo),
            None,
        );
        assert_eq!(out.results[0].status, Status::Ok);
        let out = verify_extracted("the head def5678 is pushed", dir.path(), Some(&repo), None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
    }

    #[test]
    fn git_claims_without_a_repository_are_unverifiable() {
        let dir = bundle_with(&[]);
        let out = verify_extracted("the head abc1234 is pushed", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
        assert!(out.results[0].detail.contains("no repository"));
        let full = "a".repeat(40);
        let out = verify_extracted(&format!("merged at {full}"), dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
    }

    #[test]
    fn tally_checks_against_bundled_logs_and_names_the_log() {
        let dir = bundle_with(&[("run.log", b"running\n15 passed\n0 failed\n")]);
        let out = verify_extracted("the suite saw 15 passed", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Ok);
        assert_eq!(out.results[0].trust, Trust::Trusted);
        assert!(out.results[0].detail.contains("run.log"));
        let out = verify_extracted("the suite saw 14 passed", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Contradicted);
        assert!(out.results[0].detail.contains("run.log"));
    }

    #[test]
    fn failed_and_skipped_tallies_check_end_to_end() {
        let dir = bundle_with(&[("run.log", b"3 failed, 1 skipped\n")]);
        let out = verify_extracted("the run saw 3 failed", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Ok);
        let out = verify_extracted("the run saw 2 failed", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Contradicted);
        let out = verify_extracted("the run saw 1 skipped", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Ok);
    }

    #[test]
    fn extracted_pr_result_carries_the_record_evidence() {
        let record = b"{\"number\":452,\"state\":\"closed\",\"merged\":true}";
        let dir = bundle_with(&[("records/pr-452.json", record)]);
        let out = verify_extracted("the gate for PR #452 is green", dir.path(), None, None);
        let ev = out.results[0].evidence.as_ref().expect("record evidence");
        assert_eq!(ev.path, "records/pr-452.json");
        assert_eq!(ev.sha256, sha256::sha256_hex(record));
        assert_eq!(ev.fields.len(), 3);
    }

    #[test]
    fn tally_contradiction_moves_the_exit_code() {
        // Tallies are trusted after D17: a contradicted tally
        // is a contradicted claim.
        let dir = bundle_with(&[("run.log", b"15 passed\n")]);
        let out = verify_extracted("the suite saw 14 passed", dir.path(), None, None);
        assert_eq!(out.exit_code(), 1);
    }

    #[test]
    fn shape_verdicts_agree_contradict_and_miss() {
        let dir = bundle_with(&[("run.log", b"30/38 tests green\n4,554 of 4,701 trials\n")]);
        let out = verify_extracted("the board shows 30/38 tests", dir.path(), None, None);
        assert!(
            out.results.iter().all(|r| r.status == Status::Ok),
            "agreeing shapes verify: {:?}",
            out.results
        );
        let out = verify_extracted("the board shows 32/38 tests", dir.path(), None, None);
        let frac = out
            .results
            .iter()
            .find(|r| r.detail.contains("different tally"))
            .expect("the fraction claim is contradicted by the log's 30/38");
        assert_eq!(frac.status, Status::Contradicted);
        assert!(frac.detail.contains("run.log"));
        let out = verify_extracted("the trials ran 4,554 of 4,701", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Ok);
        let out = verify_extracted("the board shows 41 tests", dir.path(), None, None);
        assert_eq!(
            out.results[0].status,
            Status::Contradicted,
            "the log records a total for the same family"
        );
        let out = verify_extracted("the board shows 9 of 12 areas", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
    }

    #[test]
    fn count_lines_checks_against_the_named_file() {
        let dir = bundle_with(&[("doc.md", b"a\nb\nc\n")]);
        let out = verify_extracted("the doc.md grew to 3 lines", dir.path(), None, None);
        assert_eq!(out.results.len(), 1);
        assert_eq!(out.results[0].status, Status::Ok);
        assert_eq!(out.results[0].trust, Trust::Provisional);
        let out = verify_extracted("the doc.md grew to 5 lines", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Contradicted);
        assert_eq!(
            out.exit_code(),
            0,
            "a provisional contradiction moves nothing"
        );
    }

    #[test]
    fn counts_the_bundle_cannot_answer_are_unverifiable() {
        let dir = bundle_with(&[("doc.md", b"a\nb\nc\n")]);
        let out = verify_extracted("the doc.md lists 3 files", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
        let out = verify_extracted("the draft grew to 3 lines", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
        let out = verify_extracted("the other.md grew to 3 lines", dir.path(), None, None);
        assert_eq!(out.results[0].status, Status::Unverifiable);
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
