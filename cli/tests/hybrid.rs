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

//! The hybrid measurement binds what it claims to bind.
//!
//! Replacing "absorb the bytes" with "absorb a digest of the bytes" is a
//! change to what a proof means, so it needs the properties spelled out and
//! tested rather than assumed from the speedup:
//!
//!   - it is a function of the bytes: the same image measures the same way,
//!     different images measure differently;
//!   - it is domain separated from the direct measurement, so no image can
//!     measure to the same leaf under both schemes and a set cannot be
//!     silently reinterpreted from one to the other;
//!   - a set committed one way does not verify against a root built the other
//!     way, which is what makes the separation operationally real rather than
//!     a comment;
//!   - it still proves membership end to end, with tampering refused exactly
//!     as the direct path refuses it.

use nonos_stark::air::{
    build_attestation_trailer_from_set, deserialize_proof_ext, measure_capsule,
    measure_capsule_hybrid, stark_verify_ext_blown_bound, MeasuredSet, MerkleMembership, Poseidon,
    RATE,
};
use nonos_stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use nonos_stark::field::Fp;

const DEPTH: usize = 8;
const LEAVES: usize = 1 << DEPTH;
const MAGIC: &[u8; 8] = b"NZKSTRK1";
const PAD: &[u8] = b"\x00STARK-ATTEST-RESERVED-SLOT-v1";

fn hasher() -> Poseidon {
    Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE])
}

fn padded<'a>(imgs: &[&'a [u8]]) -> Vec<&'a [u8]> {
    let mut v: Vec<&[u8]> = imgs.to_vec();
    while v.len() < LEAVES {
        v.push(PAD);
    }
    v
}

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

fn gate(root: &[u8; 32], trailer: &[u8], context: &[u8]) -> bool {
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
    let air = MerkleMembership::new(hasher(), LOG_ROUNDS, to_rate(root), sib, d);
    stark_verify_ext_blown_bound(&air, &proof, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS, context)
}

#[test]
fn the_hybrid_measurement_is_a_function_of_the_bytes() {
    let h = hasher();
    let a: &[u8] = b"the same artifact bytes, measured twice";
    assert_eq!(
        measure_capsule_hybrid(&h, a),
        measure_capsule_hybrid(&h, a),
        "measurement must be deterministic"
    );
    let b: &[u8] = b"the same artifact bytes, measured twicE";
    assert_ne!(
        measure_capsule_hybrid(&h, a),
        measure_capsule_hybrid(&h, b),
        "a one bit difference must change the measurement"
    );
}

#[test]
fn the_two_schemes_are_domain_separated() {
    let h = hasher();
    for img in [b"".as_slice(), b"x".as_slice(), b"a longer artifact body".as_slice()] {
        assert_ne!(
            measure_capsule(&h, img),
            measure_capsule_hybrid(&h, img),
            "an image must not measure to the same leaf under both schemes"
        );
    }
}

#[test]
fn a_hybrid_set_does_not_verify_against_a_direct_root() {
    let h = hasher();
    let a: &[u8] = b"artifact one";
    let b: &[u8] = b"artifact two";
    let imgs = [a, b];

    let direct = MeasuredSet::commit(&h, &padded(&imgs));
    let hybrid = MeasuredSet::commit_hybrid(&h, &padded(&imgs));
    assert_ne!(
        root_bytes(direct.root()),
        root_bytes(hybrid.root()),
        "the two schemes must not share a root"
    );

    // a trailer proven under the hybrid set must not verify under the direct
    // root: changing how a set is measured invalidates the set
    let mut ctx = Vec::new();
    ctx.extend_from_slice(blake3::hash(a).as_bytes());
    ctx.extend_from_slice(&[0x19]);
    let trailer = build_attestation_trailer_from_set(
        &h,
        LOG_ROUNDS,
        &hybrid,
        0,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    );
    assert!(gate(&root_bytes(hybrid.root()), &trailer, &ctx), "the hybrid proof must verify");
    assert!(
        !gate(&root_bytes(direct.root()), &trailer, &ctx),
        "a hybrid proof must not verify against a direct root"
    );
}

#[test]
fn the_hybrid_path_proves_membership_and_refuses_tampering() {
    let h = hasher();
    let a: &[u8] = b"artifact alpha, enrolled";
    let b: &[u8] = b"artifact beta, enrolled";
    let set = MeasuredSet::commit_hybrid(&h, &padded(&[a, b]));
    let root = root_bytes(set.root());

    let mut ctx = Vec::new();
    ctx.extend_from_slice(blake3::hash(a).as_bytes());
    ctx.extend_from_slice(&[0x19]);
    let trailer = build_attestation_trailer_from_set(
        &h,
        LOG_ROUNDS,
        &set,
        0,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    );
    assert!(gate(&root, &trailer, &ctx), "an honest hybrid membership must verify");

    // the same forgeries the direct path refuses
    let mut swapped = Vec::new();
    swapped.extend_from_slice(blake3::hash(b"artifact alpha, TAMPERED").as_bytes());
    swapped.extend_from_slice(&[0x19]);
    assert!(!gate(&root, &trailer, &swapped), "a swapped artifact must not verify");

    let mut escalated = ctx.clone();
    let last = escalated.len() - 1;
    escalated[last] ^= 0xFF;
    assert!(!gate(&root, &trailer, &escalated), "a tampered context must not verify");
}
