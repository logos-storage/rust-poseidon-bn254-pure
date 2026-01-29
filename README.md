rust-poseidon-bn254-pure
------------------------

Self-contained (no external dependencies), pure Rust implementation of Poseidon 
and Poseidon2 hash functions over the BN254 curve's scalar field, using 32 bit 
limbs internally.

It's primarily intended to be used on 32-bit platforms, eg. 32-bit RISC-V (`rv32im`)
(though porting to 64 bits shouldn't be a big effort; TODO).

The algebra implementation is based on [`zikkurat-algebra`](https://github.com/faulhornlabs/zikkurat-algebra/)
and [`staging-agda`](https://github.com/faulhornlabs/staging-agda/).

### Compatibility

The Poseidon implementation is compatible with [`circomlib`](https://github.com/iden3/circomlib/).

The Poseidon2 implementation is compatible with [`zkfriendlyhashzoo`](https://extgit.isec.tugraz.at/krypto/zkfriendlyhashzoo).
It _used to be_ compatible with the [HorizenLabs implementation](https://github.com/HorizenLabs/poseidon2),
until they changed all their constants in [this commit](https://github.com/HorizenLabs/poseidon2/commit/bb476b9ca38198cf5092487283c8b8c5d4317c4e).
We don't think it's worth the pain to follow this change.

### Status

Currently, only the following instances are implemented:

- Poseidon permutation with `t=2,3,4,5` over BN254's scalar field
- Poseidon2 permutation with `t=3` over BN254's scalar field

I feel that larger states are unneccesary in practice. As a concrete example,
[PSE's RLN circuit](https://github.com/Rate-Limiting-Nullifier/circom-rln) uses `t=2,3,4`.

The proper way to handle larger input is to implement the sponge construction.

### Usage

There are three main types:

- `BigInt<N>` is an unsigned big integer consisting of `N` words (so `2^(32*N)` or `2^(64*N)` bits);
- `Felt`, short for "Field Element", is a prime field element in the standard representation
  (integers modulo `p`);
- `Mont` is a field element in the Montgomery represntation. This is used internally 
  for calculations, as the multiplications is much faster this way.

The core functionality of the Poseidon family of hash functions is the _permutation_, 
which takes an array of `t` field elements, and returns the same:

    fn permute( [Felt; t] ) -> [Felt; t]

From this one can build all kind of stuff, including a proper hash function (using
the so-called "sponge construction). The latter is not implemented in `circomlib`,
instead, what they have is a compression function parametrized by `t`:

    fn compress( [Felt; t-1] ) -> Felt

This takes `t-1` field elements and returns one (which is interpreted as a hash).

This is implemented by extending the input with a 0, applying the permutation, and
taking the first element of the output vector (note: in `circomlib`, the extra 0 is 
at the beginning, not at the end, but that doesn't matter at all; just be consistent).

Remark: That extra zero (called the "capacity") is _extremely important_, without 
that the whole construction would be totally insecure!

### Speed

Some approximate benchmark numbers below.

#### 32-bit RISC-V 

On RV32IM (the primary target as of now), we have approximately the following cycle counts:

- Poseidon: about 900k cycles for a single `t=3` permutation
- Poseidon2: about 350k cycles for a single `t=3` permutation

Note: Poseidon is about 2.5x slower, simply because there are about 2.5x more 
field multiplications involved (which absolutely dominate the runtime).

#### Modern CPUs

On modern 64-bit CPU-s, the 64-bit version would be preferred (TODO: implement it).

32 bit version, running on an M2 macbook pro (single threaded):

- Poseidon:  320 msec for 10,000 `t=3` permutations
- Poseidon2: 140 msec for 10,000 `t=3` permutations

### TODO

- [x] clean up the code and make it more idiomatic
- [x] implement `circomlib`-compatible Poseidon
- [x] benchmark RISC-V cycles
- [ ] add more Poseidon2 state widths (not just `t=3`)
- [ ] add a proper test-suite; in particular, more complete testing of the field operations
- [ ] add a 64 bit version
- [ ] implement the sponge construction
- [ ] optimize squaring to use less multiplications (?)
- [ ] investigate further optimization possibilities (?)

