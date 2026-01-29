
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::{black_box};

use rust_poseidon_bn254_pure::bn254::field::*;
use rust_poseidon_bn254_pure::poseidon;
use rust_poseidon_bn254_pure::poseidon2;

//------------------------------------------------------------------------------

type Triple = [Felt; 3];

fn initial_triple() -> Triple {
  [ Felt::from_u32(0)
  , Felt::from_u32(1)
  , Felt::from_u32(2)
  ]
}

fn iterate_poseidon1(n: usize) -> Triple {
  let mut state: Triple = initial_triple();
  for _i in 0..n {
    state = poseidon::permute::<3>(state);
  }
  state
}

fn iterate_poseidon2(n: usize) -> Triple {
  let mut state: Triple = initial_triple();
  for _i in 0..n {
    state = poseidon2::permute(state);
  }
  state
}

fn bench_iterated_poseidon1(c: &mut Criterion , n: usize) {
  let msg = format!("Poseidon1 permutation iterated {} times", n);
  c.bench_function(&msg, |b| b.iter(|| iterate_poseidon1(black_box(n)) ));
}

fn bench_iterated_poseidon2(c: &mut Criterion , n: usize) {
  let msg = format!("Poseidon2 permutation iterated {} times", n);
  c.bench_function(&msg, |b| b.iter(|| iterate_poseidon2(black_box(n)) ));
}

//------------------------------------------------------------------------------

fn bench_permutations(c: &mut Criterion) {
  bench_iterated_poseidon1(c, 10000);
  bench_iterated_poseidon2(c, 10000);
}

//------------------------------------------------------------------------------

criterion_group!(benches, bench_permutations);
criterion_main!(benches);

