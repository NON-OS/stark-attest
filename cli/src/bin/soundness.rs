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

//! The cryptographic measurements, as opposed to the wall clock ones.
//!
//! A benchmark that reports nanoseconds says how fast the system is. It says
//! nothing about how much security those nanoseconds bought, and the second
//! question is the one an auditor asks first. This binary computes the
//! quantities that decide that, from the parameters actually compiled into the
//! verifier rather than from a document that hopes to match them:
//!
//!   - the soundness error of the low degree test, in bits, and its parts:
//!     the query term, the grinding term, and the proximity parameter the
//!     blowup buys;
//!   - the challenge space the folds are drawn from, which is the ceiling on
//!     the whole argument regardless of query count;
//!   - the cost of forging, expressed as the work an adversary must do, so
//!     the number can be compared against a budget rather than admired;
//!   - the proof size and the verifier's work in hash invocations, which is
//!     what a consumer embedding this must actually pay.
//!
//! Everything is derived, not asserted. Change `attest_params` and every
//! number here moves with it, which is the point: a security claim in the
//! repository that disagrees with the compiled verifier is a claim that will
//! be believed and should not be.
//!
//! The model, stated so a reviewer can reject it: for a codeword at relative
//! distance delta from the low degree code, one FRI query catches the prover
//! with probability at least delta, so q independent queries leave at most
//! (1 - delta)^q, and the transcript grind multiplies an attacker's cost of
//! searching for favourable challenges by 2^grind. The proximity delta used
//! here is the conservative (1 - rate)/2 unique decoding bound rather than a
//! list decoding radius, so the reported soundness is a floor and not the
//! best the literature supports.
//!
//! Run: `cargo run --release --bin soundness -- bench/soundness.json`

use std::env;
use std::fs;

use nonos_stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};

/// The membership AIR's own blowup, before the extra bits the parameters add.
/// The AIR's constraint degree sets it; three is what the membership circuit
/// compiles to, and it is stated here as the one input this file does not read
/// from `attest_params`, because it is a property of the AIR rather than of
/// the parameter set.
const AIR_BLOWUP_BITS: u32 = 1;

/// Bits in a field element of the base field (Goldilocks is just under 64).
const BASE_FIELD_BITS: f64 = 63.999_999_999_2;

/// The tree depth the deployed trailer format carries.
const TREE_DEPTH: u32 = 8;

