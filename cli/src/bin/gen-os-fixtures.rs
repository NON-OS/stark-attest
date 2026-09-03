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

//! Assemble the browser fixture for a whole attested operating system: the
//! kernel and every enrolled capsule, each with the exact context the running
//! system binds, each verified against the root the system enforces before the
//! bytes are written out.
//!
//! This is not a demo fixture. Every trailer here is the one shipped with the
//! image, every measurement is the BLAKE3 of the real binary, every capability
//! mask is the one the capsule was granted, and every proof is re-verified with
//! the same gate the kernel runs at spawn before it is emitted. A capsule whose
//! proof does not verify is dropped and reported, not shipped.
//!
//! Usage:
//!   gen-os-fixtures <out-dir> \
//!     --policy-root <bin> --epoch <n> \
//!     --kernel-elf <bin> --kernel-root <bin> --kernel-trailer <bin> \
//!     --capsules <tsv>
//!
//! The TSV is `slug\thandle\tcaps_hex\telf_path\ttrailer_path` per line.

use std::collections::BTreeMap;
use std::env;
use std::fs;

use nonos_stark::air::{measure_capsule, verify_attestation_trailer, Poseidon, RATE};
use nonos_stark::attest_params::{LOG_ROUNDS, N_QUERIES};
use nonos_stark::field::Fp;
use nonos_stark::poseidon_merkle::PoseidonMerkleTree;

fn root_from_bytes(b: &[u8]) -> [Fp; RATE] {
    let mut r = [Fp::ZERO; RATE];
    for (i, lane) in r.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&b[i * 8..i * 8 + 8]);
        *lane = Fp::from_u64(u64::from_le_bytes(w));
    }
    r
}

