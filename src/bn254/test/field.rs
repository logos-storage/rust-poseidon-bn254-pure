
//
// tests for fields (standard representation)
//

#![allow(unused)]

use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

use crate::bn254::traits::*;
use crate::bn254::constant::*;
use crate::bn254::bigint::*;
use crate::bn254::field::*;
use crate::bn254::test::properties::*;

//------------------------------------------------------------------------------
// some trivialities to get it started

#[test]
fn zero_is_zero()    { assert!(  Felt::is_zero( Felt::zero() ) ) }

#[test]
fn zero_is_not_one() { assert!( !Felt::is_one ( Felt::zero() ) ) }

#[test]
fn one_is_not_zero() { assert!( !Felt::is_zero( Felt::one () ) ) }

#[test]
fn one_is_one()      { assert!(  Felt::is_one ( Felt::one () ) ) }

//------------------------------------------------------------------------------
// quickcheck property-based testing

impl Group for Felt {}
impl Ring  for Felt {}
impl Field for Felt {}

// aux helper for arbitrary
#[derive(Debug, Copy, Clone)]
struct Masked(BigInt256);

impl Arbitrary for Masked {
  fn arbitrary(g: &mut Gen) -> Masked {
    let mut xs: [u32; 8] = [0; 8];
    for i in 0..8 {
      xs[i] = u32::arbitrary(g);
    }
    xs[7] = xs[7] & 0x_3FFF_FFFF;
    Masked(BigInt::make(xs))
  }
}

// rejection sampling
impl Arbitrary for Felt {
  fn arbitrary(g: &mut Gen) -> Felt {
    let mut mx: Masked = Masked::arbitrary(g);
    while( mx.0 >= FIELD_PRIME ) {
      mx = Masked::arbitrary(g);
    }
    Felt::unsafe_from_bigint(mx.0)
  }
}

//--------------------------------------

#[quickcheck]
fn from_to_bytes_le(x: Felt) -> bool {
  let bs: [u8; 32] = Felt::to_le_bytes(x);
  Felt::unsafe_from_le_bytes(bs) == x
}

#[quickcheck]
fn from_to_bytes_be(x: Felt) -> bool {
  let bs: [u8; 32] = Felt::to_be_bytes(x);
  Felt::unsafe_from_be_bytes(bs) == x
}
//--------------------------------------

#[quickcheck]
fn left_additive_unit(x: Felt) -> bool { prop_left_additive_unit(x) }

#[quickcheck]
fn right_additive_unit(x: Felt) -> bool { prop_right_additive_unit(x) }

#[quickcheck]
fn sub_zero(x: Felt) -> bool { prop_sub_zero(x) }

#[quickcheck]
fn zero_sub(x: Felt) -> bool { prop_zero_sub(x) }

#[quickcheck]
fn add_commutative(x: Felt, y: Felt) -> bool { prop_add_commutative(x,y) }

#[quickcheck]
fn sub_anticommutative(x: Felt, y: Felt) -> bool { prop_sub_anticommutative(x,y) }

#[quickcheck]
fn add_associative(x: Felt, y: Felt, z: Felt) -> bool { prop_add_associative(x,y,z) }

#[quickcheck]
fn neg_involutive(x: Felt) -> bool { prop_neg_involutive(x) }

#[quickcheck]
fn add_sub(x: Felt, y: Felt) -> bool { prop_add_sub(x,y) }

#[quickcheck]
fn sub_add(x: Felt, y: Felt) -> bool { prop_sub_add(x,y) }

#[quickcheck]
fn sub_neg_add(x: Felt, y: Felt) -> bool { prop_sub_neg_add(x,y) }

#[quickcheck]
fn sub_add_neg(x: Felt, y: Felt) -> bool { prop_sub_add_neg(x,y) }

//------------------------------------------------------------------------------

#[quickcheck]
fn halve_double(x: Felt) -> bool {
  let y = Felt::div_by_2(x);
  x == y + y
}

#[quickcheck]
fn double_halve(x: Felt) -> bool {
  let z = Felt::div_by_2(x + x);
  x == z
}

#[quickcheck]
fn halve_definition(x: Felt) -> bool {
  let a = Felt::div_by_2(x);
  let b = x / Felt::from_u32(2);
  a == b
}

//------------------------------------------------------------------------------

#[quickcheck]
fn twice(x: Felt) -> bool { prop_twice(x) }

#[quickcheck]
fn thrice(x: Felt) -> bool { prop_thrice(x) }

//--------------------------------------

#[quickcheck]
fn left_multiplicative_unit(x: Felt) -> bool { prop_left_multiplicative_unit(x) }

#[quickcheck]
fn right_multiplicative_unit(x: Felt) -> bool { prop_right_multiplicative_unit(x) }

#[quickcheck]
fn mul_commutative(x: Felt, y: Felt) -> bool { prop_mul_commutative(x,y) }

#[quickcheck]
fn mul_associative(x: Felt, y: Felt, z: Felt) -> bool { prop_mul_associative(x,y,z) }

#[quickcheck]
fn mul_neg(x: Felt, y: Felt) -> bool { prop_mul_neg(x,y) }

#[quickcheck]
fn distributive_add(x: Felt, y: Felt, z: Felt) -> bool { prop_distributive_add(x,y,z) }

#[quickcheck]
fn distributive_sub(x: Felt, y: Felt, z: Felt) -> bool { prop_distributive_sub(x,y,z) }

//------------------------------------------------------------------------------

#[quickcheck]
fn div_by_1(x: Felt) -> bool { prop_div_by_1(x) }

#[quickcheck]
fn inv_def(x: Felt) -> bool { prop_inv_def(x) }

#[quickcheck]
fn mul_left_inverse(x: Felt) -> bool { prop_mul_left_inverse(x) }

#[quickcheck]
fn mul_right_inverse(x: Felt) -> bool { prop_mul_right_inverse(x) }

#[quickcheck]
fn mul_div(x: Felt, y: Felt) -> bool { prop_mul_div(x,y) }

#[quickcheck]
fn div_mul(x: Felt, y: Felt) -> bool { prop_div_mul(x,y) }

#[quickcheck]
fn distributive_div(x: Felt, y: Felt, z: Felt) -> bool { prop_distributive_div(x,y,z) }

#[quickcheck]
fn div_div(x: Felt, y: Felt, z: Felt) -> bool { prop_div_div(x,y,z) }

//------------------------------------------------------------------------------
