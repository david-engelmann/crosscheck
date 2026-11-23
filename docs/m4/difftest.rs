// Differential harness: print extract_claims for one report in
// the measured extractor's projection (line|class|value).
use crosscheck_core::extract::extract_claims;

fn main() {
    let path = std::env::args().nth(1).expect("report path");
    let bytes = std::fs::read(&path).expect("read report");
    let text = String::from_utf8_lossy(&bytes);
    for c in extract_claims(&text) {
        println!("{}|{}|{}", c.line, c.class, c.value);
    }
}
