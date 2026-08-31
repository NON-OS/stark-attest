// stark-attest
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Compare two measurement records and fail on a real regression.
//!
//! Recording numbers is worth little if nothing reads them. This compares a
//! fresh record against the one in the repository and exits non-zero when an
//! operation got materially slower, so a change that doubles proving time is
//! caught by the same push that introduced it.
//!
//! Two rules keep it from crying wolf on a shared runner:
//!
//!   - medians are compared, not means, because one descheduled iteration
//!     moves a mean and not a median;
//!   - the tolerance is generous (the default is 100 percent, a doubling),
//!     because a hosted runner's variance across two arbitrary machines is
//!     itself tens of percent. A gate that fires on noise gets muted, and a
//!     muted gate is worse than none.
//!
//! Comparing across different host shapes is refused rather than fudged: a
//! four core runner and a sixteen core runner are not comparable and pretending
//! otherwise produces confident nonsense.
//!
//! Run: `compare-bench <baseline.json> <candidate.json> [tolerance-percent]`

use std::env;
use std::fs;
use std::process::exit;

/// Pull a `"key": value` number out of a flat-enough JSON text. The records
/// this reads are written by `emit-bench` in a fixed shape, so a full parser
/// would be ceremony; a wrong read here fails loudly rather than silently
/// because the caller checks that every expected sample was found.
fn field(text: &str, key: &str, from: usize) -> Option<(u128, usize)> {
    let pat = format!("\"{key}\":");
    let at = text[from..].find(&pat)? + from + pat.len();
    let rest = &text[at..];
    let end = rest.find(|c: char| c == ',' || c == '\n' || c == '}')?;
    let raw = rest[..end].trim();
    raw.parse::<u128>().ok().map(|v| (v, at + end))
}

fn string_field(text: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":");
    let at = text.find(&pat)? + pat.len();
    let rest = text[at..].trim_start();
    let rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// Every sample's name and median, in file order.
fn medians(text: &str) -> Vec<(String, u128)> {
    let mut out = Vec::new();
    let mut cursor = 0usize;
    while let Some(at) = text[cursor..].find("\"name\":") {
        let start = cursor + at;
        let rest = &text[start + 7..];
        let Some(open) = rest.find('"') else { break };
        let after = &rest[open + 1..];
        let Some(close) = after.find('"') else { break };
        let name = after[..close].to_string();
        match field(text, "median_ns", start) {
            Some((median, next)) => {
                out.push((name, median));
                cursor = next;
            }
            None => break,
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: compare-bench <baseline.json> <candidate.json> [tolerance-percent]");
        exit(2);
    }
    let tolerance: f64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(100.0);

    let base = fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("no baseline at {}: {e}", args[1]);
        // A missing baseline is the first run, not a regression.
        exit(0)
    });
    let cand = fs::read_to_string(&args[2]).unwrap_or_else(|e| {
        eprintln!("cannot read candidate {}: {e}", args[2]);
        exit(2)
    });

    // Different machines are not comparable. Say so and stop rather than
    // report a regression that is really a change of hardware.
    let shape = |t: &str| {
        (
            string_field(t, "os").unwrap_or_default(),
            string_field(t, "arch").unwrap_or_default(),
            field(t, "logical_cores", 0).map(|(v, _)| v).unwrap_or(0),
        )
    };
    if shape(&base) != shape(&cand) {
        println!("host shape changed between records; comparison skipped");
        let (o, a, c) = shape(&base);
        println!("  baseline  {o}/{a}, {c} cores");
        let (o, a, c) = shape(&cand);
        println!("  candidate {o}/{a}, {c} cores");
        exit(0);
    }

    let b = medians(&base);
    let c = medians(&cand);
    if b.is_empty() || c.is_empty() {
        eprintln!("one of the records carries no samples; refusing to compare");
        exit(2);
    }

    let mut regressed = Vec::new();
    for (name, base_median) in &b {
        let Some((_, cand_median)) = c.iter().find(|(n, _)| n == name) else {
            eprintln!("sample {name} vanished from the candidate record");
            exit(2);
        };
        let ratio = *cand_median as f64 / *base_median as f64;
        let delta = (ratio - 1.0) * 100.0;
        let verdict = if delta > tolerance { "REGRESSED" } else { "ok" };
        println!("{verdict:>10}  {name:<16} {base_median} -> {cand_median} ns ({delta:+.1}%)");
        if delta > tolerance {
            regressed.push(name.clone());
        }
    }

    if regressed.is_empty() {
        println!("\nno operation exceeded the {tolerance:.0}% tolerance");
    } else {
        eprintln!(
            "\n{} operation(s) regressed past {tolerance:.0}%: {}",
            regressed.len(),
            regressed.join(", ")
        );
        exit(1);
    }
}
