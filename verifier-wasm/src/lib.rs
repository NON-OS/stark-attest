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

//! The attestation gate, compiled for a browser tab.
//!
//! This is not a demo verifier or a reimplementation: it is the same
//! `verify_attestation_trailer` the NONOS kernel runs at every process spawn
//! and the bootloader runs before jumping to the kernel, compiled to
//! WebAssembly. A visitor who verifies a release trailer here has run the
//! deployment's own gate against the deployment's own bytes, in their own
//! machine, with no trust placed in the page that served it beyond the wasm
//! being this crate; and the wasm is reproducible from this source.
//!
//! The interface is deliberately primitive: raw exported functions and linear
//! memory, no binding framework. Three exports, each total:
//!
//!   `wasm_alloc(len)`   reserve `len` bytes, returns a pointer
//!   `wasm_free(ptr, len)` release a reservation
//!   `verify(root, trailer, trailer_len, ctx, ctx_len)` run the gate,
//!                       1 = the proof verifies, 0 = it does not
//!
//! Nothing panics on any input: the gate is a total parse (a fact proven in
//! `lean/Zkolang/Trailer.lean`) and this wrapper adds only bounds-checked
//! slices over memory the caller allocated through `wasm_alloc`.

use nonos_stark::air::verify_attestation_trailer;
use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::attest_params::{LOG_ROUNDS, N_QUERIES};
use nonos_stark::field::Fp;

use std::alloc::{alloc, dealloc, Layout};

/// Reserve `len` bytes of linear memory for the caller to fill.
///
/// # Safety
/// The returned pointer is valid for exactly `len` bytes until passed to
/// `wasm_free` with the same length.
#[no_mangle]
pub extern "C" fn wasm_alloc(len: usize) -> *mut u8 {
    if len == 0 {
        return core::ptr::null_mut();
    }
    let Ok(layout) = Layout::array::<u8>(len) else {
        return core::ptr::null_mut();
    };
    unsafe { alloc(layout) }
}

/// Release a reservation made by `wasm_alloc`.
///
/// # Safety
/// `ptr` must come from `wasm_alloc(len)` and not have been freed already.
#[no_mangle]
pub unsafe extern "C" fn wasm_free(ptr: *mut u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    let Ok(layout) = Layout::array::<u8>(len) else {
        return;
    };
    dealloc(ptr, layout);
}

/// Run the gate: does `trailer` prove membership of a measurement under
/// `root`, bound to `context`? Returns 1 for a verifying proof, 0 for
/// anything else, including malformed input of any shape.
///
/// # Safety
/// `root` must point to 32 readable bytes; `trailer` and `ctx` to
/// `trailer_len` and `ctx_len` readable bytes. The page's own glue upholds
/// this by only ever passing pointers from `wasm_alloc`.
#[no_mangle]
pub unsafe extern "C" fn verify(
    root: *const u8,
    trailer: *const u8,
    trailer_len: usize,
    ctx: *const u8,
    ctx_len: usize,
) -> u32 {
    if root.is_null() || trailer.is_null() || ctx.is_null() {
        return 0;
    }
    let root = core::slice::from_raw_parts(root, 32);
    let trailer = core::slice::from_raw_parts(trailer, trailer_len);
    let context = core::slice::from_raw_parts(ctx, ctx_len);

    let mut root_fp = [Fp::ZERO; RATE];
    for (i, lane) in root_fp.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&root[i * 8..i * 8 + 8]);
        let v = u64::from_le_bytes(w);
        // a root lane past the field modulus is not a root
        if v >= 0xFFFF_FFFF_0000_0001 {
            return 0;
        }
        *lane = Fp::from_u64(v);
    }

    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    u32::from(verify_attestation_trailer(&hasher, LOG_ROUNDS, root_fp, N_QUERIES, trailer, context))
}

/// Fold a complete leaf set to its Merkle root, in the tree the deployment
/// uses. `leaves` is `count` digests of `RATE` little-endian u64 lanes; the
/// 32-byte root is written to `out`. Returns 1 on success, 0 on any
/// malformed input.
///
/// This is what lets a browser reconstruct the policy root from the full
/// enrolled set rather than trust the served copy: a membership proof shows
/// one member, the fold shows there are no others.
///
/// # Safety
/// `leaves` must point to `count * 32` readable bytes and `out` to 32
/// writable bytes, both from `wasm_alloc`.
#[no_mangle]
pub unsafe extern "C" fn fold_root(leaves: *const u8, count: usize, out: *mut u8) -> u32 {
    use nonos_stark::poseidon_merkle::PoseidonMerkleTree;
    if leaves.is_null() || out.is_null() || count == 0 || !count.is_power_of_two() {
        return 0;
    }
    let bytes = core::slice::from_raw_parts(leaves, count * 32);
    let mut parsed = Vec::with_capacity(count);
    for chunk in bytes.chunks_exact(32) {
        let mut leaf = [Fp::ZERO; RATE];
        for (i, lane) in leaf.iter_mut().enumerate() {
            let mut w = [0u8; 8];
            w.copy_from_slice(&chunk[i * 8..i * 8 + 8]);
            let v = u64::from_le_bytes(w);
            if v >= 0xFFFF_FFFF_0000_0001 {
                return 0;
            }
            *lane = Fp::from_u64(v);
        }
        parsed.push(leaf);
    }
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let root = PoseidonMerkleTree::commit(&hasher, &parsed).root();
    let o = core::slice::from_raw_parts_mut(out, 32);
    for (i, lane) in root.iter().enumerate() {
        o[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    1
}

/// The measurement of the deployment's reserved-slot image, written to `out`.
/// A page reconstructing a policy root needs this to check that every
/// non-member slot really is the reserved pad and not a member in disguise;
/// without that check, "the fold matches" only proves consistency with the
/// served leaves, not that the set is complete.
///
/// # Safety
/// `out` must point to 32 writable bytes from `wasm_alloc`.
#[no_mangle]
pub unsafe extern "C" fn reserved_leaf(out: *mut u8) -> u32 {
    use nonos_stark::air::measure_capsule;
    if out.is_null() {
        return 0;
    }
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let leaf = measure_capsule(&hasher, b"\x00NONOS-POLICY-RESERVED-SLOT-v1");
    let o = core::slice::from_raw_parts_mut(out, 32);
    for (i, lane) in leaf.iter().enumerate() {
        o[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    1
}

/// BLAKE3 of `len` bytes at `data`, written to `out`. Exposed so a page can
/// measure a multi-megabyte boot artifact at native speed and bind the digest
/// it just computed, rather than one it was told.
///
/// # Safety
/// `data` must point to `len` readable bytes, `out` to 32 writable bytes.
#[no_mangle]
pub unsafe extern "C" fn blake3_hash(data: *const u8, len: usize, out: *mut u8) -> u32 {
    if data.is_null() || out.is_null() {
        return 0;
    }
    let bytes = core::slice::from_raw_parts(data, len);
    let digest = blake3::hash(bytes);
    core::slice::from_raw_parts_mut(out, 32).copy_from_slice(digest.as_bytes());
    1
}
