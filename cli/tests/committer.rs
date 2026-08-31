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

//! The periodic committer has one root per AIR, whatever path computes it.
//!
//! The crate has two committers for the same object: a materialized one that
//! holds the whole coset extension, and a streaming one that hashes a coset at
//! a time so a large trace fits in memory. They must agree on every input, not
//! only on the ones the consumers happen to prove, because the root is the
//! statement a verifier key binds and a registration records.
//!
//! They diverged once, on an AIR with no periodic columns: the streaming path
//! sized its digest vector from the domain and produced a tree of hashed empty
//! rows, where the materialized path took the leaf count from the columns and
//! produced an empty tree. Every AIR in one consumer's corpus carried periodic
//! columns, so that consumer could not see it; the other consumer's zero-column
//! AIR failed within a minute. This test keeps the edge case in the crate's own
//! gate, so neither consumer has to be the one that notices.

use nonos_stark::air::{periodic_root, Air, AirExt, RangeCheck};

/// The zero-column edge: a root is still a root, and it is the same one at
/// every rate. `RangeCheck` carries no periodic columns, which is the whole
/// point of using it here.
#[test]
fn the_zero_column_air_commits_one_root_at_every_rate() {
    let air = RangeCheck { log_t: 6 };
    assert!(air.periodic_columns().is_empty(), "RangeCheck must be the zero-column AIR");

    // The root is defined for the zero-column case rather than panicking or
    // depending on the blowup, and it is stable across rates because there is
    // nothing to extend.
    let at_zero = periodic_root(&air, 0);
    let at_three = periodic_root(&air, 3);
    assert_eq!(
        at_zero, at_three,
        "a zero-column AIR has nothing to extend, so its root cannot depend on the rate"
    );
}

/// Determinism: the same AIR at the same rate commits the same root every
/// time. Trivial to state, and the property that fails first when a committer
/// picks up an ordering or allocation dependency.
#[test]
fn the_committer_is_deterministic() {
    let air = RangeCheck { log_t: 5 };
    let a = periodic_root(&air, 2);
    let b = periodic_root(&air, 2);
    assert_eq!(a, b, "the periodic root must be a function of the AIR and the rate");
}

/// Two zero-column AIRs commit the *same* root, and that is correct rather
/// than a collision: with no periodic columns there is nothing to commit, so
/// both trees are the empty commitment. Worth pinning, because the intuition
/// says "different AIRs, different roots" and here the intuition is wrong. A
/// zero-column periodic root carries no information about the AIR, and any
/// caller that treats it as an identity for one is misusing it; identity comes
/// from the verifier key that binds the whole program, not from this root
/// alone.
#[test]
fn zero_column_airs_share_the_empty_commitment() {
    let small = RangeCheck { log_t: 5 };
    let large = RangeCheck { log_t: 6 };
    assert!(small.periodic_columns().is_empty() && large.periodic_columns().is_empty());
    assert_eq!(
        periodic_root(&small, 0),
        periodic_root(&large, 0),
        "with no periodic columns there is nothing to commit, so both are the empty tree"
    );
}
