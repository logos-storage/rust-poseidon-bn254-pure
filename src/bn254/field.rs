
//
// standard representation of field elements
//
// note: this is used primarily for input/output; 
// for actual computations, use the Montgomery representation!
//

#![allow(dead_code)]
#![allow(non_snake_case)]

use std::fmt;
use std::ops::{Neg,Add,Sub,Mul};

use crate::bn254::bigint::*;
use crate::bn254::constant::*;
use crate::bn254::montgomery::*;

//------------------------------------------------------------------------------

type Big = BigInt<8>;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Felt(Big);

//------------------------------------------------------------------------------
// display traits

impl fmt::Display for Felt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_fmt(format_args!("{}",self.0))
  }
}

impl Felt {
  pub fn debug_print(s: &str, A: Felt) {
    println!("{} = {}", s, A);
  }
}

//------------------------------------------------------------------------------
// standard numeric traits

impl Neg for Felt {
  type Output = Self;
  fn neg(self) -> Self { Felt::neg(self) }
}

impl Add for Felt {
  type Output = Self;
  fn add(self, other: Self) -> Self { Felt::add(self, other) }
}

impl Sub for Felt {
  type Output = Self;
  fn sub(self, other: Self) -> Self { Felt::sub(self, other) }
}

impl Mul for Felt {
  type Output = Self;
  fn mul(self, other: Self) -> Self { Felt::mul(self, other) }
}

//------------------------------------------------------------------------------
// conversion traits

impl From<Mont> for Felt {
  fn from(mont: Mont) -> Self { Felt::from_mont(mont) }
}

impl Into<Mont> for Felt {
  fn into(self: Self) -> Mont { Felt::to_mont(self) }
}

// note: we dont implement `From<BigInt<8>>` as it's unsafe, 
// and we don't have a "safe" (modulo p) implementation yet

impl Into<BigInt<8>> for Felt {
  fn into(self: Self) -> BigInt<8> { Felt::to_bigint(self) }
}

//------------------------------------------------------------------------------
// small values

impl Default for Felt {
  fn default() -> Self { Felt(BigInt::zero()) }
}

impl From<u32> for Felt {
  fn from(x: u32) -> Self { Self::from_u32(x) }
}

//------------------------------------------------------------------------------
// internal implementations

impl Felt {

  #[inline(always)]
  pub fn to_bigint(felt: Felt) -> Big {
    felt.0
  }

  pub const fn unsafe_make( xs: [u32; 8] ) -> Felt {
    Felt(BigInt::from_limbs(xs))
  }

  pub fn checked_make( xs: [u32; 8] ) -> Felt {
    let big: Big = BigInt::make(xs);
    if BigInt::is_lt_prime(big) {
      Felt(big)
    }
    else {
      panic!("Felt::checked_make: not in range")
    }
  }

  pub fn is_valid(felt: Felt) -> bool {
    BigInt::is_lt_prime(felt.0)
  }

  //------------------------------------

  pub fn to_le_bytes(felt: Felt) -> [u8; 32] {
    BigInt::to_le_bytes(felt.0)
  }

  pub fn unsafe_from_le_bytes(buf: [u8; 32]) -> Felt {
    Felt(BigInt::from_le_bytes(buf))
  }

  pub fn to_be_bytes(felt: Felt) -> [u8; 32] {
    BigInt::to_be_bytes(felt.0)
  }

  pub fn unsafe_from_be_bytes(buf: [u8; 32]) -> Felt {
    Felt(BigInt::from_be_bytes(buf))
  }

  // convert to Montgomery representation
  pub fn to_mont(fld: Felt) -> Mont {
    Mont::unsafe_convert_from_big( fld.0 )
  }

  // convert from Montgomery representation
  pub fn from_mont(mont: Mont) -> Felt {
    Felt(Mont::convert_to_big(mont))
  }

  pub fn to_decimal_string(input: Felt) -> String {
    BigInt::to_decimal_string(input.0)
  }

  //------------------------------------

  pub fn zero() -> Felt {
    Felt(BigInt::zero())
  }

  pub fn from_u32(x: u32) -> Felt {
    Felt(BigInt::from_u32(x))
  }

  pub fn neg(fld: Felt) -> Felt {
    if BigInt::is_zero(fld.0) {
      Felt::zero()
    }
    else {
      Felt(BigInt::sub(FIELD_PRIME, fld.0))
    }
  }

  pub fn add(fld1: Felt, fld2: Felt) -> Felt {
    let (A, _) = BigInt::addCarry(fld1.0, fld2.0);
    let  B     = BigInt::subtract_prime_if_necessary(A);
    Felt(B) 
  }

  pub fn sub(fld1: Felt, fld2: Felt) -> Felt {
    let (big, carry) = BigInt::subBorrow(fld1.0, fld2.0);
    if carry {
      let (corrected, _) = BigInt::add_prime(big);
      Felt(corrected)
    }
    else {
      Felt(big)
    }
  }

  pub fn dbl(fld: Felt) -> Felt {
    Felt::add(fld, fld)
  }

  pub fn sqr(fld: Felt) -> Felt {
    let mont = Felt::to_mont(fld);
    Felt::from_mont(Mont::sqr(mont))
  }

  pub fn mul(fld1: Felt, fld2: Felt) -> Felt {
    let mont1 = Felt::to_mont(fld1);
    let mont2 = Felt::to_mont(fld2);
    Felt::from_mont(Mont::mul(mont1,mont2))
  }

}

//------------------------------------------------------------------------------
