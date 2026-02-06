
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::{black_box};

use rust_poseidon_bn254_pure::bn254::field::*;

//------------------------------------------------------------------------------

type Triple = [Felt; 3];

fn initial_state<const T: usize>() -> [Felt; T] {
  let mut xs: [Felt; T] = [Default::default(); T];
  for i in 0..T {
    xs[i] = Felt::from_u32(i as u32);
  }
  xs
}

mod v1 {
  use super::*;
  use rust_poseidon_bn254_pure::poseidon::*;
  
  fn iterate_poseidon1<const T: usize>(n: usize) -> [Felt; T] where Params: PoseidonParams<T> {
    let mut state: [Felt; T] = initial_state::<T>();
    for _i in 0..n {
      state = permute::<T>(state);
    }
    state
  }

  pub fn bench_iterated_poseidon1<const T: usize>(c: &mut Criterion, n: usize) where Params: PoseidonParams<T> {
    let msg = format!("Poseidon1 permutation w/ state width t={} iterated {} times", T, n);
    c.bench_function(&msg, |b| b.iter(|| iterate_poseidon1::<T>(black_box(n)) ));
  }

}


mod v2 {
  use super::*;
  use rust_poseidon_bn254_pure::poseidon2::old::*;

  fn iterate_poseidon2<const T: usize>(n: usize) -> [Felt; T] where Params: Poseidon2Params<false,T> {
    let mut state: [Felt; T] = initial_state::<T>();
    for _i in 0..n {
      state = permute::<T>(state);
    }
    state
  }
  
  pub fn bench_iterated_poseidon2<const T: usize>(c: &mut Criterion, n: usize) where Params: Poseidon2Params<false,T> {
    let msg = format!("Poseidon2 permutation w/ state width t={} iterated {} times", T, n);
    c.bench_function(&msg, |b| b.iter(|| iterate_poseidon2::<T>(black_box(n)) ));
  }

}

//------------------------------------------------------------------------------

fn bench_permutations(c: &mut Criterion) {

  v1::bench_iterated_poseidon1::<2> (c, 10000);
  v1::bench_iterated_poseidon1::<3> (c, 10000);
  v1::bench_iterated_poseidon1::<4> (c, 10000);

  v2::bench_iterated_poseidon2::<2> (c, 10000);
  v2::bench_iterated_poseidon2::<3> (c, 10000);
  v2::bench_iterated_poseidon2::<4> (c, 10000);

}

//------------------------------------------------------------------------------

criterion_group!(benches, bench_permutations);
criterion_main!(benches);

