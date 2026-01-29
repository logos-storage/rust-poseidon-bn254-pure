
//
// circomlib-compatible Poseidon (v1) implementation
//

#![allow(unused)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::bn254::field::*;
use crate::bn254::montgomery::*;

use crate::poseidon::constants::t2;
use crate::poseidon::constants::t3;
use crate::poseidon::constants::t4;
use crate::poseidon::constants::t5;

//------------------------------------------------------------------------------

// number of internal rounds for `t = 2..17`
const INTERNAL_ROUND_COUNT: [usize; 16] = [56, 57, 56, 60, 60, 63, 64, 63, 60, 66, 60, 65, 70, 60, 64, 68];

pub trait PoseidonParams<const T: usize> {
  const NP: usize;
  fn const_C() -> &'static [Mont];
  fn const_M() -> &'static [Mont];
  fn const_P() -> &'static [Mont];
  fn const_S() -> &'static [Mont];
}

pub struct Params;

macro_rules! impl_params {
  ($T:literal, $mod:ident) => {
    impl PoseidonParams<$T> for Params {
      const NP: usize = INTERNAL_ROUND_COUNT[$T - 2];
      fn const_C() -> &'static [Mont] { &$mod::CONST_C }
      fn const_M() -> &'static [Mont] { &$mod::CONST_M }
      fn const_P() -> &'static [Mont] { &$mod::CONST_P }
      fn const_S() -> &'static [Mont] { &$mod::CONST_S }
    }
  };
}

impl_params!(2, t2);
impl_params!(3, t3);
impl_params!(4, t4);
impl_params!(5, t5);

//------------------------------------------------------------------------------

#[inline(always)]
fn sbox(x: Mont) -> Mont {
  let x2 = Mont::sqr(x );
  let x4 = Mont::sqr(x2);
  Mont::mul(x,x4)
}

fn matrix_mul<const T: usize>(input: [Mont; T], mtx: &[Mont]) -> [Mont; T] {
  let mut out: [Mont; T] = [Mont::zero(); T];
  for i in 0..T {
    let mut acc: Mont = Mont::zero();
    for j in 0..T {
      acc = Mont::mulAdd( mtx[j*T+i] , input[j] , acc );
    }
    out[i] = acc;
  }
  out
}

fn mix_S<const T: usize>(input: [Mont; T], scoeffs: &[Mont]) -> [Mont; T] { 
  let mut out: [Mont; T] = [Mont::zero(); T];
  let mut acc: Mont = Mont::zero();
  for j in 0..T {
    acc = Mont::mulAdd( scoeffs[j] , input[j] , acc );
  }
  out[0] = acc;
  for j in 1..T {
    out[j] = Mont::mulAdd( scoeffs[T+j-1] , input[0] , input[j] );
  }
  out
}

fn internal_round<const T: usize>(input: [Mont; T], rc: Mont, scoeffs: &[Mont]) -> [Mont; T] {
  let mut xs: [Mont; T] = input;
  xs[0] = Mont::add( sbox( xs[0] ) , rc );
  mix_S::<T>(xs, scoeffs)
}

fn external_round<const T: usize>(input: [Mont; T], rcs: &[Mont], mtx: &[Mont]) -> [Mont; T] {
  let mut xs: [Mont; T] = [Mont::zero(); T];
  for j in 0..T {
    xs[j] = Mont::add( sbox( input[j] ) , rcs[j] );
  }
  matrix_mul::<T>(xs, mtx)
}

//------------------------------------------------------------------------------

pub fn permute_mont<const T: usize>(input: [Mont; T]) -> [Mont; T] where Params: PoseidonParams<T> {

  let TT = 2*T - 1;
  let NP = <Params as PoseidonParams<T>>::NP;
  let C  = <Params as PoseidonParams<T>>::const_C();
  let M  = <Params as PoseidonParams<T>>::const_M();
  let P  = <Params as PoseidonParams<T>>::const_P();
  let S  = <Params as PoseidonParams<T>>::const_S();

  let mut state: [Mont; T] = input;
  for j in 0..T { 
    state[j] = Mont::add( state[j] , C[j] );
  }
  for i in 0..4  { 
    let rcs: &[Mont] = &C[ ((i+1)*T) .. ((i+2)*T) ];
    let mat = if i<3 { M } else { P };
    state = external_round::<T>( state , rcs , mat ); 
  }
  for i in 0..NP { 
    let rc: Mont = C[ i + 5*T ];
    let scoeffs: &[Mont]  = &S[ (i*TT) .. ((i+1)*TT) ];
    state = internal_round::<T>( state , rc , scoeffs );
  }
  for i in 4..8  { 
    let rcs: &[Mont] = if i<7  { &C[ (NP + (i+1)*T) .. (NP + (i+2)*T) ] } else { &[Mont::zero(); T] };
    state = external_round::<T>( state , rcs , M ); 
  }
  state
}

pub fn compress_mont<const K: usize>(input: [Mont; K]) -> Mont where Params: PoseidonParams<{K+1}> {
  let mut state: [Mont; K+1] = [Mont::zero(); K+1]; 
  for i in 0..K { state[i+1] = input[i]; }
  state = permute_mont::<{K+1}>(state);
  state[0]
}

//------------------------------------------------------------------------------

pub fn permute<const T: usize>(input: [Felt; T]) -> [Felt; T] where Params: PoseidonParams<T> {
  let state: [Mont; T] = Felt::to_mont_vec(input);
  let output = permute_mont::<T>(state);
  Felt::from_mont_vec(output)
}

pub fn compress<const K: usize>(input: [Felt; K]) -> Felt where Params: PoseidonParams<{K+1}> {
  let mut state: [Mont; K+1] = [Mont::zero(); K+1]; 
  for i in 0..K { state[i+1] = Felt::to_mont(input[i]); }
  state = permute_mont::<{K+1}>(state);
  Felt::from_mont(state[0])
}

//------------------------------------------------------------------------------

pub fn hash1(a: Felt) -> Felt {
  compress::<1>([ a ])
}

pub fn hash2(a: Felt, b: Felt) -> Felt {
  compress::<2>([ a, b ])
}

pub fn hash3(a: Felt, b: Felt, c: Felt) -> Felt {
  compress::<3>([ a, b, c ])
}

pub fn hash4(a: Felt, b: Felt, c: Felt, d: Felt) -> Felt {
  compress::<4>([ a, b, c, d ])
}

//------------------------------------------------------------------------------
