
#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::bn254::field::*;
use crate::bn254::montgomery::*;
use crate::poseidon2::constants::*;

pub type FeltTriple = [Felt; 3];
pub type MontTriple = [Mont; 3];

//------------------------------------------------------------------------------

#[inline(always)]
fn sbox(x: Mont) -> Mont {
  let x2 = Mont::sqr(x );
  let x4 = Mont::sqr(x2);
  Mont::mul(x,x4)
}

#[inline(always)]
fn add3(x: Mont, y: Mont, z: Mont) -> Mont {
  Mont::add(Mont::add(x,y),z)
}

fn linear(input: MontTriple) -> MontTriple {
  let s = add3( input[0], input[1], input[2] );
  [ Mont::add( s , input[0] )
  , Mont::add( s , input[1] )
  , Mont::add( s , input[2] )
  ]
}

fn internal_round(input: MontTriple, rc: Mont) -> MontTriple {
  let x = sbox( Mont::add( input[0] , rc ) );
  let s = add3( x , input[1] , input[2] );
  [ Mont::add( s , x                   )
  , Mont::add( s , input[1]            )
  , Mont::add( s , Mont::dbl(input[2]) )
  ]
}

fn external_round(input: MontTriple, rcs: MontTriple) -> MontTriple {
  let x = sbox( Mont::add( input[0] , rcs[0] ) );
  let y = sbox( Mont::add( input[1] , rcs[1] ) );
  let z = sbox( Mont::add( input[2] , rcs[2] ) );
  let s = add3( x , y , z );
  [ Mont::add( s , x )
  , Mont::add( s , y )
  , Mont::add( s , z )
  ]
}

pub fn permute_mont(input: MontTriple) -> MontTriple {
  let mut state = linear(input);
  for i in 0..4  { state = external_round( state , get_initial_RCs(i) ); }
  for i in 0..56 { state = internal_round( state , INTERNAL_MONT  [i] ); }
  for i in 0..4  { state = external_round( state , get_final_RCs  (i) ); }
  state
}

//------------------------------------------------------------------------------

pub fn compress(input: [Felt; 2]) -> Felt {
  let mut state: [Mont; 3] = [Mont::zero(); 3]; 
  for i in 0..2 { state[i] = Felt::to_mont(input[i]); }
  state = permute_mont(state);
  Felt::from_mont(state[0])
}

pub fn permute(input: [Felt; 3]) -> [Felt; 3] {
  let state: MontTriple = Felt::to_mont_vec(input);
  let output = permute_mont(state);
  Felt::from_mont_vec(output) 
}

pub fn permute_iterated(input: [Felt; 3], count: usize) -> [Felt; 3] {
  let mut state: MontTriple = Felt::to_mont_vec(input);
  for _i in 0..count { 
    state = permute_mont(state);
  }
  let out: FeltTriple = Felt::from_mont_vec(state);
  out
}

//------------------------------------------------------------------------------
