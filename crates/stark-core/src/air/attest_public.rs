// NONOS Operating System
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

//! The attestation gate with a public leaf, the one verify every attested image
//! goes through. The verifier measures the image itself and pins it as the
//! opening's leaf, so a proof shows that exactly this image sits under the root.
//! The private-leaf form proved only that some enrolled leaf exists, which any
//! enrolled image's public path satisfies for a context the prover chooses.

use super::measure::measure_capsule_hybrid;
use super::poseidon::{Poseidon, RATE};
use super::{deserialize_proof_ext, stark_verify_ext_blown_bound, MultiMembership, Opening};
use crate::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use crate::field::Fp;
use alloc::vec::Vec;

/// A new magic, so a private-leaf trailer is refused rather than reinterpreted.
pub const PUBLIC_TRAILER_MAGIC: &[u8; 8] = b"NZKSTRK2";

pub(super) fn to_rate(bytes: &[u8]) -> [Fp; RATE] {
    let mut out = [Fp::ZERO; RATE];
    for (lane, word) in out.iter_mut().zip(bytes.chunks_exact(8)) {
        let mut w = [0u8; 8];
        w.copy_from_slice(word);
        *lane = Fp::from_u64(u64::from_le_bytes(w));
    }
    out
}

/// True only when `trailer` proves that the measurement of `image` is a leaf of
/// the `depth`-level tree under `root`, bound to `context`. Any malformed byte
/// is a refusal, never a panic.
#[must_use = "an image must not run unless its attestation verifies"]
pub fn verify_public_trailer(
    root: &[u8; 32],
    depth: usize,
    image: &[u8],
    trailer: &[u8],
    context: &[u8],
) -> bool {
    let dir_bytes = depth.div_ceil(8);
    let sib_end = 9 + depth * 32;
    if depth == 0
        || trailer.len() < sib_end + dir_bytes
        || &trailer[0..8] != PUBLIC_TRAILER_MAGIC
        || trailer[8] as usize != depth
    {
        return false;
    }
    let siblings: Vec<[Fp; RATE]> = trailer[9..sib_end].chunks_exact(32).map(to_rate).collect();
    let dirs = &trailer[sib_end..sib_end + dir_bytes];
    let directions: Vec<bool> = (0..depth).map(|i| (dirs[i / 8] >> (i % 8)) & 1 == 1).collect();
    let Some(proof) = deserialize_proof_ext(&trailer[sib_end + dir_bytes..]) else {
        return false;
    };
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    // The leaf comes from the bytes about to run, never from the trailer.
    let leaf = measure_capsule_hybrid(&hasher, image);
    let opening = Opening { leaf, root: to_rate(root), siblings, directions };
    let air = MultiMembership::new(hasher, LOG_ROUNDS, alloc::vec![opening]);
    stark_verify_ext_blown_bound(&air, &proof, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS, context)
}
