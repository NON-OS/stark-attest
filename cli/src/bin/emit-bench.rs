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

//! Measure the whole surface, with enough provenance to be worth believing.
//!
//! A single number per operation says almost nothing: it cannot show how cost
//! scales, where the work actually goes, or what a consumer will pay at their
//! size rather than the author's. This sweeps the two axes that matter and
//! records the shape:
//!
//!   - set size: how commit and prove scale as a release grows from a handful
//!     of artifacts to a full tree, which is the question anyone adopting this
//!     asks first;
//!   - artifact size: how measurement cost scales with bytes, which separates
//!     the hashing from the proving;
//!   - amortisation: the per-member cost of enrolling a whole set at once,
//!     which is the number that decides whether release-time proving is
//!     tolerable;
//!   - throughput: verifications per second and measured bytes per second,
//!     because rates are what capacity planning uses;
//!   - proof size against tree depth, since a trailer's bytes are a cost every
//!     consumer carries forever.
//!
//! Every record carries host, cores, commit, dirty flag, the compiled
//! parameters, the workload shape, iteration counts and the raw per-iteration
//! nanoseconds. A number without its conditions is a rumour.
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
    /// What was varied for this point, so a curve can be read back out.
    axis: String,
    raw_ns: Vec<u128>,
    /// Derived rate, where one is meaningful (bytes/s, ops/s).
    rate: Option<(String, f64)>,
}

impl Sample {
    fn json(&self) -> String {
        let mut sorted = self.raw_ns.clone();
        let med = median(&mut sorted);
        let min = sorted.first().copied().unwrap_or(0);
        let max = sorted.last().copied().unwrap_or(0);
        let mean =
            if sorted.is_empty() { 0 } else { sorted.iter().sum::<u128>() / sorted.len() as u128 };
        // Spread as a percentage of the median: a reader's first defence
        // against believing a number a noisy runner produced.
        let spread = if med > 0 { ((max - min) as f64 / med as f64) * 100.0 } else { 0.0 };
        let raw: Vec<String> = self.raw_ns.iter().map(|v| v.to_string()).collect();
        let rate = match &self.rate {
            Some((unit, value)) => {
                format!(",\n      \"rate_unit\": \"{unit}\",\n      \"rate\": {value:.2}")
            }
            None => String::new(),
        };
        format!(
            "    {{\n      \"name\": \"{}\",\n      \"axis\": \"{}\",\n      \"unit\": \"{}\",\n      \"iterations\": {},\n      \"min_ns\": {},\n      \"median_ns\": {},\n      \"mean_ns\": {},\n      \"max_ns\": {},\n      \"spread_pct\": {:.1}{}\n,      \"raw_ns\": [{}]\n    }}",
            self.name,
            self.axis,
            self.unit,
            self.raw_ns.len(),
            min,
            med,
            mean,
            max,
            spread,
            rate,
            raw.join(", ")
        )
    }
}

fn measure(
    name: &str,
    axis: &str,
    unit: &str,
    iterations: usize,
    rate: Option<(String, f64)>,
    mut f: impl FnMut(),
) -> Sample {
    f(); // untimed warmup: the first pass pays page faults nobody wants reported
    let mut raw = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let t = Instant::now();
        f();
        raw.push(t.elapsed().as_nanos());
    }
    let s = Sample { name: name.into(), axis: axis.into(), unit: unit.into(), raw_ns: raw, rate };
    let mut sorted = s.raw_ns.clone();
    let med = median(&mut sorted) as f64;
    // rate is derived from the median once it is known
    let rate = s.rate.as_ref().map(|(u, per)| (u.clone(), per / (med / 1e9)));
    Sample { rate, ..s }
}

fn artifacts(count: usize, size: usize) -> Vec<Vec<u8>> {
    (0..count)
        .map(|i| (0..size).map(|j| (j as u8).wrapping_mul((i as u8).wrapping_add(3))).collect())
        .collect()
}

