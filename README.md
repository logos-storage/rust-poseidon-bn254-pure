rust-poseidon-bn254-rv32im
--------------------------

Self-contained, pure Rust implementation of Poseidon and Poseidon2 hash functions
over the BN254 curve's scalar field, using 32 bit limbs internally.

It's intended to be used on 32-bit platforms eg. 32-bit RISC-V (though porting 
to 64 bits shouldn't be a big effort).

The algebra implementation is based on [`zikkurat-algebra`](https://github.com/faulhornlabs/zikkurat-algebra/)
and [`staging-agda`](https://github.com/faulhornlabs/staging-agda/).

### TODO

- [ ] optimize squaring to use less multiplication
- [ ] benchmark RISC-V cycles
- [ ] add a 64 bit version
- [ ] add more Poseidon2 state widths than `t=3`
- [ ] implement `circomlib`-compatible Poseidon

