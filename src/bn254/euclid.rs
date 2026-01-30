
//
// extended binary euclidean algorithm
// (used for modular inversion)
//

#![allow(unused)]
#![allow(non_snake_case)]

use std::ops::{Neg,Add,Sub,Mul};

use crate::bn254::bigint::*;

//------------------------------------------------------------------------------

// NOTE: the prime is hard-wired into this
pub fn euclid
  ( x0: BigInt256
  , y0: BigInt256
  , u0: BigInt256
  , v0: BigInt256
  ) -> BigInt256 {

  let mut x = x0;     // x and y are modulo P
  let mut y = y0;      
  let mut u = u0;     // u and v are just integers     
  let mut v = v0;      

  while( !BigInt::is_one(u) && !BigInt::is_one(v) ) {

    // shift right u and (x mod p) while u is even
    while( BigInt::is_even(u) ) {
      let (u1, _) = BigInt::shiftRightBy1(u);
      u = u1;
      x = BigInt::div_by_2_mod_prime(x);
    }
  
    // shift right v and (y mod p) while v is even
    while( BigInt::is_even(v) ) {
      let (v1, _) = BigInt::shiftRightBy1(v);
      v = v1;
      y = BigInt::div_by_2_mod_prime(y);
    }

    if u < v {
      v = v - u;
      y = BigInt::sub_mod_prime( y , x );
    }
    else {
      u = u - v;
      x = BigInt::sub_mod_prime( x , y );
    }

  }

  if BigInt::is_one(u) { x } else { y }

}

//------------------------------------------------------------------------------
