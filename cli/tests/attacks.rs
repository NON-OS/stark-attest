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

//! Adversarial suite. A verifier is only worth its rejections, so this suite is
//! all rejections: every test enrolls an honest set, then mounts one specific
//! forgery and asserts it fails. A passing suite means each attack modeled here
//! is defeated by the construction, not by luck. Positive verification is
//! covered by the CLI selftest; nothing here should ever print "ok".
//!
//! The attacks are built by calling the same library the CLI does, so they
//! exercise the real gate, not a reimplementation.

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

fn ctx(bytes: &[u8], caller: &[u8]) -> Vec<u8> {
    let mut c = Vec::new();
    c.extend_from_slice(blake3::hash(bytes).as_bytes());
    c.extend_from_slice(caller);
    c
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
fn padded<'a>(imgs: &[&'a [u8]]) -> Vec<&'a [u8]> {
    let mut v: Vec<&[u8]> = imgs.to_vec();
    while v.len() < LEAVES {
        v.push(PAD);
    }
    v
}

/// The exact gate the consumers run.
fn gate(root: &[u8; 32], trailer: &[u8], context: &[u8]) -> bool {
    let dir_bytes = DEPTH.div_ceil(8);
    let sib_end = 9 + DEPTH * 32;
    if trailer.len() < sib_end + dir_bytes
        || &trailer[0..8] != MAGIC
        || trailer[8] as usize != DEPTH
    {
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
    let air =
        MerkleMembership::new(Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]), LOG_ROUNDS, to_rate(root), sib, d);
    stark_verify_ext_blown_bound(&air, &proof, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS, context)
}

/// Enroll two members; return (root, member-0 trailer, its context).
fn honest() -> ([u8; 32], Vec<u8>, Vec<u8>) {
    let h = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let a: &[u8] = b"artifact-alpha-genuine-bytes";
    let b: &[u8] = b"artifact-beta-genuine-bytes";
    let imgs = [a, b];
    let set = MeasuredSet::commit(&h, &padded(&imgs));
    let root = root_bytes(set.root());
    let context = ctx(a, &[0x19]);
    let trailer =
        build_attestation_trailer_from_set(&h, LOG_ROUNDS, &set, 0, &context, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS);
    assert!(gate(&root, &trailer, &context), "honest baseline must verify");
    (root, trailer, context)
}

#[test]
fn swapped_artifact_is_rejected() {
    let (root, trailer, _) = honest();
    // verify the genuine trailer against a different artifact's measurement
    let forged_ctx = ctx(b"artifact-alpha-TAMPERED-bytes", &[0x19]);
    assert!(!gate(&root, &trailer, &forged_ctx), "a swapped artifact must not verify");
}

#[test]
fn tampered_context_is_rejected() {
    let (root, trailer, mut context) = honest();
    // flip the caller byte (a capability escalation, in the OS deployment)
    let last = context.len() - 1;
    context[last] ^= 0xFF;
    assert!(!gate(&root, &trailer, &context), "a tampered context must not verify");
}

#[test]
fn wrong_root_is_rejected() {
    let (mut root, trailer, context) = honest();
    root[0] ^= 0x01;
    assert!(!gate(&root, &trailer, &context), "a proof must not verify under a different root");
}

#[test]
fn non_member_has_no_trailer() {
    // an outsider cannot produce a passing trailer for a set it is not in:
    // reuse a member's trailer for a non-member context and root, all fail
    let (root, trailer, _) = honest();
    let outsider = ctx(b"artifact-gamma-never-enrolled", &[0x19]);
    assert!(!gate(&root, &trailer, &outsider), "a non-member must not verify");
}

#[test]
fn truncated_trailer_is_rejected() {
    let (root, trailer, context) = honest();
    for cut in [0usize, 7, 8, 9, trailer.len() / 2, trailer.len() - 1] {
        assert!(!gate(&root, &trailer[..cut], &context), "a truncated trailer must not verify (cut {cut})");
    }
}

#[test]
fn corrupted_proof_bytes_are_rejected() {
    let (root, trailer, context) = honest();
    // flip one bit in the proof body (past the sibling region) at several spots
    let sib_end = 9 + DEPTH * 32 + DEPTH.div_ceil(8);
    for off in [sib_end, sib_end + 32, trailer.len() - 8, trailer.len() - 1] {
        let mut t = trailer.clone();
        t[off] ^= 0x01;
        assert!(!gate(&root, &t, &context), "a corrupted proof must not verify (offset {off})");
    }
}

#[test]
fn corrupted_sibling_is_rejected() {
    let (root, trailer, context) = honest();
    let mut t = trailer.clone();
    t[9] ^= 0x01; // first byte of the first sibling digest
    assert!(!gate(&root, &t, &context), "a corrupted authentication path must not verify");
}

#[test]
fn flipped_direction_bit_is_rejected() {
    let (root, trailer, context) = honest();
    let sib_end = 9 + DEPTH * 32;
    let mut t = trailer.clone();
    t[sib_end] ^= 0x01; // flip a path direction
    assert!(!gate(&root, &t, &context), "a flipped path direction must not verify");
}

#[test]
fn bad_magic_is_rejected() {
    let (root, trailer, context) = honest();
    let mut t = trailer.clone();
    t[0] ^= 0x01;
    assert!(!gate(&root, &t, &context), "a trailer with wrong magic must not verify");
}

#[test]
fn non_canonical_sibling_is_rejected() {
    // a sibling field element at or above p must be refused by the parse
    let (root, trailer, context) = honest();
    let mut t = trailer.clone();
    // write p itself (0xFFFFFFFF00000001) little-endian into the first sibling
    let p = 0xFFFF_FFFF_0000_0001u64.to_le_bytes();
    t[9..9 + 8].copy_from_slice(&p);
    assert!(!gate(&root, &t, &context), "a non-canonical field element must not verify");
}
