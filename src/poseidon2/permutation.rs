
#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::bn254::field::*;
use crate::bn254::montgomery::*;

use crate::poseidon2::constants::old;
use crate::poseidon2::constants::new;

use crate::poseidon2::mds;
use crate::poseidon2::diag;

//------------------------------------------------------------------------------

pub struct Params;

pub trait Poseidon2Params<const NEW: bool, const T: usize> {
  const NP: usize;
  fn const_initial () -> &'static [Mont];
  fn const_internal() -> &'static [Mont];
  fn const_final   () -> &'static [Mont];
  fn const_KAT     () -> &'static [Mont];
  fn mul_by_mds    ( xs: [Mont; T] ) -> [Mont; T];
  fn mul_by_diag   ( xs: [Mont; T] ) -> [Mont; T];
}

macro_rules! impl_params {
  ($NEW:literal, $T:literal, $oldnew:ident, $tmod:ident) => {
    impl Poseidon2Params<$NEW,$T> for Params {
      const NP: usize = 56;
      fn const_initial () -> &'static [Mont] { &$oldnew::$tmod::INITIAL  }
      fn const_internal() -> &'static [Mont] { &$oldnew::$tmod::INTERNAL }
      fn const_final   () -> &'static [Mont] { &$oldnew::$tmod::FINAL    }
      fn const_KAT     () -> &'static [Mont] { &$oldnew::$tmod::KAT_MONT }
      fn mul_by_mds ( xs: [Mont; $T] ) -> [Mont; $T] { mds::$tmod::mds(xs) }
      fn mul_by_diag( xs: [Mont; $T] ) -> [Mont; $T] { diag::$oldnew::$tmod::diag(xs) }
    }
  };
}

// old parameters
impl_params!( false, 2, old, t2 );
impl_params!( false, 3, old, t3 );
impl_params!( false, 4, old, t4 );

// new parameters
impl_params!( true, 2, new, t2 );
impl_params!( true, 3, new, t3 );
impl_params!( true, 4, new, t4 );

//------------------------------------------------------------------------------

#[inline(always)]
fn get_initial_rcs<const NEW: bool, const T: usize>(round: usize) -> [Mont; T] where Params: Poseidon2Params<NEW,T> {
  let mut rcs: [Mont; T] = [Default::default(); T];
  let k = round * T;
  for i in 0..T {
    rcs[i] = <Params as Poseidon2Params<NEW,T>>::const_initial() [k+i];
  }
  rcs
}

#[inline(always)]
fn get_final_rcs<const NEW: bool, const T: usize>(round: usize) -> [Mont; T] where Params: Poseidon2Params<NEW,T> {
  let mut rcs: [Mont; T] = [Default::default(); T];
  let k = round * T;
  for i in 0..T {
    rcs[i] = <Params as Poseidon2Params<NEW,T>>::const_final() [k+i];
  }
  rcs
}

#[inline(always)]
fn get_internal_rc<const NEW: bool, const T: usize>(round: usize) -> Mont where Params: Poseidon2Params<NEW,T> {
  <Params as Poseidon2Params<NEW,T>>::const_internal()[ round ]
}

//------------------------------------------------------------------------------

#[inline(always)]
fn sbox(x: Mont) -> Mont {
  let x2 = Mont::sqr(x );
  let x4 = Mont::sqr(x2);
  Mont::mul(x,x4)
}

//------------------------------------------------------------------------------

#[inline(always)]
fn internal_round<const NEW: bool, const T: usize>(input: [Mont; T], rc: Mont) -> [Mont; T] 
where Params: Poseidon2Params<NEW,T> {
  let mut xs: [Mont; T] = input;
  xs[0] = sbox( Mont::add( xs[0] , rc ) );
  <Params as Poseidon2Params<NEW,T>>::mul_by_diag( xs )
}

fn external_round<const NEW: bool, const T: usize>(input: [Mont; T], rcs: [Mont;T]) -> [Mont; T] 
where Params: Poseidon2Params<NEW,T> {
  let mut xs: [Mont; T] = [Default::default(); T];
  for i in 0..T {
    xs[i] = sbox( Mont::add( input[i] , rcs[i] ) );
  }
  <Params as Poseidon2Params<NEW,T>>::mul_by_mds( xs )
}

pub fn permute_mont<const NEW: bool, const T: usize>(input: [Mont; T]) -> [Mont; T] 
where Params: Poseidon2Params<NEW,T> {
  let mut state = <Params as Poseidon2Params<NEW,T>>::mul_by_mds(input);
  for i in 0..4  { state = external_round::<NEW,T>( state , get_initial_rcs::<NEW,T>(i) ); }
  for i in 0..56 { state = internal_round::<NEW,T>( state , get_internal_rc::<NEW,T>(i) ); }
  for i in 0..4  { state = external_round::<NEW,T>( state , get_final_rcs  ::<NEW,T>(i) ); }
  state
}

//------------------------------------------------------------------------------

pub fn compress<const NEW: bool, const K: usize>(input: [Felt; K]) -> Felt 
where Params: Poseidon2Params<NEW,{K+1}> {
  let mut state: [Mont; K+1] = [Mont::zero(); K+1];
  for i in 0..K {
    state[i] = Felt::to_mont(input[i]);
  }
  state = permute_mont::<NEW,{K+1}>(state);
  Felt::from_mont(state[0])
}

pub fn permute<const NEW: bool, const T: usize>(input: [Felt; T]) -> [Felt; T] 
where Params: Poseidon2Params<NEW,T> {
  let state: [Mont; T] = Felt::to_mont_vec(input);
  let output = permute_mont::<NEW,T>(state);
  Felt::from_mont_vec(output) 
}

/*
pub fn permute_iterated<const NEW: bool, const T: usize>(input: [Felt; T], count: usize) -> [Felt; T] 
where Params: Poseidon2Params<NEW,T> {
  let mut state: MontTriple = Felt::to_mont_vec(input);
  for _i in 0..count { 
    state = permute_mont::<NEW,T>(state);
  }
  let out: FeltTriple = Felt::from_mont_vec(state);
  out
}
*/

//==============================================================================
// *** TESTS

/*
#[cfg(test)]
mod test {

  use crate::bn254::field::{Felt};
  use super::*;

  #[test]
  fn permute3_kat() {
    let out: [Felt; 3] = permute( [ 0u32.into() , 1u32.into() , 2u32.into() ] );
    println!(" 0 -> {}" , out[0] );
    println!(" 1 -> {}" , out[1] );
    println!(" 2 -> {}" , out[2] );
    assert_eq!( Felt::to_hex_string( out[0] ) , "0x30610a447b7dec194697fb50786aa7421494bd64c221ba4d3b1af25fb07bd103" );
    assert_eq!( Felt::to_hex_string( out[1] ) , "0x13f731d6ffbad391be22d2ac364151849e19fa38eced4e761bcd21dbdc600288" );
    assert_eq!( Felt::to_hex_string( out[2] ) , "0x1433e2c8f68382c447c5c14b8b3df7cbfd9273dd655fe52f1357c27150da786f" );
  }

}
*/

//------------------------------------------------------------------------------

