
#![allow(unused)]

//
// circomlib-compatible Poseidon (v1) implementation
//

#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::bn254::field::*;
use crate::bn254::montgomery::*;

use crate::poseidon::constants::t2;
use crate::poseidon::constants::t3;
use crate::poseidon::constants::t4;
use crate::poseidon::constants::t5;

//------------------------------------------------------------------------------

// width of the permutation state
#[derive(Copy, Clone)]
#[repr(usize)]
pub enum Width {
  T2 = 2,
  T3 = 3,
  T4 = 4,
  T5 = 5,
}

//------------------------------------------------------------------------------

// number of internal rounds for `t = 2..17`
const INTERNAL_ROUND_COUNT: [usize; 16] = [56, 57, 56, 60, 60, 63, 64, 63, 60, 66, 60, 65, 70, 60, 64, 68];

const fn internal_round_count(T: usize) -> usize {
  let k = T - 2;
  if k < 16 { 
    INTERNAL_ROUND_COUNT[ k - 2 ]
  }
  else {
    0
  }
}

//------------------------------------------------------------------------------

#[inline(always)]
fn sbox(x: Mont) -> Mont {
  let x2 = Mont::sqr(x );
  let x4 = Mont::sqr(x2);
  Mont::mul(x,x4)
}

fn matrix_mul<const T: usize>(input: [Mont; T], mtx: [Mont; T*T]) -> [Mont; T] {
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

fn internal_round<const T: usize>(rc: Mont, scoeffs: &[Mont], input: [Mont; T]) -> [Mont; T] {
  let mut xs: [Mont; T] = input;
  xs[0] = Mont::add( sbox( xs[0] ) , rc );
  mix_S::<T>(xs, scoeffs)
}

fn external_round<const T: usize>(rcs: &[Mont], input: [Mont; T], mtx: [Mont; T*T]) -> [Mont; T] {
  let mut xs: [Mont; T] = [Mont::zero(); T];
  for j in 0..T {
    xs[j] = Mont::add( sbox( input[j] ) , rcs[j] );
  }
  matrix_mul::<T>(xs, mtx)
}

//------------------------------------------------------------------------------
// TODO: can we somehow unify the different T cases????

/*
// debugging
fn printRound(text: &str, round: usize, state: &[Mont]) {
  println!("{} {} -> ", text, round);
  for x in state {
    println!("  {}", Mont::to_decimal_string(x) );
  }
}
*/

//--------------------------------------
// T = 2

pub fn permute_mont_T2(input: [Mont; 2]) -> [Mont; 2] {
  const T:  usize = 2;

  const TT: usize = 2*T-1;
  const NP: usize = INTERNAL_ROUND_COUNT[T-2];
  const C:  [Mont;  72] = t2::CONST_C;
  const M:  [Mont;   4] = t2::CONST_M;
  const P:  [Mont;   4] = t2::CONST_P;
  const S:  [Mont; 168] = t2::CONST_S;

  let mut state: [Mont; T] = input;
  for j in 0..T { 
    state[j] = Mont::add( state[j] , C[j] );
  }
  for i in 0..4  { 
    let rcs: &[Mont] = &C[ ((i+1)*T) .. ((i+2)*T) ];
    let mat = if i<3 { M } else { P };
    state = external_round::<T>( rcs , state , mat ); 
    // printRound("initial round", i, &state); 
  }
  for i in 0..NP { 
    let rc: Mont = C[ i + 5*T ];
    let scoeffs: &[Mont]  = &S[ (i*TT) .. ((i+1)*TT) ];
    state = internal_round::<T>( rc , scoeffs , state );
    // printRound("internal round", i, &state); 
  }
  for i in 4..8  { 
    let rcs: &[Mont] = if i<7  { &C[ (NP + (i+1)*T) .. (NP + (i+2)*T) ] } else { &[Mont::zero(); T] };
    state = external_round::<T>( rcs , state , M ); 
    // printRound("final round", i, &state); 
  }
  state
}

//--------------------------------------

pub fn permute_mont_T3(input: [Mont; 3]) -> [Mont; 3] {
  const T:  usize = 3;

  const TT: usize = 2*T-1;
  const NP: usize = INTERNAL_ROUND_COUNT[T-2];
  const C:  [Mont;  81] = t3::CONST_C;
  const M:  [Mont;   9] = t3::CONST_M;
  const P:  [Mont;   9] = t3::CONST_P;
  const S:  [Mont; 285] = t3::CONST_S;

  let mut state: [Mont; T] = input;
  for j in 0..T { 
    state[j] = Mont::add( state[j] , C[j] );
  }
  for i in 0..4  { 
    let rcs: &[Mont] = &C[ ((i+1)*T) .. ((i+2)*T) ];
    let mat = if i<3 { M } else { P };
    state = external_round::<T>( rcs , state , mat ); 
    // printRound("initial round", i, &state); 
  }
  for i in 0..NP { 
    let rc: Mont = C[ i + 5*T ];
    let scoeffs: &[Mont]  = &S[ (i*TT) .. ((i+1)*TT) ];
    state = internal_round::<T>( rc , scoeffs , state );
    // printRound("internal round", i, &state); 
  }
  for i in 4..8  { 
    let rcs: &[Mont] = if i<7  { &C[ (NP + (i+1)*T) .. (NP + (i+2)*T) ] } else { &[Mont::zero(); T] };
    state = external_round::<T>( rcs , state , M ); 
    // printRound("final round", i, &state); 
  }
  state
}

//--------------------------------------

pub fn permute_mont_T4(input: [Mont; 4]) -> [Mont; 4] {
  const T:  usize = 4;

  const TT: usize = 2*T-1;
  const NP: usize = INTERNAL_ROUND_COUNT[T-2];
  const C:  [Mont;  88] = t4::CONST_C;
  const M:  [Mont;  16] = t4::CONST_M;
  const P:  [Mont;  16] = t4::CONST_P;
  const S:  [Mont; 392] = t4::CONST_S;

  let mut state: [Mont; T] = input;
  for j in 0..T { 
    state[j] = Mont::add( state[j] , C[j] );
  }
  for i in 0..4  { 
    let rcs: &[Mont] = &C[ ((i+1)*T) .. ((i+2)*T) ];
    let mat = if i<3 { M } else { P };
    state = external_round::<T>( rcs , state , mat ); 
    // printRound("initial round", i, &state); 
  }
  for i in 0..NP { 
    let rc: Mont = C[ i + 5*T ];
    let scoeffs: &[Mont]  = &S[ (i*TT) .. ((i+1)*TT) ];
    state = internal_round::<T>( rc , scoeffs , state );
    // printRound("internal round", i, &state); 
  }
  for i in 4..8  { 
    let rcs: &[Mont] = if i<7  { &C[ (NP + (i+1)*T) .. (NP + (i+2)*T) ] } else { &[Mont::zero(); T] };
    state = external_round::<T>( rcs , state , M ); 
    // printRound("final round", i, &state); 
  }
  state
}

//--------------------------------------

pub fn permute_mont_T5(input: [Mont; 5]) -> [Mont; 5] {
  const T:  usize = 5;

  const TT: usize = 2*T-1;
  const NP: usize = INTERNAL_ROUND_COUNT[T-2];
  const C:  [Mont; 100] = t5::CONST_C;
  const M:  [Mont;  25] = t5::CONST_M;
  const P:  [Mont;  25] = t5::CONST_P;
  const S:  [Mont; 540] = t5::CONST_S;

  let mut state: [Mont; T] = input;
  for j in 0..T { 
    state[j] = Mont::add( state[j] , C[j] );
  }
  for i in 0..4  { 
    let rcs: &[Mont] = &C[ ((i+1)*T) .. ((i+2)*T) ];
    let mat = if i<3 { M } else { P };
    state = external_round::<T>( rcs , state , mat ); 
    // printRound("initial round", i, &state); 
  }
  for i in 0..NP { 
    let rc: Mont = C[ i + 5*T ];
    let scoeffs: &[Mont]  = &S[ (i*TT) .. ((i+1)*TT) ];
    state = internal_round::<T>( rc , scoeffs , state );
    // printRound("internal round", i, &state); 
  }
  for i in 4..8  { 
    let rcs: &[Mont] = if i<7  { &C[ (NP + (i+1)*T) .. (NP + (i+2)*T) ] } else { &[Mont::zero(); T] };
    state = external_round::<T>( rcs , state , M ); 
    // printRound("final round", i, &state); 
  }
  state
}

//------------------------------------------------------------------------------

pub fn compress_1(input: Felt) -> Felt {
  let mut state: [Mont; 2] = 
    [ Mont::zero()
    , Felt::to_mont(input)
    ]; 
  state = permute_mont_T2(state);
  Felt::from_mont(state[0])
}

pub fn compress_2(input: [Felt;2]) -> Felt {
  let mut state: [Mont; 3] = 
    [ Mont::zero()
    , Felt::to_mont(input[0])
    , Felt::to_mont(input[1])
    ]; 
  state = permute_mont_T3(state);
  Felt::from_mont(state[0])
}

pub fn compress_3(input: [Felt;3]) -> Felt {
  let mut state: [Mont; 4] = 
    [ Mont::zero()
    , Felt::to_mont(input[0])
    , Felt::to_mont(input[1])
    , Felt::to_mont(input[2])
    ]; 
  state = permute_mont_T4(state);
  Felt::from_mont(state[0])
}

pub fn compress_4(input: [Felt;4]) -> Felt {
  let mut state: [Mont; 5] = 
    [ Mont::zero()
    , Felt::to_mont(input[0])
    , Felt::to_mont(input[1])
    , Felt::to_mont(input[2])
    , Felt::to_mont(input[3])
    ]; 
  state = permute_mont_T5(state);
  Felt::from_mont(state[0])
}

//------------------------------------------------------------------------------
