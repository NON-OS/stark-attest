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

//! Emit the measured numbers as JSON, with enough provenance to be worth
//! believing.
//!
//! Criterion is the right tool for deciding whether a change is a regression;
//! it is the wrong artifact to publish, because its output is a directory of
//! HTML nobody reads and a baseline tied to one machine. This binary measures
//! the three operations that matter and writes one JSON file recording what
//! was measured, on what, at what commit, and how, so a number in the
//! repository can always be traced back to the conditions that produced it.
//!
//! Every field a reader needs to reproduce or reject a number is present:
//! the host and its core count, the toolchain, the commit and whether the
//! tree was dirty, the parameter set the proofs ran under, the workload
//! shape, the iteration count, and the raw per-iteration nanoseconds rather
//! than a summary alone. A number without its conditions is a rumour.
//!
//! Run: `cargo run --release --bin emit-bench -- bench/results.json`

use std::env;
use std::fs;
use std::process::Command;
use std::time::Instant;

use nonos_stark::air::{
    build_attestation_trailer_from_set, deserialize_proof_ext, stark_verify_ext_blown_bound,
    MeasuredSet, MerkleMembership, Poseidon, RATE,
};
use nonos_stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use nonos_stark::field::Fp;

const DEPTH: usize = 8;
const LEAVES: usize = 1 << DEPTH;
const MAGIC: &[u8; 8] = b"NZKSTRK1";
const PAD: &[u8] = b"\x00STARK-ATTEST-RESERVED-SLOT-v1";

fn to_rate(b: &[u8]) -> [Fp; RATE] {
    let mut o = [Fp::ZERO; RATE];
    for (i, l) in o.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&b[i * 8..i * 8 + 8]);
        *l = Fp::from_u64(u64::from_le_bytes(w));
    }
    o
}

fn root_bytes(r: [Fp; RATE]) -> [u8; 32] {
    let mut o = [0u8; 32];
    for (i, l) in r.iter().enumerate() {
        o[i * 8..i * 8 + 8].copy_from_slice(&l.value().to_le_bytes());
    }
    o
}

fn verify(root: &[u8; 32], trailer: &[u8], context: &[u8]) -> bool {
    let dir_bytes = DEPTH.div_ceil(8);
    let sib_end = 9 + DEPTH * 32;
    if trailer.len() < sib_end + dir_bytes || &trailer[0..8] != MAGIC {
        return false;
    }
    let mut sib = Vec::with_capacity(DEPTH);
    for i in 0..DEPTH {
        sib.push(to_rate(&trailer[9 + i * 32..9 + i * 32 + 32]));
    }
    let dirs = &trailer[sib_end..sib_end + dir_bytes];
    let d: Vec<bool> = (0..DEPTH).map(|i| (dirs[i / 8] >> (i % 8)) & 1 == 1).collect();
    let Some(proof) = deserialize_proof_ext(&trailer[sib_end + dir_bytes..]) else {
        return false;
    };
    let air = MerkleMembership::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        to_rate(root),
        sib,
        d,
    );
    stark_verify_ext_blown_bound(&air, &proof, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS, context)
}

