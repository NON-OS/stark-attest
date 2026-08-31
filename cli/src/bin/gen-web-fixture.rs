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

//! Enroll real artifacts and emit the browser page's fixture: the root, the
//! trailer, and the context, as the raw bytes the wasm gate consumes.
//!
//! The intended first argument is the compiled wasm verifier itself, so the
//! page proves a statement about the very binary the visitor is running: the
//! wasm that verifies the trailer is the member the trailer attests. There is
//! no circularity in that, because the trailer lives beside the wasm rather
//! than inside it; the wasm's bytes are fixed before the proof over them is
//! ground.
//!
//! Run: `gen-web-fixture <out-dir> <artifact> [artifact..]`

use std::env;
use std::fs;

use nonos_stark::air::{build_attestation_trailer_from_set, MeasuredSet, Poseidon, RATE};
use nonos_stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use nonos_stark::field::Fp;

const LEAVES: usize = 256;
const PAD: &[u8] = b"\x00STARK-ATTEST-RESERVED-SLOT-v1";
/// The caller byte bound into the context, mirroring the OS deployment's
/// capability mask position.
const CALLER: u8 = 0x19;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: gen-web-fixture <out-dir> <artifact> [artifact..]");
        std::process::exit(2);
    }
    let out = &args[1];
    fs::create_dir_all(out).expect("create the output directory");

    let images: Vec<Vec<u8>> =
        args[2..].iter().map(|p| fs::read(p).unwrap_or_else(|e| panic!("read {p}: {e}"))).collect();
    for (path, img) in args[2..].iter().zip(&images) {
        assert!(!img.is_empty() && img[0] != 0, "{path} is not an admissible artifact");
    }

    let refs: Vec<&[u8]> = images.iter().map(Vec::as_slice).collect();
    let mut padded = refs.clone();
    while padded.len() < LEAVES {
        padded.push(PAD);
    }

    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let set = MeasuredSet::commit(&hasher, &padded);

    let mut root = [0u8; 32];
    for (i, lane) in set.root().iter().enumerate() {
        root[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    fs::write(format!("{out}/root.bin"), root).expect("write the root");

    // member zero is the artifact the page is about; the rest are enrolled
    // alongside it, as a release enrolls a whole set
    let mut ctx = Vec::new();
    ctx.extend_from_slice(blake3::hash(&images[0]).as_bytes());
    ctx.push(CALLER);
    fs::write(format!("{out}/context.bin"), &ctx).expect("write the context");

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
    fs::write(format!("{out}/trailer.bin"), &trailer).expect("write the trailer");

    println!("enrolled {} artifact(s) into a {LEAVES}-slot set", images.len());
    println!("root     {}", root.iter().map(|b| format!("{b:02x}")).collect::<String>());
    println!("trailer  {} bytes", trailer.len());
    println!("context  BLAKE3(member 0) plus caller byte {CALLER:#04x}");
}