fn main() {
    let out = env::args().nth(1).unwrap_or_else(|| "bench/soundness.json".to_string());

    // The evaluation domain is 2^(air + extra) times the trace: the code rate
    // is the reciprocal, and the unique decoding radius is half its complement.
    let blowup_bits = AIR_BLOWUP_BITS + EXTRA_BLOWUP_BITS;
    let blowup = 2f64.powi(blowup_bits as i32);
    let rate = 1.0 / blowup;
    let delta = (1.0 - rate) / 2.0;

    // One query misses a far codeword with probability at most 1 - delta;
    // q independent queries miss with at most (1 - delta)^q. In bits.
    let per_query_miss_bits = -(1.0 - delta).log2();
    let query_bits = per_query_miss_bits * N_QUERIES as f64;

    // Grinding adds its bits directly to the work of searching for a
    // favourable transcript.
    let grind_bits = GRIND_BITS as f64;

    // The folding challenges come from the quadratic extension, so the
    // challenge space is twice the base field's bits. No number of queries can
    // buy soundness past this ceiling.
    let challenge_space_bits = 2.0 * BASE_FIELD_BITS;

    // The optimistic model, and the one deployed STARKs are analysed under:
    // in the list decoding regime a query catches a far prover with
    // probability approaching 1 - rate, so each query is worth about
    // -log2(rate) bits rather than the unique decoding bound's fraction of a
    // bit. Both are reported. Neither is a proof: the honest reading is that
    // the true value lies between them, nearer the list decoding figure for
    // the conjectured regime the literature uses, and that this crate has not
    // had an independent analysis pin it.
    let per_query_list_bits = -rate.log2();
    let query_bits_list = per_query_list_bits * N_QUERIES as f64;
    let total_list = (query_bits_list + grind_bits).min(challenge_space_bits);

    // The argument's soundness is the query term plus the grind, capped by the
    // challenge space: an adversary who can enumerate the challenge space does
    // not need to defeat the queries at all.
    let total_bits = (query_bits + grind_bits).min(challenge_space_bits);

    // Forging work, expressed as operations rather than as an exponent, so it
    // can be compared with a budget. Saturates rather than overflowing.
    let forge_ops = if total_bits >= 128.0 { f64::INFINITY } else { 2f64.powf(total_bits) };

    // What a consumer pays to verify: one hash per level per query for the
    // authentication paths, plus the fold checks. This is the shape of the
    // verifier's cost, not a timing.
    let hashes_per_query = TREE_DEPTH;
    let verifier_hashes = hashes_per_query as usize * N_QUERIES;

    // The in-circuit Poseidon's round count, from the parameter set.
    let poseidon_rounds = 1usize << LOG_ROUNDS;

    let doc = format!(
        r#"{{
  "schema": "stark-attest.soundness.v1",
  "note": "derived from the compiled parameters, not from documentation",
  "parameters": {{
    "log_rounds": {log_rounds},
    "poseidon_rounds": {poseidon_rounds},
    "queries": {queries},
    "grind_bits": {grind},
    "air_blowup_bits": {air_blowup},
    "extra_blowup_bits": {extra_blowup},
    "total_blowup_bits": {blowup_bits},
    "tree_depth": {depth}
  }},
  "code": {{
    "rate": {rate},
    "unique_decoding_radius": {delta},
    "model": "conservative unique decoding bound, not a list decoding radius"
  }},
  "soundness_bits": {{
    "unique_decoding": {{
      "per_query": {per_query},
      "queries_total": {query_bits},
      "argument_total": {total},
      "model": "pessimistic floor: delta = (1 - rate) / 2, provable today"
    }},
    "list_decoding": {{
      "per_query": {per_query_list},
      "queries_total": {query_bits_list},
      "argument_total": {total_list},
      "model": "conjectured regime the deployed STARK literature analyses under"
    }},
    "grinding": {grind},
    "challenge_space_ceiling": {ceiling},
    "honest_reading": "the true value lies between the two; no independent analysis of this crate pins it"
  }},
  "adversary": {{
    "forge_operations": {forge},
    "interpretation": "expected work to produce one accepted false proof"
  }},
  "verifier_cost": {{
    "hash_invocations": {hashes},
    "hashes_per_query": {per_q_hash},
    "note": "authentication paths only; fold arithmetic is field work, not hashing"
  }}
}}
"#,
        log_rounds = LOG_ROUNDS,
        poseidon_rounds = poseidon_rounds,
        queries = N_QUERIES,
        grind = GRIND_BITS,
        air_blowup = AIR_BLOWUP_BITS,
        extra_blowup = EXTRA_BLOWUP_BITS,
        blowup_bits = blowup_bits,
        depth = TREE_DEPTH,
        rate = rate,
        delta = delta,
        per_query = per_query_miss_bits,
        query_bits = query_bits,
        per_query_list = per_query_list_bits,
        query_bits_list = query_bits_list,
        total_list = total_list,
        ceiling = challenge_space_bits,
        total = total_bits,
        forge = if forge_ops.is_infinite() {
            "\"beyond 2^128\"".to_string()
        } else {
            format!("{forge_ops:.0}")
        },
        hashes = verifier_hashes,
        per_q_hash = hashes_per_query,
    );

    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(&out, &doc).expect("write the soundness record");
    println!("{doc}");

    // The gate is on the list decoding figure, because that is the regime the
    // parameters were chosen under, and on the pessimistic floor not falling
    // below the level at which the difference between the two models stops
    // being academic. A parameter change that moves either is a security
    // change and must not pass silently.
    assert!(
        total_list >= 100.0,
        "list decoding soundness fell to {total_list:.1} bits; the parameters were weakened"
    );
    assert!(
        total_bits >= 40.0,
        "the provable floor fell to {total_bits:.1} bits, which is too thin a margin \
         to rest on the conjectured regime; raise the queries or the blowup"
    );
    println!("wrote {out}");
}
