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

//! The public-leaf gate over a BLAKE3 digest. `verify_public_trailer` hashes
//! the image it is given and comes here; the kernel and the bootloader do that.

use super::attest_public::{to_rate, PUBLIC_TRAILER_MAGIC};
use super::measure_hybrid::measure_digest_hybrid;
use super::poseidon::{Poseidon, RATE};
use super::{deserialize_proof_ext, stark_verify_ext_blown_bound, MultiMembership, Opening};
use crate::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use crate::field::Fp;
use alloc::vec::Vec;

/// The same check from the image's BLAKE3 digest, for a verifier that holds
/// the digest rather than the bytes. It proves the member with that digest is
/// enrolled; binding the digest to an artifact is the caller's to do.
#[must_use = "an image must not run unless its attestation verifies"]
pub fn verify_public_trailer_digest(
    root: &[u8; 32],
    depth: usize,
    digest: &[u8; 32],
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
    // The leaf comes from the measurement, never from the trailer.
    let leaf = measure_digest_hybrid(&hasher, digest);
    let opening = Opening { leaf, root: to_rate(root), siblings, directions };
    let air = MultiMembership::new(hasher, LOG_ROUNDS, alloc::vec![opening]);
    stark_verify_ext_blown_bound(&air, &proof, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS, context)
}
