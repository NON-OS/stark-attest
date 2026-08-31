# Machine-checked facts

The streamed prover rests on two load-bearing claims that are easy to state,
easy to get subtly wrong, and cheap to check by machine, so they are checked
by machine. Lean 4, core library only, no axioms beyond `propext` and
`Quot.sound`, no `sorry`.

- **`Zkolang/Stream.lean`**: the coset-window fact. Streaming the evaluation
  domain one coset at a time visits exactly the points the monolithic domain
  holds, each once, in an order the commitment is invariant to. This is the
  theorem that makes "holds one coset resident" a refactor rather than a
  protocol change: if it held anything less or anything twice, the streamed
  prover would commit to a different codeword than the monolithic one.
- **`Zkolang/BatchInv.lean`**: the Montgomery batch-inversion walk. One
  inversion per block plus a prefix-product walk yields exactly the
  elementwise inverses, provided no element is zero, and the zero case is
  detected rather than folded into garbage. This is what lets the DEEP pass
  batch its denominators without changing a single emitted value.
- **`Zkolang/Field.lean`**: the integer-level facts about the field the
  others lean on.
- **`Zkolang/Reduce.lean`**: the Goldilocks fast reduction. The crate reduces
  128-bit products without division through the identities
  `2^64 = EPSILON (mod p)` and `2^96 = -1 (mod p)`; this module proves the
  limb form the fast path computes is congruent to the original value for
  every input, exhibiting the difference as the explicit multiple
  `p * (x_hi_lo + (2^32 + 1) * x_hi_hi)`. The Rust property test samples this
  identity; the theorem covers all 2^128 inputs, so the sampling is a
  backstop, not the guarantee.

The module names carry the `Zkolang` namespace because the files are imported
verbatim from the proof corpus they were written in; renaming modules would
re-open proofs that are closed, for cosmetics. Provenance over polish.

Three further modules prove the security-critical seams of the crate itself,
not just its algorithms:

- **`Zkolang/Trailer.lean`**: the trailer parse is total. The first code a
  verifier runs on attacker-controlled bytes slices a magic, a depth, a
  direction bitfield and `depth * RATE` field elements out of a blob whose
  length it does not choose. Eight theorems show every one of those reads lies
  inside the length the guard established, for every depth and every length,
  that the sibling region is exactly the digests it claims, that no unparsed
  gap precedes the proof bytes, and that a short blob and a zero depth are
  both refused. A panic in a verifier is a denial of service at best and a
  parser differential at worst; this rules both out arithmetically rather
  than by fuzzing.
- **`Zkolang/Padding.lean`**: reserved slots are unreachable. The policy tree
  pads to a fixed width, and if a real artifact could measure into a padding
  slot an attacker would have a free membership proof. The separation is
  structural: the padding image begins with a byte no admissible artifact
  begins with, so the images differ before any hash is applied. The theorem is
  stated with the hash abstract, which means the separation survives a hash
  break. It is a domain separation proof, not a collision argument.
- **`Zkolang/Params.lean`**: the verifier cannot be talked down. Soundness
  parameters come from one module and nothing a prover supplies can influence
  them; the theorems state that the effective parameters equal the verifier's
  own for every claim a proof might carry, that the survival bound is monotone
  in the query count, and pin the deployed values so a silent change to
  `attest_params` breaks a proof rather than a promise.

Every theorem depends on at most `propext` and `Quot.sound`, the two axioms of
Lean's core logic, and CI enforces that with `#print axioms` on each advertised
result plus a scan for `sorry`. A claimed proof that quietly acquired an axiom
or a hole fails the build.

What this directory does not claim: these are proofs about specific algorithms
and parse-level properties, not a verification of the prover, the AIR, or the
protocol. The
honest map of what is proven, tested, and inherited across the whole crate is
[../docs/security.md](../docs/security.md).

Build with `lake build` under the toolchain pinned in `lean-toolchain`, or
through the flake's checks.
