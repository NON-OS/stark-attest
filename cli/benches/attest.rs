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

//! Measured performance, criterion-grade: warmup, sampling, outlier analysis,
//! and a saved baseline under target/criterion for regression comparison.
//! Three numbers matter and each gets its own group:
//!
//!   commit    - measuring a set and building its tree (per set)
//!   prove     - one member's trailer, the release-time cost (per member)
//!   verify    - one member's gate check, the everywhere-forever cost
//!
//! Artifacts are synthetic but sized like real ones. Run with
//! `cargo bench -p stark-attest`; compare runs with criterion's baselines
//! rather than eyeballs.

use criterion::{criterion_group, criterion_main, Criterion};
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

fn bench_attest(c: &mut Criterion) {
    // eight synthetic artifacts, 64 KiB each: a realistic capsule-scale set
    let images: Vec<Vec<u8>> = (0u8..8)
        .map(|i| (0..65536).map(|j| (j as u8).wrapping_mul(i.wrapping_add(3))).collect())
        .collect();
    let refs: Vec<&[u8]> = images.iter().map(Vec::as_slice).collect();
    let mut padded: Vec<&[u8]> = refs.clone();
    while padded.len() < LEAVES {
        padded.push(PAD);
    }
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);

    c.bench_function("commit/8x64KiB", |b| b.iter(|| MeasuredSet::commit(&hasher, &padded)));

    let set = MeasuredSet::commit(&hasher, &padded);
    let root = root_bytes(set.root());
    let mut ctx = Vec::new();
    ctx.extend_from_slice(blake3::hash(&images[0]).as_bytes());
    ctx.extend_from_slice(&[0x19]);

    let mut prove_group = c.benchmark_group("prove");
    prove_group.sample_size(10); // proving is seconds-scale; ten samples is honest
    prove_group.bench_function("one-member", |b| {
        b.iter(|| {
            build_attestation_trailer_from_set(
                &hasher,
                LOG_ROUNDS,
                &set,
                0,
                &ctx,
                N_QUERIES,
                GRIND_BITS,
                EXTRA_BLOWUP_BITS,
            )
        })
    });
    prove_group.finish();

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
    assert!(verify(&root, &trailer, &ctx), "bench fixture must verify");

    c.bench_function("verify/one-member", |b| b.iter(|| verify(&root, &trailer, &ctx)));
}

criterion_group!(benches, bench_attest);
criterion_main!(benches);
