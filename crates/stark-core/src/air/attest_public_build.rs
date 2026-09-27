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

//! The prover side of the public-leaf gate. The set must be committed with
//! `MeasuredSet::commit_hybrid`, the measurement the verifier recomputes; a set
//! committed any other way yields trailers that never verify.

use super::attest_build::MeasuredSet;
use super::attest_public::PUBLIC_TRAILER_MAGIC;
use super::poseidon::{Poseidon, RATE};
use super::prove_ext::stark_prove_ext_blown_bound;
use super::serialize_ext::serialize_proof_ext;
use super::{MultiMembership, Opening};
use crate::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use crate::field::Fp;
use alloc::vec::Vec;

/// The trailer for slot `index` of `set`, bound to `context`, or `None` when
/// the slot does not exist. Same layout as the private-leaf trailer: magic,
/// depth, sibling path, direction bits, proof.
pub fn build_public_trailer(set: &MeasuredSet, index: usize, context: &[u8]) -> Option<Vec<u8>> {
    let leaf = set.leaf(index)?;
    let siblings = set.path(index)?;
    let depth = siblings.len();
    let directions: Vec<bool> = (0..depth).map(|k| (index >> k) & 1 == 1).collect();
    let mut out = Vec::new();
    out.extend_from_slice(PUBLIC_TRAILER_MAGIC);
    out.push(u8::try_from(depth).ok()?);
    for node in &siblings {
        for lane in node {
            out.extend_from_slice(&lane.value().to_le_bytes());
        }
    }
    let mut dirs = alloc::vec![0u8; depth.div_ceil(8)];
    for (k, d) in directions.iter().enumerate() {
        if *d {
            dirs[k / 8] |= 1 << (k % 8);
        }
    }
    out.extend_from_slice(&dirs);
    let opening = Opening { leaf, root: set.root(), siblings, directions };
    let proof = prove(opening, context);
    out.extend_from_slice(&serialize_proof_ext(&proof));
    Some(out)
}

/// Prove one opening exactly as the verifier will check it. Public so a test
/// can prove a leaf other than the image's and show the gate refuses it.
pub fn prove_public_opening(opening: Opening, context: &[u8]) -> Vec<u8> {
    serialize_proof_ext(&prove(opening, context))
}

fn prove(opening: Opening, context: &[u8]) -> super::StarkProofExt {
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let air = MultiMembership::new(hasher, LOG_ROUNDS, alloc::vec![opening]);
    let trace = air.trace();
    stark_prove_ext_blown_bound(&air, &trace, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS, context)
}
