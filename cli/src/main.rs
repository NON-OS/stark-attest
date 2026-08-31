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

//! Attest a set of artifacts under one 32-byte root; verify any member alone.
//!
//! Each spec is `context:path:trailer`, where context is hex bytes the caller
//! chooses (an epoch, a role, a capability mask, anything the proof should be
//! bound to besides the bytes themselves). The proof context is the BLAKE3
//! measurement of the file followed by those bytes, so a swapped artifact and
//! a tampered context fail the same verification. Extracted from the NONOS
//! build, where the identical construction gates every process spawn.

use std::env;
use std::fs;
use std::process::exit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use nonos_stark::air::{
    build_attestation_trailer_from_set, deserialize_proof_ext, stark_verify_ext_blown_bound,
    MeasuredSet, MerkleMembership, Poseidon, RATE,
};
use nonos_stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use nonos_stark::field::Fp;

/// Fixed tree depth: up to 256 members per set. A parameterized depth is the
/// first planned extension; the depth is carried in every trailer, so old
/// trailers stay verifiable when it lands.
const TREE_DEPTH: usize = 8;
const LEAVES: usize = 1 << TREE_DEPTH;
const MAGIC: &[u8; 8] = b"NZKSTRK1";

/// Padding for unused slots. Begins with a byte no ELF, archive, or text file
/// starts with a domain tag after, so no real artifact measures to a pad leaf.
const PAD_IMAGE: &[u8] = b"\x00STARK-ATTEST-RESERVED-SLOT-v1";

fn context_for(file_bytes: &[u8], caller_ctx: &[u8]) -> Vec<u8> {
    let mut ctx = Vec::with_capacity(32 + caller_ctx.len());
    ctx.extend_from_slice(blake3::hash(file_bytes).as_bytes());
    ctx.extend_from_slice(caller_ctx);
    ctx
}

fn to_rate(bytes: &[u8]) -> [Fp; RATE] {
    let mut out = [Fp::ZERO; RATE];
    for (i, lane) in out.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&bytes[i * 8..i * 8 + 8]);
        *lane = Fp::from_u64(u64::from_le_bytes(w));
    }
    out
}

fn root_to_bytes(root: [Fp; RATE]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, lane) in root.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    out
}

fn padded<'a>(images: &[&'a [u8]]) -> Vec<&'a [u8]> {
    assert!(images.len() <= LEAVES, "more artifacts than tree slots");
    let mut v: Vec<&[u8]> = images.to_vec();
    while v.len() < LEAVES {
        v.push(PAD_IMAGE);
    }
    v
}

/// The exact parse-and-verify a consumer runs. Kept byte-compatible with the
/// NONOS spawn gate so a trailer made by either tool verifies in both worlds.
fn gate_verify(root_bytes: &[u8; 32], trailer: &[u8], context: &[u8]) -> bool {
    let dir_bytes = TREE_DEPTH.div_ceil(8);
    let sib_end = 9 + TREE_DEPTH * 32;
    if trailer.len() < sib_end + dir_bytes
        || &trailer[0..8] != MAGIC
        || trailer[8] as usize != TREE_DEPTH
    {
        return false;
    }
    let mut siblings = Vec::with_capacity(TREE_DEPTH);
    for i in 0..TREE_DEPTH {
        siblings.push(to_rate(&trailer[9 + i * 32..9 + i * 32 + 32]));
    }
    let dirs = &trailer[sib_end..sib_end + dir_bytes];
    let directions: Vec<bool> =
        (0..TREE_DEPTH).map(|i| (dirs[i / 8] >> (i % 8)) & 1 == 1).collect();
    let Some(proof) = deserialize_proof_ext(&trailer[sib_end + dir_bytes..]) else {
        return false;
    };
    let air = MerkleMembership::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        to_rate(root_bytes),
        siblings,
        directions,
    );
    stark_verify_ext_blown_bound(&air, &proof, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS, context)
}

struct Spec {
    ctx: Vec<u8>,
    path: String,
    trailer: String,
}

fn parse_specs(raw: &[String]) -> Vec<Spec> {
    raw.iter()
        .map(|spec| {
            let parts: Vec<&str> = spec.splitn(3, ':').collect();
            if parts.len() != 3 {
                eprintln!("bad spec {spec}, want contexthex:path:trailer");
                exit(1);
            }
            let ctx = if parts[0].is_empty() {
                Vec::new()
            } else {
                hex_decode(parts[0]).unwrap_or_else(|| {
                    eprintln!("bad context hex {}", parts[0]);
                    exit(1)
                })
            };
            Spec { ctx, path: parts[1].to_string(), trailer: parts[2].to_string() }
        })
        .collect()
}

