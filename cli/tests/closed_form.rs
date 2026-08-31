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

//! Two implementations of the subgroup closed form, held to agree.
//!
//! The crate carries two ways to evaluate a column on a multiplicative
//! subgroup at a point outside it. They were written independently, by two
//! teams, for the same reason: the Lagrange form costs a multiplication per
//! evaluation point per column, and the closed form collapses it.
//!
//! Exactly one of them is on the proving path. `poly::lagrange`'s form is read
//! by every call site and covered by the prover digest gate, so it is the
//! implementation. `poly::barycentric` is kept as the cross-check: same math,
//! different derivation, and this test asserts they agree on random inputs.
//!
//! Two implementations of one function in one crate is normally the disease
//! this unification exists to cure. It is only worth keeping when the second
//! is never called in anger and its whole job is to disagree loudly if the
//! first drifts. That is what this file enforces: if the two ever diverge, the
//! suite fails, and the crate is telling the truth about which one it trusts.

use nonos_stark::field::{Fp, Fp2};
use nonos_stark::poly::{eval_cols_on_subgroup_ext, eval_subgroup_ext};

/// A small deterministic generator, so a failure is reproducible from the seed
/// printed in the assertion rather than from a lucky run.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64star: adequate for choosing test points, and it keeps the
        // test free of a dependency for something this small
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn fp(&mut self) -> Fp {
        Fp::from_u64(self.next())
    }

    fn fp2(&mut self) -> Fp2 {
        Fp2::new(self.fp(), self.fp())
    }
}

/// The generator of the subgroup of the given size, from the field's two-adic
/// tower. Both implementations take the generator as their domain description,
/// so the test supplies the same one to each.
fn subgroup_generator(log_size: u32) -> Fp {
    nonos_stark::fri::root_of_unity(log_size)
}

#[test]
fn the_two_closed_forms_agree_on_random_columns() {
    let mut rng = Rng(0x5EED_1234_ABCD_0001);

    for log_t in [2u32, 3, 4, 5] {
        let t = 1usize << log_t;
        let g = subgroup_generator(log_t);

        for trial in 0..8 {
            // one random column over the subgroup
            let col: Vec<Fp> = (0..t).map(|_| rng.fp()).collect();
            // a random evaluation point in the extension, essentially never in
            // the subgroup, which is the case both forms are used for
            let z = rng.fp2();

            let via_barycentric = eval_subgroup_ext(g, &col, z);
            let via_lagrange = eval_cols_on_subgroup_ext(g, t, core::slice::from_ref(&col), z);

            let Some(b) = via_barycentric else {
                panic!("barycentric returned none for an off-domain point (log_t {log_t}, trial {trial})");
            };
            assert_eq!(
                b, via_lagrange[0],
                "the two closed forms disagree at log_t {log_t}, trial {trial}; \
                 one of them has drifted and the proving path reads the lagrange form"
            );
        }
    }
}

#[test]
fn the_two_closed_forms_agree_across_several_columns_at_once() {
    let mut rng = Rng(0x5EED_1234_ABCD_0002);
    let log_t = 4u32;
    let t = 1usize << log_t;
    let g = subgroup_generator(log_t);

    let cols: Vec<Vec<Fp>> = (0..6).map(|_| (0..t).map(|_| rng.fp()).collect()).collect();
    let z = rng.fp2();

    let batched = eval_cols_on_subgroup_ext(g, t, &cols, z);
    assert_eq!(batched.len(), cols.len(), "one evaluation per column");

    for (i, col) in cols.iter().enumerate() {
        let single = eval_subgroup_ext(g, col, z).expect("off-domain point");
        assert_eq!(
            single, batched[i],
            "column {i} evaluates differently batched than singly; the batched form \
             is the one on the proving path"
        );
    }
}

#[test]
fn the_closed_form_reproduces_a_known_evaluation() {
    // A column that is the constant one evaluates to one everywhere, including
    // off the domain. Independent of both implementations' internals, so it
    // catches the case where they agree with each other and are both wrong.
    let log_t = 3u32;
    let t = 1usize << log_t;
    let g = subgroup_generator(log_t);
    let ones: Vec<Fp> = core::iter::repeat(Fp::ONE).take(t).collect();

    let mut rng = Rng(0x5EED_1234_ABCD_0003);
    let z = rng.fp2();

    let bary = eval_subgroup_ext(g, &ones, z).expect("off-domain point");
    let lagr = eval_cols_on_subgroup_ext(g, t, core::slice::from_ref(&ones), z);
    assert_eq!(bary, Fp2::ONE, "the constant one column must evaluate to one");
    assert_eq!(lagr[0], Fp2::ONE, "the constant one column must evaluate to one");
}
