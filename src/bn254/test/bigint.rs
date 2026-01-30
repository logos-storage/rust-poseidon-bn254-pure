
// tests for bigints

#![allow(unused)]

use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

use crate::bn254::traits::*;
use crate::bn254::bigint::*;
use crate::bn254::test::properties::*;

type Big  = BigInt<8>;
type Big7 = BigInt<7>;
type Big9 = BigInt<9>;

//------------------------------------------------------------------------------
// some trivialities to get it started

#[test]
fn zero_is_zero()    { assert!(  Big::is_zero( Big::zero() ) ) }

#[test]
fn zero_is_not_one() { assert!( !Big::is_one ( Big::zero() ) ) }

#[test]
fn one_is_not_zero() { assert!( !Big::is_zero( Big::one () ) ) }

#[test]
fn one_is_one()      { assert!(  Big::is_one ( Big::one () ) ) }

//------------------------------------------------------------------------------

#[test]
fn unit_to_decimal() {
  let x: Big = Big::make( [ 0xff74e7f5 , 0x86ab86c2 , 0x7829f01b , 0x6dff3d9f , 0x7c6194d1 , 0x58fce839 , 0x1c3fc759 , 0x0ee7c7b9 ] );
  assert_eq!( Big::to_decimal_string(x) , "6741899990217662167434591118162422674873486558668509109681649862022285027317" )
}

#[test]
fn unit_to_hex() {
  let x: Big = Big::make( [ 0x5d8aa877 , 0x40b543d5 , 0x115812cd , 0x0563e2bd , 0x26c9552a , 0x1890edd6 , 0x803b772b , 0x1a12005c ] );
  assert_eq!( Big::to_hex_string(x) , "0x1a12005c803b772b1890edd626c9552a0563e2bd115812cd40b543d55d8aa877" )
}

//------------------------------------------------------------------------------

fn prop_from_to_bytes_le<const N: usize>(x: BigInt<N>) -> bool where [(); 4*N]: {
  let bs: [u8; 4*N] = BigInt::to_le_bytes(x);
  BigInt::<N>::from_le_bytes(bs) == x
}

fn prop_to_from_bytes_le<const N: usize>(bs: [u8; 4*N]) -> bool {
  let y: BigInt<N> = BigInt::from_le_bytes(bs);
  BigInt::<N>::to_le_bytes(y) == bs
}

fn prop_from_to_bytes_be<const N: usize>(x: BigInt<N>) -> bool where [(); 4*N]: {
  let bs: [u8; 4*N] = BigInt::to_be_bytes(x);
  BigInt::<N>::from_be_bytes(bs) == x
}

fn prop_to_from_bytes_be<const N: usize>(bs: [u8; 4*N]) -> bool {
  let y: BigInt<N> = BigInt::from_be_bytes(bs);
  BigInt::<N>::to_be_bytes(y) == bs
}

//------------------------------------------------------------------------------
// quickcheck property-based testing

impl<const N: usize> Group for BigInt<N> {}

// ...as you apparently cannot have an impl for an array...
#[derive(Debug, Copy, Clone)]
struct ByteArray<const K: usize>([u8; K]);

impl<const K: usize> Arbitrary for ByteArray<K> {
  fn arbitrary(g: &mut Gen) -> ByteArray<K> {
    let mut bs: [u8; K] = [0; K];
    for i in 0..K {
      bs[i] = u8::arbitrary(g);
    }
    ByteArray(bs)
  }
}

impl<const N: usize> Arbitrary for BigInt<N> {
  fn arbitrary(g: &mut Gen) -> BigInt<N> {
    let mut xs: [u32; N] = [0; N];
    for i in 0..N {
      xs[i] = u32::arbitrary(g);
    }
    BigInt::make(xs)
  }
}

//--------------------------------------

#[quickcheck]
fn from_to_bytes_le(x: Big7) -> bool { prop_from_to_bytes_le::<7>(x) }

#[quickcheck]
fn from_to_bytes_be(x: Big7) -> bool { prop_from_to_bytes_be::<7>(x) }

#[quickcheck]
fn to_fom_bytes_le(bs: ByteArray<{7*4}>) -> bool { prop_to_from_bytes_le::<7>(bs.0) }

#[quickcheck]
fn to_fom_bytes_be (bs: ByteArray<{7*4}>) -> bool { prop_to_from_bytes_be::<7>(bs.0) }

//--------------------------------------

#[quickcheck]
fn left_additive_unit(x: Big) -> bool { prop_left_additive_unit(x) }

#[quickcheck]
fn right_additive_unit(x: Big) -> bool { prop_right_additive_unit(x) }

#[quickcheck]
fn sub_zero(x: Big) -> bool { prop_sub_zero(x) }

#[quickcheck]
fn zero_sub(x: Big) -> bool { prop_zero_sub(x) }

#[quickcheck]
fn add_commutative(x: Big, y: Big) -> bool { prop_add_commutative(x,y) }

#[quickcheck]
fn sub_anticommutative(x: Big, y: Big) -> bool { prop_sub_anticommutative(x,y) }

#[quickcheck]
fn neg_involutive(x: Big) -> bool { prop_neg_involutive(x) }

#[quickcheck]
fn add_sub(x: Big, y: Big) -> bool { prop_add_sub(x,y) }

#[quickcheck]
fn sub_add(x: Big, y: Big) -> bool { prop_sub_add(x,y) }

#[quickcheck]
fn sub_neg_add(x: Big, y: Big) -> bool { prop_sub_neg_add(x,y) }

#[quickcheck]
fn sub_add_neg(x: Big, y: Big) -> bool { prop_sub_add_neg(x,y) }

//------------------------------------------------------------------------------

