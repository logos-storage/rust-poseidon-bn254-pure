
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_poseidon_bn254_pure::bn254::field::*;
use rust_poseidon_bn254_pure::poseidon2::permutation::*;

//------------------------------------------------------------------------------

type State = (Felt,Felt,Felt);

fn initial_state() -> State {
  ( Felt::from_u32(0)
  , Felt::from_u32(1)
  , Felt::from_u32(2)
  )
}

fn iterate_perm(n: usize) -> State {
  let mut state: State = initial_state();
  for _i in 0..n {
    state = permute_felt(&state);
  }
  state
}

// for a Merkle tree update with depth 20, we need 20 permutation calls
fn twenty_permutations() -> State {
  let mut state: State = initial_state();
  iterate_perm(20);
  state
}

fn bench_iterated_perm(c: &mut Criterion , n: usize) {
  let msg = format!("Poseidon2 permutation iterated {} times", n);
  c.bench_function(&msg, |b| b.iter(|| iterate_perm(black_box(n)) ));
}

fn bench_twenty(c: &mut Criterion) {
  let msg = format!("Poseidon2 permutation iterated 20 times");
  c.bench_function(&msg, |b| b.iter(|| twenty_permutations() ));
}

//------------------------------------------------------------------------------

fn bench_permutations(c: &mut Criterion) {
  bench_iterated_perm(c, 1000);
  bench_twenty(c);
}

//------------------------------------------------------------------------------

criterion_group!(benches, bench_permutations);
criterion_main!(benches);

