rust-poseidon-bn254-rv32im
--------------------------

Self-contained Rust implementation of Poseidon and Poseidon2 hash functions
over the BN254 curve's scalar field, using 32 bit limbs internally.

It's intended to be used on 32-bit platforms eg. 32-bit Risc-V (porting to 64 bits
shouldn't be a big effort though).

The algebra implementation is based on [`zikkurat-algebra`](https://github.com/faulhornlabs/zikkurat-algebra/).