fn enroll(root_out: &str, specs: &[Spec]) {
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let images: Vec<Vec<u8>> = specs.iter().map(|s| read(&s.path)).collect();
    let contexts: Vec<Vec<u8>> =
        images.iter().zip(specs).map(|(img, s)| context_for(img, &s.ctx)).collect();
    let refs: Vec<&[u8]> = images.iter().map(Vec::as_slice).collect();
    let set = MeasuredSet::commit(&hasher, &padded(&refs));
    let root = root_to_bytes(set.root());

    let n = specs.len();
    let counter = AtomicUsize::new(0);
    let workers = thread::available_parallelism().map(|v| v.get()).unwrap_or(1).min(n).max(1);
    let collected: Vec<Vec<(usize, Vec<u8>)>> = thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                scope.spawn(|| {
                    let mut local = Vec::new();
                    loop {
                        let i = counter.fetch_add(1, Ordering::Relaxed);
                        if i >= n {
                            break;
                        }
                        let trailer = build_attestation_trailer_from_set(
                            &hasher,
                            LOG_ROUNDS,
                            &set,
                            i,
                            &contexts[i],
                            N_QUERIES,
                            GRIND_BITS,
                            EXTRA_BLOWUP_BITS,
                        );
                        if !gate_verify(&root, &trailer, &contexts[i]) {
                            eprintln!("enroll: trailer {i} failed the gate self-check");
                            exit(2);
                        }
                        local.push((i, trailer));
                    }
                    local
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    for local in collected {
        for (i, trailer) in local {
            write(&specs[i].trailer, &trailer);
        }
    }
    write(root_out, &root);
    println!("enrolled {n} artifacts under root {}", hex(&root));
}

fn verify(root_path: &str, specs: &[Spec]) {
    let started = std::time::Instant::now();
    let root_bytes = read(root_path);
    if root_bytes.len() != 32 {
        eprintln!("root {root_path} is {} bytes, want 32", root_bytes.len());
        exit(1);
    }
    let mut root = [0u8; 32];
    root.copy_from_slice(&root_bytes);
    let mut failed = 0usize;
    for s in specs {
        let image = read(&s.path);
        let trailer = read(&s.trailer);
        let ctx = context_for(&image, &s.ctx);
        if gate_verify(&root, &trailer, &ctx) {
            println!("  ok    {}", s.path);
        } else {
            println!("  FAIL  {}", s.path);
            failed += 1;
        }
    }
    if failed > 0 {
        eprintln!("{failed} of {} proofs failed under root {}", specs.len(), hex(&root));
        exit(1);
    }
    println!(
        "verified {} proofs under root {} ({:.2?})",
        specs.len(),
        hex(&root),
        started.elapsed()
    );
}

fn selftest() {
    let dir = std::env::temp_dir().join("stark-attest-selftest");
    let _ = fs::create_dir_all(&dir);
    let a = dir.join("a.bin");
    let b = dir.join("b.bin");
    fs::write(&a, b"artifact-a-content").unwrap();
    fs::write(&b, b"artifact-b-content").unwrap();
    let ta = dir.join("a.trailer");
    let tb = dir.join("b.trailer");
    let rootp = dir.join("root.bin");
    let specs = vec![
        Spec { ctx: vec![0xAA], path: a.display().to_string(), trailer: ta.display().to_string() },
        Spec { ctx: vec![0xBB], path: b.display().to_string(), trailer: tb.display().to_string() },
    ];
    enroll(&rootp.display().to_string(), &specs);
    verify(&rootp.display().to_string(), &specs);
    // a swapped artifact must fail
    fs::write(&a, b"artifact-a-TAMPERED").unwrap();
    let root = read(&rootp.display().to_string());
    let mut r = [0u8; 32];
    r.copy_from_slice(&root);
    let img = read(&specs[0].path);
    let ctx = context_for(&img, &specs[0].ctx);
    if gate_verify(&r, &read(&specs[0].trailer), &ctx) {
        eprintln!("selftest: tampered artifact verified; refusing to exist");
        exit(2);
    }
    // a tampered context must fail
    fs::write(&a, b"artifact-a-content").unwrap();
    let img = read(&specs[0].path);
    let bad_ctx = context_for(&img, &[0xAB]);
    if gate_verify(&r, &read(&specs[0].trailer), &bad_ctx) {
        eprintln!("selftest: tampered context verified; refusing to exist");
        exit(2);
    }
    println!("selftest passed: honest verifies, tampered artifact and context both refused");
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len() / 2).map(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()).collect()
}

fn read(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|e| {
        eprintln!("read {path}: {e}");
        exit(1)
    })
}

fn write(path: &str, bytes: &[u8]) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(path, bytes).unwrap_or_else(|e| {
        eprintln!("write {path}: {e}");
        exit(1)
    })
}

fn usage() -> ! {
    eprintln!(
        "stark-attest: one 32-byte statement over a set of artifacts\n\n\
         usage:\n  \
         stark-attest selftest\n  \
         stark-attest enroll <root.bin> <contexthex:path:trailer-out> ...\n  \
         stark-attest verify <root.bin> <contexthex:path:trailer> ...\n\n\
         context is caller-chosen hex (may be empty), bound into each proof\n\
         alongside the artifact's BLAKE3 measurement."
    );
    exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("selftest") => selftest(),
        Some("enroll") if args.len() >= 4 => enroll(&args[2], &parse_specs(&args[3..])),
        Some("verify") if args.len() >= 4 => verify(&args[2], &parse_specs(&args[3..])),
        _ => usage(),
    }
}
