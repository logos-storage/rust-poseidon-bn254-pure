
//
// Montgomery representation of field elements
//

#![allow(dead_code)]
#![allow(non_snake_case)]

use std::fmt;

use crate::bigint::*;
use crate::constant::*;

//------------------------------------------------------------------------------

type Big = BigInt<8>;

#[derive(Clone)]
pub struct Mont {
  pub big: Big
}

pub const MONT_R1 : Mont = Mont { big: BIG_R1 };
pub const MONT_R2 : Mont = Mont { big: BIG_R2 };
pub const MONT_R3 : Mont = Mont { big: BIG_R3 };

//------------------------------------------------------------------------------

impl fmt::Display for Mont {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _   = f.write_str("[");
    let res = f.write_fmt(format_args!("{}",self.big));
    let _   = f.write_str("]");
    res
  }
}

//------------------------------------------------------------------------------

impl Mont {

  pub const fn unsafe_make( xs: [u32; 8] ) -> Mont {
    Mont { big: BigInt::make(xs) }
  }

  pub fn zero() -> Mont {
    Mont { big: BigInt::zero() }
  }

  pub fn is_equal(mont1: &Mont, mont2: &Mont) -> bool {
    BigInt::is_equal(&mont1.big, &mont2.big)
  }

  pub fn neg(mont: &Mont) -> Mont {
    if BigInt::is_zero(&mont.big) {
      Mont::zero()
    }
    else {
      Mont { big: BigInt::sub(&FIELD_PRIME, &mont.big) }
    }
  }

  pub fn add(mont1: &Mont, mont2: &Mont) -> Mont {
    let (big, carry) = BigInt::addCarry(&mont1.big, &mont2.big);
    if carry || BigInt::is_ge(&big, &FIELD_PRIME) {
      Mont { big: BigInt::sub(&big, &FIELD_PRIME) }
    }
    else {
      Mont { big: big }      
    }
  }

  pub fn sub(mont1: &Mont, mont2: &Mont) -> Mont {
    let (big, carry) = BigInt::subBorrow(&mont1.big, &mont2.big);
    if carry {
      Mont { big: BigInt::add(&big, &FIELD_PRIME) }
    }
    else {
      Mont { big: big }
    }
  }

  pub fn dbl(mont: &Mont) -> Mont {
    Mont::add(&mont, &mont)
  }

  // the Montgomery reduction algorithm
  fn redc(input: BigInt<16>) -> Big {
    let mut T: [u32; 17] = [0; 17];
    for i in 0..16 { T[i] = input.limbs[i] }

    BigInt::zero()
  }

  pub fn sqr(mont: &Mont) -> Mont {
    let large = BigInt::sqr(&mont.big);
    Mont { big: Mont::redc(large) }    
  }

  pub fn mul(mont1: &Mont, mont2: &Mont) -> Mont {
    let large = BigInt::mul(&mont1.big, &mont2.big);
    Mont { big: Mont::redc(large) }    
  }

  // this does conversion from the standard representation
  // we assume the input is in the range `[0..p-1]`
  pub fn unsafe_convert_from_big(input: &Big) -> Mont {
    let mont: Mont = Mont { big: input.clone() };
    Mont::mul( &mont , &MONT_R2 )
  }

  // this does conversion to the standard representation
  pub fn convert_to_big(mont: &Mont) -> Big {
    let mut tmp: [u32; 16] = [0; 16];
    for i in 0..8 { tmp[i] = mont.big.limbs[i] } 
    Mont::redc( BigInt { limbs: tmp } )
  }

}

//------------------------------------------------------------------------------

impl Mont {
  pub fn print_internal(s: &str, A: &Mont) {
    println!("{} = [{}]", s, A.big);
  }

  pub fn print_standard(s: &str, A: &Mont) {
    println!("{} = {}", s, Mont::convert_to_big(A) ) ;
  }
}

//------------------------------------------------------------------------------