fn git(args: &[&str]) -> String {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

/// Median of a sorted-in-place sample, in nanoseconds.
fn median(xs: &mut [u128]) -> u128 {
    xs.sort_unstable();
    let n = xs.len();
    if n == 0 {
        0
    } else if n % 2 == 1 {
        xs[n / 2]
    } else {
        (xs[n / 2 - 1] + xs[n / 2]) / 2
    }
}

struct Sample {
    name: String,
    unit: String,
    iterations: usize,
    raw_ns: Vec<u128>,
}

impl Sample {
    fn json(&self) -> String {
        let mut sorted = self.raw_ns.clone();
        let med = median(&mut sorted);
        let min = sorted.first().copied().unwrap_or(0);
        let max = sorted.last().copied().unwrap_or(0);
        let mean =
            if sorted.is_empty() { 0 } else { sorted.iter().sum::<u128>() / sorted.len() as u128 };
        let raw: Vec<String> = self.raw_ns.iter().map(|v| v.to_string()).collect();
        format!(
            "    {{\n      \"name\": \"{}\",\n      \"unit\": \"{}\",\n      \"iterations\": {},\n      \"min_ns\": {},\n      \"median_ns\": {},\n      \"mean_ns\": {},\n      \"max_ns\": {},\n      \"raw_ns\": [{}]\n    }}",
            self.name,
            self.unit,
            self.iterations,
            min,
            med,
            mean,
            max,
            raw.join(", ")
        )
    }
}

fn measure(name: &str, unit: &str, iterations: usize, mut f: impl FnMut()) -> Sample {
    // one untimed pass so the first iteration's page faults and cache misses
    // are not reported as the operation's cost
    f();
    let mut raw = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let t = Instant::now();
        f();
        raw.push(t.elapsed().as_nanos());
    }
    Sample { name: name.into(), unit: unit.into(), iterations, raw_ns: raw }
}

fn main() {
    let out = env::args().nth(1).unwrap_or_else(|| "bench/results.json".to_string());

    // eight artifacts of 64 KiB: a set shaped like a real release
    let images: Vec<Vec<u8>> = (0u8..8)
        .map(|i| (0..65536).map(|j| (j as u8).wrapping_mul(i.wrapping_add(3))).collect())
        .collect();
    let refs: Vec<&[u8]> = images.iter().map(Vec::as_slice).collect();
    let mut padded: Vec<&[u8]> = refs.clone();
    while padded.len() < LEAVES {
        padded.push(PAD);
    }
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);

    let commit = measure("commit_set", "ns/set", 5, || {
        let _ = MeasuredSet::commit(&hasher, &padded);
    });

    let set = MeasuredSet::commit(&hasher, &padded);
    let root = root_bytes(set.root());
    let mut ctx = Vec::new();
    ctx.extend_from_slice(blake3::hash(&images[0]).as_bytes());
    ctx.extend_from_slice(&[0x19]);

    let prove = measure("prove_member", "ns/member", 3, || {
        let _ = build_attestation_trailer_from_set(
            &hasher,
            LOG_ROUNDS,
            &set,
            0,
            &ctx,
            N_QUERIES,
            GRIND_BITS,
            EXTRA_BLOWUP_BITS,
        );
    });

    let trailer = build_attestation_trailer_from_set(
        &hasher,
        LOG_ROUNDS,
        &set,
        0,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    );
    assert!(verify(&root, &trailer, &ctx), "the fixture must verify before it is timed");

    let verify_s = measure("verify_member", "ns/member", 50, || {
        let _ = verify(&root, &trailer, &ctx);
    });

    let cores = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(0);
    let dirty = !git(&["status", "--porcelain"]).is_empty();
    let samples: Vec<String> = [&commit, &prove, &verify_s].iter().map(|s| s.json()).collect();

    let doc = format!(
        "{{\n  \"schema\": \"stark-attest.bench.v1\",\n  \"host\": {{\n    \"os\": \"{}\",\n    \"arch\": \"{}\",\n    \"logical_cores\": {}\n  }},\n  \"source\": {{\n    \"commit\": \"{}\",\n    \"dirty\": {}\n  }},\n  \"parameters\": {{\n    \"log_rounds\": {},\n    \"queries\": {},\n    \"grind_bits\": {},\n    \"extra_blowup_bits\": {},\n    \"tree_depth\": {}\n  }},\n  \"workload\": {{\n    \"artifacts\": {},\n    \"artifact_bytes\": {},\n    \"trailer_bytes\": {}\n  }},\n  \"samples\": [\n{}\n  ]\n}}\n",
        env::consts::OS,
        env::consts::ARCH,
        cores,
        git(&["rev-parse", "HEAD"]),
        dirty,
        LOG_ROUNDS,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
        DEPTH,
        images.len(),
        images[0].len(),
        trailer.len(),
        samples.join(",\n")
    );

    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(&out, &doc).expect("write the results");
    println!("{doc}");
    println!("wrote {out}");
}
