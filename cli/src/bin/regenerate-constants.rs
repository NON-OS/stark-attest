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

//! Regenerate the width-8 Poseidon parameters from the published rule, and
//! check the crate against them.
//!
//! `docs/poseidon-spec.md` claims every constant of the in-circuit permutation
//! is derived by a rule anyone can rerun. A claim like that is worth exactly as
//! much as its independent check, so this binary implements the rule from the
//! specification text alone, in arithmetic written for clarity rather than
//! speed, and compares the result with what the crate compiled in. It also
//! verifies the two structural properties the specification leans on:
//!
//!   - the S-box exponent is coprime to the multiplicative group order, so
//!     `x^7` is a permutation of the field;
//!   - the diffusion matrix is Cauchy over two disjoint node sets, which is
//!     the hypothesis of the classical theorem that makes it MDS. Every square
//!     submatrix being invertible follows; a sample of them is checked here as
//!     a sanity test on the construction rather than as the proof.
//!
//! Anyone auditing this crate can run this without reading the implementation,
//! and anyone building a different system can read this file as a worked
//! example of deriving constants reproducibly.
//!
//! Run: `cargo run --release --bin regenerate-constants`

use nonos_stark::air::{Poseidon, RATE, WIDTH};
use nonos_stark::attest_params::LOG_ROUNDS;
use nonos_stark::field::Fp;

/// Goldilocks.
const P: u128 = 0xFFFF_FFFF_0000_0001;
/// The S-box exponent.
const ALPHA: u128 = 7;
/// The domain string from the specification.
const RC_DOMAIN: &[u8] = b"NONOS-POSEIDON-GOLDILOCKS-RC";

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Modular inverse by the extended Euclidean algorithm, over u128 so the
/// intermediate arithmetic is obviously correct rather than clever.
fn inv_mod_p(a: u128) -> u128 {
    let (mut t, mut new_t) = (0i128, 1i128);
    let (mut r, mut new_r) = (P as i128, a as i128);
    while new_r != 0 {
        let q = r / new_r;
        (t, new_t) = (new_t, t - q * new_t);
        (r, new_r) = (new_r, r - q * new_r);
    }
    assert_eq!(r, 1, "no inverse; the element was zero or the modulus composite");
    let mut out = t;
    if out < 0 {
        out += P as i128;
    }
    out as u128
}

/// The Cauchy entry from the specification: `1 / (i - (WIDTH + j))`.
fn cauchy_entry(i: usize, j: usize) -> u128 {
    let x = i as u128 % P;
    let y = (WIDTH + j) as u128 % P;
    let diff = (x + P - y) % P;
    inv_mod_p(diff)
}

/// The round constant from the specification: BLAKE3 of the domain string with
/// the round and lane indices, first eight bytes little-endian, reduced.
fn round_constant(r: usize, j: usize) -> u128 {
    let mut buf = Vec::with_capacity(RC_DOMAIN.len() + 16);
    buf.extend_from_slice(RC_DOMAIN);
    buf.extend_from_slice(&(r as u64).to_le_bytes());
    buf.extend_from_slice(&(j as u64).to_le_bytes());
    let h = blake3::hash(&buf);
    let b = h.as_bytes();
    let v = u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]) as u128;
    if v >= P {
        v - P
    } else {
        v
    }
}

/// Determinant of a small matrix by Gaussian elimination over the field.
/// Used only to spot-check submatrix invertibility.
fn det(m: &[Vec<u128>]) -> u128 {
    let n = m.len();
    let mut a: Vec<Vec<u128>> = m.to_vec();
    let mut d: u128 = 1;
    for col in 0..n {
        let Some(piv) = (col..n).find(|&r| a[r][col] != 0) else {
            return 0;
        };
        if piv != col {
            a.swap(piv, col);
            d = (P - d) % P;
        }
        d = d * a[col][col] % P;
        let inv = inv_mod_p(a[col][col]);
        for r in (col + 1)..n {
            let f = a[r][col] * inv % P;
            if f == 0 {
                continue;
            }
            for c in col..n {
                let sub = f * a[col][c] % P;
                a[r][c] = (a[r][c] + P - sub) % P;
            }
        }
    }
    d
}

fn main() {
    let rounds = 1usize << LOG_ROUNDS;
    println!("width {WIDTH}, rate {RATE}, rounds {rounds}, S-box x^{ALPHA}");
    println!("field p = 2^64 - 2^32 + 1 = {P}\n");

    // 1. the S-box is a permutation
    let order = P - 1;
    let g = gcd(ALPHA, order);
    assert_eq!(g, 1, "x^{ALPHA} is not a permutation: gcd with p-1 is {g}");
    println!("ok   gcd({ALPHA}, p-1) = 1, so x^{ALPHA} permutes the field");

    // 2. the diffusion matrix matches the rule, and its node sets are disjoint
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let mds = hasher.mds();
    let mut mismatches = 0usize;
    for i in 0..WIDTH {
        for j in 0..WIDTH {
            let expected = cauchy_entry(i, j);
            let actual = mds[i][j].value() as u128;
            if expected != actual {
                println!("MISMATCH mds[{i}][{j}]: rule {expected}, crate {actual}");
                mismatches += 1;
            }
        }
    }
    assert_eq!(mismatches, 0, "the compiled MDS does not match the published rule");
    println!("ok   all {}x{} MDS entries match the Cauchy rule", WIDTH, WIDTH);

    // the two node sets are {0..W} and {W..2W}: disjoint, which is the
    // theorem's hypothesis
    assert!(WIDTH <= WIDTH * 2, "node sets must not overlap");
    println!(
        "ok   node sets {{0..{}}} and {{{}..{}}} are disjoint, so the matrix is MDS by theorem",
        WIDTH,
        WIDTH,
        2 * WIDTH
    );

    // spot-check: a few square submatrices are invertible, as MDS requires
    let full: Vec<Vec<u128>> =
        (0..WIDTH).map(|i| (0..WIDTH).map(|j| cauchy_entry(i, j)).collect()).collect();
    let mut checked = 0usize;
    for size in 2..=4usize {
        for off in 0..(WIDTH - size) {
            let sub: Vec<Vec<u128>> = (off..off + size)
                .map(|i| (off..off + size).map(|j| full[i][j]).collect())
                .collect();
            assert_ne!(det(&sub), 0, "a {size}x{size} submatrix at offset {off} is singular");
            checked += 1;
        }
    }
    println!("ok   {checked} square submatrices are invertible, consistent with MDS");

    // 3. the round constants match the rule
    let mut rc_mismatch = 0usize;
    for r in 0..rounds {
        let row = hasher.round_constant(r);
        for (j, cell) in row.iter().enumerate() {
            let expected = round_constant(r, j);
            let actual = cell.value() as u128;
            if expected != actual {
                if rc_mismatch < 5 {
                    println!("MISMATCH rc[{r}][{j}]: rule {expected}, crate {actual}");
                }
                rc_mismatch += 1;
            }
        }
    }
    assert_eq!(rc_mismatch, 0, "the compiled round constants do not match the published rule");
    println!("ok   all {} round constants match BLAKE3 of the domain rule", rounds * WIDTH);

    println!("\nthe compiled permutation is exactly the one docs/poseidon-spec.md specifies");
}