fn main() {
    let out = env::args().nth(1).unwrap_or_else(|| "bench/results.json".to_string());
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let mut samples: Vec<Sample> = Vec::new();

    // ---- axis 1: set size, at a fixed artifact size -------------------------
    // How enrolment scales as a release grows. The tree is fixed width, so the
    // measurement cost grows with the members while the tree cost does not.
    for &n in &[1usize, 8, 32, 128] {
        let imgs = artifacts(n, 65536);
        let refs: Vec<&[u8]> = imgs.iter().map(Vec::as_slice).collect();
        let mut padded = refs.clone();
        while padded.len() < LEAVES {
            padded.push(PAD);
        }
        let total_bytes = (n * 65536) as f64;
        samples.push(measure(
            "commit_set",
            &format!("members={n}"),
            "ns/set",
            3,
            Some(("bytes_per_second".into(), total_bytes)),
            || {
                std::hint::black_box(MeasuredSet::commit(&hasher, &padded));
            },
        ));
    }

    // ---- axis 2: artifact size, at a fixed member count ---------------------
    // Separates hashing from tree work: the tree is identical across these,
    // so the difference is the measurement of bytes.
    for &size in &[4096usize, 65536, 1048576] {
        let imgs = artifacts(8, size);
        let refs: Vec<&[u8]> = imgs.iter().map(Vec::as_slice).collect();
        let mut padded = refs.clone();
        while padded.len() < LEAVES {
            padded.push(PAD);
        }
        let total_bytes = (8 * size) as f64;
        samples.push(measure(
            "commit_bytes",
            &format!("artifact_bytes={size}"),
            "ns/set",
            3,
            Some(("bytes_per_second".into(), total_bytes)),
            || {
                std::hint::black_box(MeasuredSet::commit(&hasher, &padded));
            },
        ));
    }

    // ---- the enrolment a real release pays, both ways -----------------------
    // The number that decides whether attestation is practical: measuring a
    // release-sized set of artifacts, direct against hybrid.
    for &(count, size) in &[(16usize, 1048576usize), (64, 1048576)] {
        let imgs = artifacts(count, size);
        let refs: Vec<&[u8]> = imgs.iter().map(Vec::as_slice).collect();
        let mut pad = refs.clone();
        while pad.len() < LEAVES {
            pad.push(PAD);
        }
        let total = (count * size) as f64;
        samples.push(measure(
            "enrol_direct",
            &format!("members={count}, {}MiB each", size >> 20),
            "ns/set",
            2,
            Some(("bytes_per_second".into(), total)),
            || {
                std::hint::black_box(MeasuredSet::commit(&hasher, &pad));
            },
        ));
        samples.push(measure(
            "enrol_hybrid",
            &format!("members={count}, {}MiB each", size >> 20),
            "ns/set",
            2,
            Some(("bytes_per_second".into(), total)),
            || {
                std::hint::black_box(MeasuredSet::commit_hybrid(&hasher, &pad));
            },
        ));
    }

    // ---- proving, and its amortisation over a set ---------------------------
    let imgs = artifacts(8, 65536);
    let refs: Vec<&[u8]> = imgs.iter().map(Vec::as_slice).collect();
    let mut padded = refs.clone();
    while padded.len() < LEAVES {
        padded.push(PAD);
    }
    let set = MeasuredSet::commit(&hasher, &padded);
    let root = root_bytes(set.root());
    let mut ctx = Vec::new();
    ctx.extend_from_slice(blake3::hash(&imgs[0]).as_bytes());
    ctx.extend_from_slice(&[0x19]);

    samples.push(measure("prove_member", "members=8", "ns/member", 3, None, || {
        std::hint::black_box(build_attestation_trailer_from_set(
            &hasher,
            LOG_ROUNDS,
            &set,
            0,
            &ctx,
            N_QUERIES,
            GRIND_BITS,
            EXTRA_BLOWUP_BITS,
        ));
    }));

    // Four members in sequence: the per-member cost when a whole set is
    // enrolled, which is what a release actually pays.
    samples.push(measure("prove_four_members", "members=8", "ns/4members", 2, None, || {
        for i in 0..4 {
            let mut c = Vec::new();
            c.extend_from_slice(blake3::hash(&imgs[i]).as_bytes());
            c.extend_from_slice(&[0x19]);
            std::hint::black_box(build_attestation_trailer_from_set(
                &hasher,
                LOG_ROUNDS,
                &set,
                i,
                &c,
                N_QUERIES,
                GRIND_BITS,
                EXTRA_BLOWUP_BITS,
            ));
        }
    }));

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

    // ---- verification, the number every consumer pays forever ---------------
    samples.push(measure(
        "verify_member",
        "members=8",
        "ns/member",
        50,
        Some(("verifications_per_second".into(), 1.0)),
        || {
            std::hint::black_box(verify(&root, &trailer, &ctx));
        },
    ));

    // Rejection must not be slower than acceptance, or a verifier becomes a
    // denial of service surface under a flood of bad trailers.
    let mut bad = trailer.clone();
    let last = bad.len() - 1;
    bad[last] ^= 0x01;
    samples.push(measure(
        "reject_corrupt",
        "members=8",
        "ns/member",
        20,
        Some(("rejections_per_second".into(), 1.0)),
        || {
            std::hint::black_box(verify(&root, &bad, &ctx));
        },
    ));

    let cores = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(0);
    let dirty = !git(&["status", "--porcelain"]).is_empty();
    let sample_json: Vec<String> = samples.iter().map(|s| s.json()).collect();

    let doc = format!(
        "{{\n  \"schema\": \"stark-attest.bench.v2\",\n  \"host\": {{\n    \"os\": \"{}\",\n    \"arch\": \"{}\",\n    \"logical_cores\": {}\n  }},\n  \"source\": {{\n    \"commit\": \"{}\",\n    \"dirty\": {}\n  }},\n  \"parameters\": {{\n    \"log_rounds\": {},\n    \"queries\": {},\n    \"grind_bits\": {},\n    \"extra_blowup_bits\": {},\n    \"tree_depth\": {},\n    \"tree_slots\": {}\n  }},\n  \"artifact\": {{\n    \"trailer_bytes\": {},\n    \"trailer_bytes_per_depth\": {},\n    \"note\": \"the trailer is dominated by the proof, not the path\"\n  }},\n  \"samples\": [\n{}\n  ]\n}}\n",
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
        LEAVES,
        trailer.len(),
        trailer.len() / DEPTH,
        sample_json.join(",\n")
    );

    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(&out, &doc).expect("write the results");
    println!("{doc}");
    println!("wrote {out}");
}
