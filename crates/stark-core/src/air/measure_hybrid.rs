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

//! The hybrid measurement: BLAKE3, then one Poseidon permutation.

use super::super::field::Fp;
use super::poseidon::{Poseidon, RATE, WIDTH};

/// The domain separator for the hybrid measurement, absorbed before the digest
/// so a hybrid leaf can never equal a direct leaf of the same bytes.
const HYBRID_DOMAIN: u64 = 0x4E4F_4E4F_5342_3348; // "NONOSB3H"

/// The hybrid measurement: BLAKE3 the image, then absorb only the 32-byte
/// digest into the sponge. This is what a caller should use to measure an
/// artifact of any real size.
///
/// The direct measurement in `measure.rs` absorbs seven bytes per lane and permutes every
/// rate group, so it runs at about three megabytes a second: one permutation
/// per 28 bytes of artifact. BLAKE3 reads the same bytes three orders of
/// magnitude faster, and absorbing its 32-byte output costs a single
/// permutation regardless of image size. Measured on the crate's own study, the
/// hybrid reaches BLAKE3's own throughput and beats the direct path by more
/// than a thousand times at a megabyte.
///
/// The security question this raises is worth stating rather than assuming. The
/// hybrid binds a BLAKE3 digest where the direct path binds the bytes, so it
/// rests on BLAKE3 collision resistance. For an attestation whose proof context
/// already carries the artifact's BLAKE3 measurement, that assumption is
/// already load bearing and the hybrid removes redundant work rather than
/// adding a hypothesis. A caller whose context does not already commit to a
/// general purpose digest is making a different trade and should keep the
/// direct measurement.
///
/// The length is bound by BLAKE3 itself, and the domain separator keeps the two
/// measurement schemes disjoint: no image measures to the same leaf under both.
pub fn measure_capsule_hybrid(hasher: &Poseidon, image: &[u8]) -> [Fp; RATE] {
    measure_digest_hybrid(hasher, blake3::hash(image).as_bytes())
}

/// The same leaf from the image's BLAKE3 digest alone, for a verifier that
/// holds the digest a context carries rather than the image.
pub fn measure_digest_hybrid(hasher: &Poseidon, bytes: &[u8; 32]) -> [Fp; RATE] {
    let mut state = [Fp::ZERO; WIDTH];
    state[0] = state[0] + Fp::from_u64(HYBRID_DOMAIN);
    // The 32-byte digest is four canonical field elements: each eight-byte
    // group is reduced, which is exact because 2^64 - p is smaller than p.
    for (lane, word) in bytes.chunks(8).enumerate() {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(word);
        state[lane + 1] = state[lane + 1] + Fp::from_u64(u64::from_le_bytes(buf));
    }
    state = hasher.permute(state);

    let mut out = [Fp::ZERO; RATE];
    out.copy_from_slice(&state[..RATE]);
    out
}
