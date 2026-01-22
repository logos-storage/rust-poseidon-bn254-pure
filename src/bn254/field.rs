
//
// standard representation of field elements
//
// note: this is used primarily for input/output; 
// for actual computations, use the Montgomery representation!
//

#![allow(dead_code)]
#![allow(non_snake_case)]

use std::fmt;

use crate::bn254::bigint::*;
use crate::bn254::constant::*;
use crate::bn254::montgomery::*;

//------------------------------------------------------------------------------

type Big = BigInt<8>;

#[derive(Copy, Clone)]
pub struct Felt {
  pub big: Big
}

//------------------------------------------------------------------------------

impl fmt::Display for Felt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_fmt(format_args!("{}",self.big))
  }
}

impl Felt {
  pub fn print(s: &str, A: &Felt) {
    println!("{} = {}", s, A);
  }
}

//------------------------------------------------------------------------------

impl Felt {

  pub const fn unsafe_make( xs: [u32; 8] ) -> Felt {
    Felt { big: BigInt::make(xs) }
  }

  pub fn checked_make( xs: [u32; 8] ) -> Felt {
    let big: Big = BigInt::make(xs);
    if BigInt::is_lt(&big, &FIELD_PRIME) {
      Felt { big: big }
    }
    else {
      panic!("Felt::checked_make: not in range")
    }
  }

  // convert to Montgomery representation
  pub fn to_mont(fld: &Felt) -> Mont {
    Mont::unsafe_convert_from_big( &fld.big )
  }

  // convert from Montgomery representation
  pub fn from_mont(mont: &Mont) -> Felt {
    Felt { big: Mont::convert_to_big(&mont) }
  }

  pub fn zero() -> Felt {
    Felt { big: BigInt::zero() }
  }

  pub fn from_u32(x: u32) -> Felt {
    Felt { big: BigInt::from_u32(x) }
  }

  pub fn is_equal(fld1: &Felt, fld2: &Felt) -> bool {
    BigInt::is_equal(&fld1.big, &fld2.big)
  }

  pub fn neg(fld: &Felt) -> Felt {
    if BigInt::is_zero(&fld.big) {
      Felt::zero()
    }
    else {
      Felt { big: BigInt::sub(&FIELD_PRIME, &fld.big) }
    }
  }

  pub fn add(fld1: &Felt, fld2: &Felt) -> Felt {
    let (big, carry) = BigInt::addCarry(&fld1.big, &fld2.big);
    if carry || BigInt::is_ge(&big, &FIELD_PRIME) {
      Felt { big: BigInt::sub(&big, &FIELD_PRIME) }
    }
    else {
      Felt { big: big }      
    }
  }

  pub fn sub(fld1: &Felt, fld2: &Felt) -> Felt {
    let (big, carry) = BigInt::subBorrow(&fld1.big, &fld2.big);
    if carry {
      Felt { big: BigInt::add(&big, &FIELD_PRIME) }
    }
    else {
      Felt { big: big }
    }
  }

  pub fn dbl(fld: &Felt) -> Felt {
    Felt::add(&fld, &fld)
  }

  pub fn sqr(fld: &Felt) -> Felt {
    let mont = Felt::to_mont(&fld);
    Felt::from_mont(&Mont::sqr(&mont))
  }

  pub fn mul(fld1: &Felt, fld2: &Felt) -> Felt {
    let mont1 = Felt::to_mont(&fld1);
    let mont2 = Felt::to_mont(&fld2);
    Felt::from_mont(&Mont::mul(&mont1,&mont2))
  }

}

//------------------------------------------------------------------------------
