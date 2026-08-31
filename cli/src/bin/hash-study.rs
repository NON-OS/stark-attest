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

//! Where the hashing cost actually goes, and what the hybrid buys.
//!
//! Two hash families meet in any STARK that commits to real data. An
//! arithmetization-friendly hash (here Poseidon over Goldilocks) is cheap as
//! constraints and expensive as instructions. A general-purpose hash (BLAKE3)
//! is the reverse by three orders of magnitude. Which one measures the bytes
//! of an artifact is an architectural decision that is usually made once, in
//! passing, and then never measured.
//!
//! This measures it. Three ways to bind an artifact into a proof:
//!
//!   1. `poseidon_direct`     absorb every artifact byte into the algebraic
//!                            sponge. The proof binds the bytes. Simple, and
//!                            the cost is the sponge's rate.
//!   2. `blake3_only`         hash the bytes with BLAKE3. Not a complete
//!                            scheme on its own, measured as the floor: no
//!                            binding scheme can beat the cost of reading the
//!                            bytes once through a fast hash.
//!   3. `hybrid`              BLAKE3 the artifact to 32 bytes, absorb only
//!                            that digest into Poseidon. The proof binds the
//!                            digest, and the digest binds the bytes.
//!
//! The security question the hybrid raises is worth stating rather than
//! assuming: it replaces "the proof binds these bytes" with "the proof binds a
//! BLAKE3 digest, and BLAKE3 is collision resistant". For this crate that is
//! not a new assumption. The proof context already carries the artifact's
//! BLAKE3 measurement, so a BLAKE3 collision already breaks binding today; the
//! hybrid does not add an assumption, it removes redundant work. A system
//! whose context does not already commit to a general-purpose digest is making
//! a different trade and should measure its own case.
//!
//! Run: `cargo run --release --bin hash-study -- bench/hash-study.json`

use std::env;
use std::fs;
use std::time::Instant;

use nonos_stark::air::{Poseidon, RATE, WIDTH};
use nonos_stark::attest_params::LOG_ROUNDS;
use nonos_stark::field::Fp;

/// Absorb a byte string into the sponge, eight bytes per lane, rate lanes per
/// permutation. This is the shape a direct binding takes.
fn poseidon_absorb(h: &Poseidon, bytes: &[u8]) -> [Fp; RATE] {
    let mut state = [Fp::ZERO; WIDTH];
    for chunk in bytes.chunks(RATE * 8) {
        for (lane, word) in chunk.chunks(8).enumerate() {
            let mut w = [0u8; 8];
            w[..word.len()].copy_from_slice(word);
            state[lane] = state[lane] + Fp::from_u64(u64::from_le_bytes(w));
        }
        state = h.permute(state);
    }
    let mut out = [Fp::ZERO; RATE];
    out.copy_from_slice(&state[..RATE]);
    out
}

fn median(xs: &mut [u128]) -> u128 {
    xs.sort_unstable();
    if xs.is_empty() {
        0
    } else {
        xs[xs.len() / 2]
    }
}

fn time(iterations: usize, mut f: impl FnMut()) -> u128 {
    f();
    let mut raw = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let t = Instant::now();
        f();
        raw.push(t.elapsed().as_nanos());
    }
    median(&mut raw)
}

fn main() {
    let out = env::args().nth(1).unwrap_or_else(|| "bench/hash-study.json".to_string());
    let h = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);

    let mut rows = Vec::new();
    for &size in &[4096usize, 65536, 1048576] {
        let data: Vec<u8> = (0..size).map(|i| (i as u8).wrapping_mul(31)).collect();

        let iters = if size >= 1 << 20 { 3 } else { 20 };
        let p_ns = time(iters, || {
            let _ = poseidon_absorb(&h, &data);
        });
        let b_ns = time(iters * 10, || {
            let _ = blake3::hash(&data);
        });
        // the hybrid pays BLAKE3 over the bytes plus one sponge over 32 bytes
        let digest = blake3::hash(&data);
        let hy_ns = time(iters * 5, || {
            let d = blake3::hash(&data);
            let _ = poseidon_absorb(&h, d.as_bytes());
        });
        let _ = digest;

        let mbps = |ns: u128| (size as f64 / (ns as f64 / 1e9)) / 1e6;
        let speedup = p_ns as f64 / hy_ns.max(1) as f64;

        println!(
            "{:>9} bytes   poseidon {:>10.2} MB/s   blake3 {:>10.2} MB/s   hybrid {:>10.2} MB/s   speedup x{:.0}",
            size,
            mbps(p_ns),
            mbps(b_ns),
            mbps(hy_ns),
            speedup
        );

        rows.push(format!(
            "    {{\n      \"artifact_bytes\": {size},\n      \"poseidon_direct_ns\": {p_ns},\n      \"blake3_only_ns\": {b_ns},\n      \"hybrid_ns\": {hy_ns},\n      \"poseidon_mb_per_s\": {:.2},\n      \"blake3_mb_per_s\": {:.2},\n      \"hybrid_mb_per_s\": {:.2},\n      \"hybrid_speedup\": {:.1}\n    }}",
            mbps(p_ns),
            mbps(b_ns),
            mbps(hy_ns),
            speedup
        ));
    }

    let doc = format!(
        "{{\n  \"schema\": \"stark-attest.hash-study.v1\",\n  \"question\": \"which hash should measure artifact bytes when a proof must bind them\",\n  \"designs\": {{\n    \"poseidon_direct\": \"absorb every byte into the algebraic sponge; the proof binds the bytes\",\n    \"blake3_only\": \"not a scheme, the floor: the cost of reading the bytes once\",\n    \"hybrid\": \"BLAKE3 to 32 bytes, absorb only the digest; the proof binds the digest\"\n  }},\n  \"security_note\": \"the hybrid rests on BLAKE3 collision resistance, which this crate's proof context already assumes, so it removes redundant work rather than adding an assumption\",\n  \"host\": {{ \"os\": \"{}\", \"arch\": \"{}\" }},\n  \"measurements\": [\n{}\n  ]\n}}\n",
        env::consts::OS,
        env::consts::ARCH,
        rows.join(",\n")
    );

    if let Some(p) = std::path::Path::new(&out).parent() {
        let _ = fs::create_dir_all(p);
    }
    fs::write(&out, &doc).expect("write the study");
    println!("\nwrote {out}");
}
