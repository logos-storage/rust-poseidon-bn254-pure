
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::{black_box};

use rust_poseidon_bn254_pure::bn254::field::*;
use rust_poseidon_bn254_pure::bn254::montgomery::{Mont};
use rust_poseidon_bn254_pure::poseidon;
use rust_poseidon_bn254_pure::poseidon2;

//------------------------------------------------------------------------------

type Triple = (Felt,Felt,Felt);

fn initial_triple() -> Triple {
  ( Felt::from_u32(0)
  , Felt::from_u32(1)
  , Felt::from_u32(2)
  )
}

fn initial_vector() -> [Felt; 3] {
  [ Felt::from_u32(0)
  , Felt::from_u32(1)
  , Felt::from_u32(2)
  ]
}

pub fn poseidon1_permute_felt(input: [Felt; 3]) -> [Felt; 3] {
  let mut state: [Mont; 3] = 
    [ Felt::to_mont(input[0])
    , Felt::to_mont(input[1])
    , Felt::to_mont(input[2])
    ];
  state = poseidon::permutation::permute_mont_T3(state);
  let out: [Felt; 3] = 
    [ Felt::from_mont(state[0])
    , Felt::from_mont(state[1])
    , Felt::from_mont(state[2])
    ];
  out
}

fn iterate_poseidon1(n: usize) -> [Felt; 3] {
  let mut state: [Felt; 3] = initial_vector();
  for _i in 0..n {
    state = poseidon1_permute_felt(state);
  }
  state
}

fn iterate_poseidon2(n: usize) -> Triple {
  let mut state: Triple = initial_triple();
  for _i in 0..n {
    state = poseidon2::permutation::permute_felt(state);
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

