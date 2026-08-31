# stark-attest

Attest a set of artifacts with one 32-byte statement, transparently, and
post-quantum.

`stark-attest` commits a set of files (a release, a firmware image and its
components, a package registry snapshot, an operating system's userland) to a
single Merkle root, and issues each member a self-contained trailer: a STARK
membership proof binding that member's measurement, a caller-chosen context,
and an epoch, under that root. Anyone holding the root can verify any member
in milliseconds. There is no trusted setup, no ceremony, and no toxic waste;
the construction rests on a hash function (Poseidon over the Goldilocks
field, FRI for low-degree testing), which is also why it is post-quantum by
construction rather than by patch.

This is not a research prototype. The engine is extracted verbatim from the
NONOS operating system, where the same verifier gates every process spawn in
the kernel and the kernel's own image in the bootloader: 87 userland capsules
are enrolled under one root on every release build, and the whole set
re-verifies in seconds on a laptop-class host. The prover and the
verifier read their soundness parameters from one module
(`attest_params.rs`), so a drift between the two is a compile error, not a
quiet downgrade.

## What problem this solves

Release signing tells you who published a set of bytes. It does not give you
a single small statement that a whole *set* of artifacts is exactly what it
claims, verifiable per-member without fetching or trusting anything else.
Projects hack around this with detached signature lists, SHA256SUMS files
signed as a blob, or transparency logs that need an online service. A
`stark-attest` root is 32 bytes you can print in a release note, carve into a
bootloader, or pin in a client; each artifact then carries its own proof of
membership under it, offline, with a verifier small enough to embed anywhere.

The proofs bind three things at once, and all three are checked in one
verification: the artifact's measurement (BLAKE3), a context the issuer
chooses (in NONOS it is the capability mask a capsule was granted), and an
epoch (so yesterday's set cannot answer for today's). A swapped artifact, a
tampered context, and a replayed epoch all fail the same gate.

## Verified interoperability

The trailer format is byte-compatible with the NONOS enrollment tool: a set
enrolled by `nonos-stark-enroll` verifies under this CLI, and vice versa,
because both call the same library with parameters read from the same module.
Demonstrated, not asserted:

```
nonos-stark-enroll capsules root.bin 0x19:a.bin:a.zk 0x1819:b.bin:b.zk
# nonos binds context = caps as 8 big-endian bytes, then epoch as 8
stark-attest verify root.bin \
    00000000000000190000000000000001:a.bin:a.zk \
    00000000000018190000000000000001:b.bin:b.zk
# => ok a.bin, ok b.bin, verified 2 proofs
```

## Status and honesty

- The prover in this tree is the non-streaming one. A streamed prover
  (constant-memory per coset) exists upstream and is the load-bearing path
  for very large traces; this repo rebases onto it before any release, and
  until then large-trace proving here should be considered memory-bound.

- The core (field, Poseidon, FRI, the membership AIR, prover, verifier) is
  the `nonos-stark` engine, in production use in NONOS. Soundness of the
  construction is inherited from its standard components; there is no
  in-tree machine-checked soundness proof, and this README will say so until
  there is one.
- The CLI (`enroll`, `verify`) generalizes what NONOS's build does for its
  capsule set. It is young; the library underneath is not.
- Proving is the expensive side (tens of seconds per member, parallel across
  members); verification is milliseconds per member. That asymmetry is
  inherent and correct: pay at release, verify everywhere, forever.
- One root commits to one set. Changing any member changes the root and
  invalidates every proof; that is the semantics of a set commitment, not a
  limitation. Re-enroll on release, not on iteration.

## License

AGPL-3.0-or-later, as inherited from the source tree it was extracted from.
