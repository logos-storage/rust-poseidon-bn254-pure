rust-poseidon-bn254-pure
------------------------

Self-contained (no external dependencies), pure Rust implementation of Poseidon 
and Poseidon2 hash functions over the BN254 curve's scalar field, using 32 bit 
limbs internally.

It's primarily intended to be used on 32-bit platforms, eg. 32-bit RISC-V (`rv32im`)
(though porting to 64 bits shouldn't be a big effort; TODO).

The algebra implementation is mostly based on [`zikkurat-algebra`](https://github.com/faulhornlabs/zikkurat-algebra/)
and [`staging-agda`](https://github.com/faulhornlabs/staging-agda/).

### Compatibility

The Poseidon implementation is compatible with [`circomlib`](https://github.com/iden3/circomlib/).

The Poseidon2 implementation is compatible with [`zkfriendlyhashzoo`](https://extgit.isec.tugraz.at/krypto/zkfriendlyhashzoo).
and the [HorizenLabs implementation(s)](https://github.com/HorizenLabs/poseidon2).

#### Important compatibility note

For Poseidon2, because of a historical accident, there are unfortunately TWO different 
sets of "standard" parameters, which is obviously bad for cross-project compatibility.

For example [Codex used](https://github.com/logos-storage/logos-storage-proofs-circuits/tree/master/circuit/poseidon2) 
Poseidon2 w/ `t=3` and the "old" parameters, while
[Aztec's Barretenberg](https://github.com/AztecProtocol/barretenberg/tree/master/cpp/src/barretenberg/crypto/poseidon2) 
uses `t=4` and the "new" parameters. 

The switchover happened in 
[commit #bb476b9ca38198cf5092487283c8b8c5d4317c4e](https://github.com/HorizenLabs/poseidon2/commit/bb476b9ca38198cf5092487283c8b8c5d4317c4e)
in HorizenLab's reference repo. Both versions are safe to use though, and 
to resolve this issue, we implement both sets.

### Implementation status

Currently, the following instances are implemented:

- Poseidon permutation with `t=2,3,4,5` over BN254's scalar field
- Poseidon2 permutation with `t=2,3,4` over BN254's scalar field (Poseidon2 is not officially specified for `t=5`)

While `circomlib` implements state widths up to `t=17`, I feel that larger 
states (eg. `t > 4` in case of BN254) are unneccesary in practice. As a concrete example,
[PSE's RLN circuit](https://github.com/Rate-Limiting-Nullifier/circom-rln) uses `t=2,3,4`.

The proper way to handle larger input is to implement the sponge construction instead.

### Usage

There are three main types:

- `BigInt<N>` is an unsigned big integer consisting of `N` words (so `2^(32*N)` or `2^(64*N)` bits);
- `Felt`, short for "Field Element", is a prime field element in the standard representation
  (integers modulo `p`);
- `Mont` is a field element in the Montgomery representation. This is used internally 
  for calculations, as multiplication (the main bottleneck) is much faster this way.

The core functionality of the Poseidon family of hash functions is the _permutation_, 
which takes an array of `t >= 2` field elements, and returns the same:

    fn permute( [Felt; t] ) -> [Felt; t]

From this one can build all kinds of stuff, including a proper hash function (using
the so-called "sponge construction). The latter is not implemented in `circomlib`,
instead, what they have is a _compression function_ parametrized by `t`:

    fn compress( [Felt; t-1] ) -> Felt

This takes `t-1` field elements and returns a single one (which is interpreted as a hash. 
Note that a field element contains about 254 bits of information, which is pretty fine for a cryptographic hash output)

This is implemented by extending the input with a `0`, applying the permutation, and
taking the first element of the output vector (note: in `circomlib`, the extra `0` is 
at the beginning, not at the end, but that doesn't matter at all; just be consistent).

Remark: That extra zero (called the "capacity") is _extremely important_, without 
that the whole construction would be totally insecure!

### Speed

Some approximate benchmark numbers below.

#### 32-bit RISC-V 

On RV32IM (the primary target as of now), we have approximately the following cycle counts:

- Poseidon: 
    - about 615k cycles for a single `t=2` permutation
    - about 915k cycles for a `t=3` permutation
    - about 1220k cycles for a `t=4` permutation
- Poseidon2: 
    - about 325k cycles for a single `t=2` permutation
    - about 375k cycles for a `t=3` permutation
    - about 7705 cycles for a `t=4` permutation

Note: Poseidon is about 2.5x slower, simply because there are about 2.5x more 
field multiplications involved (which absolutely dominate the runtime).

#### Modern CPUs

On modern 64-bit CPU-s, the 64-bit version would be preferred (TODO: implement it).

32 bit version, running on an M2 macbook pro (single threaded):

- Poseidon:  
    - 214 msec for 10,000 `t=2` permutations
    - 314 msec for 10,000 `t=3` permutations
    - 414 msec for 10,000 `t=4` permutations
- Poseidon2: 
    - 124 msec for 10,000 `t=2` permutations
    - 142 msec for 10,000 `t=3` permutations
    - 271 msec for 10,000 `t=4` permutations

### TODO

- [x] clean up the code and make it more idiomatic
- [x] implement `circomlib`-compatible Poseidon
- [x] benchmark RISC-V cycles
- [x] add a proper test-suite; in particular, more complete testing of the field operations
- [x] add more Poseidon2 state widths (not just `t=3`)
- [ ] add more tests for the corner cases specifically
- [ ] implement the sponge construction
- [ ] add a 64 bit version
- [ ] optimize squaring to use less multiplications (?)
- [ ] investigate further optimization possibilities (?)