fn hexs(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn arg(args: &[String], key: &str) -> Option<String> {
    args.iter().position(|a| a == key).and_then(|i| args.get(i + 1).cloned())
}

struct Entry {
    slug: String,
    handle: String,
    kind: &'static str,
    caps: u64,
    measurement: [u8; 32],
    trailer_len: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let out = args.get(1).cloned().expect("out dir");
    fs::create_dir_all(&out).expect("mkdir out");

    let epoch: u64 = arg(&args, "--epoch").and_then(|s| s.parse().ok()).unwrap_or(1);
    let policy_root_bytes = fs::read(arg(&args, "--policy-root").expect("--policy-root")).unwrap();
    let policy_root = root_from_bytes(&policy_root_bytes);
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);

    let mut entries: Vec<Entry> = Vec::new();
    let mut verified = 0usize;
    let mut refused = 0usize;

    // the kernel: context is measurement then the boot epoch, verified against
    // the root the bootloader carries
    let k_elf = fs::read(arg(&args, "--kernel-elf").expect("--kernel-elf")).unwrap();
    let k_root_bytes = fs::read(arg(&args, "--kernel-root").expect("--kernel-root")).unwrap();
    let k_trailer = fs::read(arg(&args, "--kernel-trailer").expect("--kernel-trailer")).unwrap();
    let k_meas = *blake3::hash(&k_elf).as_bytes();
    let mut k_ctx = Vec::with_capacity(40);
    k_ctx.extend_from_slice(&k_meas);
    k_ctx.extend_from_slice(&epoch.to_be_bytes());
    let k_ok = verify_attestation_trailer(
        &hasher,
        LOG_ROUNDS,
        root_from_bytes(&k_root_bytes),
        N_QUERIES,
        &k_trailer,
        &k_ctx,
    );
    if k_ok {
        fs::write(format!("{out}/kernel.trailer.bin"), &k_trailer).unwrap();
        fs::write(format!("{out}/kernel.context.bin"), &k_ctx).unwrap();
        fs::write(format!("{out}/kernel.root.bin"), &k_root_bytes).unwrap();
        entries.push(Entry {
            slug: "kernel".into(),
            handle: "the microkernel".into(),
            kind: "kernel",
            caps: 0,
            measurement: k_meas,
            trailer_len: k_trailer.len(),
        });
        verified += 1;
        println!("ok   kernel                     verified against its boot root");
    } else {
        refused += 1;
        println!("FAIL kernel                     refused, not shipped");
    }

    let mut capsule_leaves: Vec<[Fp; RATE]> = Vec::new();

    // every enrolled capsule: context is measurement, granted capabilities, and
    // the policy epoch, verified against the one policy root
    fs::write(format!("{out}/policy_root.bin"), &policy_root_bytes).unwrap();
    let tsv = fs::read_to_string(arg(&args, "--capsules").expect("--capsules")).unwrap();
    for line in tsv.lines().filter(|l| !l.trim().is_empty()) {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 5 {
            continue;
        }
        let (slug, handle, caps_hex, elf_path, trailer_path) = (f[0], f[1], f[2], f[3], f[4]);
        let caps = u64::from_str_radix(caps_hex.trim_start_matches("0x"), 16).unwrap_or(0);
        let Ok(elf) = fs::read(elf_path) else { continue };
        let Ok(trailer) = fs::read(trailer_path) else { continue };
        capsule_leaves.push(measure_capsule(&hasher, &elf));
        let meas = *blake3::hash(&elf).as_bytes();
        let mut ctx = Vec::with_capacity(48);
        ctx.extend_from_slice(&meas);
        ctx.extend_from_slice(&caps.to_be_bytes());
        ctx.extend_from_slice(&epoch.to_be_bytes());
        let ok =
            verify_attestation_trailer(&hasher, LOG_ROUNDS, policy_root, N_QUERIES, &trailer, &ctx);
        if ok {
            fs::write(format!("{out}/{slug}.trailer.bin"), &trailer).unwrap();
            fs::write(format!("{out}/{slug}.context.bin"), &ctx).unwrap();
            entries.push(Entry {
                slug: slug.into(),
                handle: handle.into(),
                kind: "capsule",
                caps,
                measurement: meas,
                trailer_len: trailer.len(),
            });
            verified += 1;
            println!("ok   {slug:<24} caps {caps_hex:<10} verified against the policy root");
        } else {
            refused += 1;
            println!("FAIL {slug:<24} refused, not shipped");
        }
    }

    // the index the page reads: enough to render the whole trust surface without
    // a second request per capsule for anything but the proof itself
    let mut items = Vec::new();
    for e in &entries {
        let mut m = BTreeMap::new();
        m.insert("slug", format!("\"{}\"", e.slug));
        m.insert("handle", format!("\"{}\"", e.handle));
        m.insert("kind", format!("\"{}\"", e.kind));
        m.insert("caps", format!("{}", e.caps));
        m.insert("measurement", format!("\"{}\"", hexs(&e.measurement)));
        m.insert("trailer_bytes", format!("{}", e.trailer_len));
        let body: Vec<String> = m.iter().map(|(k, v)| format!("    \"{k}\": {v}")).collect();
        items.push(format!("  {{\n{}\n  }}", body.join(",\n")));
    }
    let index = format!(
        "{{\n  \"epoch\": {epoch},\n  \"policy_root\": \"{}\",\n  \"verified\": {verified},\n  \"refused\": {refused},\n  \"items\": [\n{}\n  ]\n}}\n",
        hexs(&policy_root_bytes),
        items.join(",\n")
    );
    fs::write(format!("{out}/index.json"), &index).unwrap();

    println!("\n{verified} attestations verified and written, {refused} refused");
    assert_eq!(
        refused, 0,
        "some attestation refused; the fixture would ship an unverifiable proof"
    );

    // Reconstruct both roots from the binaries alone. This is the set
    // transparency guarantee the page will hand to the browser: the policy
    // root is exactly the fold of these leaves and the reserved padding,
    // nothing more, so no unlisted member can hide under it. If either root
    // does not reproduce, the fixture is wrong and must not ship.
    // The deployment's reserved-slot image, from nonos-stark-enroll. It differs
    // from this repository's own pad constant, a divergence this reconstruction
    // is precisely built to catch; the deployed value is the truth here.
    let pad_leaf = measure_capsule(&hasher, b"\x00NONOS-POLICY-RESERVED-SLOT-v1");
    let slots = 256usize;

    let mut leaves = capsule_leaves.clone();
    while leaves.len() < slots {
        leaves.push(pad_leaf);
    }
    let rebuilt = PoseidonMerkleTree::commit(&hasher, &leaves).root();
    let mut rebuilt_bytes = [0u8; 32];
    for (i, l) in rebuilt.iter().enumerate() {
        rebuilt_bytes[i * 8..i * 8 + 8].copy_from_slice(&l.value().to_le_bytes());
    }
    assert_eq!(
        rebuilt_bytes.as_slice(),
        policy_root_bytes.as_slice(),
        "the policy root does not reproduce from the enrolled binaries; wrong member list or order"
    );
    println!("policy root reproduced from {} member leaves plus padding", capsule_leaves.len());
    let mut leaves_bin = Vec::with_capacity(slots * 32);
    for leaf in &leaves {
        for l in leaf {
            leaves_bin.extend_from_slice(&l.value().to_le_bytes());
        }
    }
    fs::write(format!("{out}/leaves.bin"), &leaves_bin).unwrap();

    let mut k_leaves = vec![measure_capsule(&hasher, &k_elf)];
    while k_leaves.len() < slots {
        k_leaves.push(pad_leaf);
    }
    let k_rebuilt = PoseidonMerkleTree::commit(&hasher, &k_leaves).root();
    let mut k_rebuilt_bytes = [0u8; 32];
    for (i, l) in k_rebuilt.iter().enumerate() {
        k_rebuilt_bytes[i * 8..i * 8 + 8].copy_from_slice(&l.value().to_le_bytes());
    }
    assert_eq!(
        k_rebuilt_bytes.as_slice(),
        k_root_bytes.as_slice(),
        "the kernel attest root does not reproduce from the kernel image"
    );
    println!("kernel root reproduced from the kernel image");
    let mut k_leaves_bin = Vec::with_capacity(slots * 32);
    for leaf in &k_leaves {
        for l in leaf {
            k_leaves_bin.extend_from_slice(&l.value().to_le_bytes());
        }
    }
    fs::write(format!("{out}/kernel_leaves.bin"), &k_leaves_bin).unwrap();
}
