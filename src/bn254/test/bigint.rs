
// tests for bigints

#![allow(unused)]

use crate::bn254::bigint::*;
use crate::bn254::test::properties::*;

type Big = BigInt<8>;

//------------------------------------------------------------------------------

#[test]
fn zero_is_zero() {
  assert!( Big::is_zero( Big::zero() ) ) 
}

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
