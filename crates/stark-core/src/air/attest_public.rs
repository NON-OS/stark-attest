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

pub use super::attest_public_digest::verify_public_trailer_digest;
use super::poseidon::RATE;
use crate::field::Fp;

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
    verify_public_trailer_digest(root, depth, blake3::hash(image).as_bytes(), trailer, context)
}
