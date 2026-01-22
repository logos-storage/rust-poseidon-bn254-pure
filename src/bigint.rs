
//
// big integers, represented as little-endian arrays of u32-s
//

#![allow(dead_code)]
#![allow(non_snake_case)]

use std::fmt;
use std::cmp::Ordering;

use crate::platform::*;

//------------------------------------------------------------------------------

#[derive(Clone)]
pub struct BigInt<const N: usize> {
  pub limbs: [u32; N]
}

pub fn mkBigInt<const N: usize>(ls: [u32; N]) -> BigInt<N> {
  BigInt { limbs: ls }
}

pub type BigInt256 = BigInt<8>;
pub type BigInt512 = BigInt<16>;

//------------------------------------------------------------------------------

fn boolToU32(c: bool) -> u32 {
  if c { 1 } else { 0 }
}

//------------------------------------------------------------------------------

impl<const N: usize> fmt::Display for BigInt<N> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = f.write_str("0x");
    for i in 0..N {
      let _ = f.write_fmt(format_args!("{:08x}",self.limbs[N-1-i]));
    }   
    Ok(())
  }
}

impl<const N: usize> BigInt<N> {
  pub fn print(s: &str, A: &BigInt<N>) {
    println!("{} = {}", s, A);
  }
}

//------------------------------------------------------------------------------

impl<const N: usize> BigInt<N> {

  pub const fn make(ls: [u32; N]) -> BigInt<N> { 
    BigInt { limbs: ls }
  }

  pub fn zero() -> BigInt<N> {
    BigInt { limbs: [0; N] }
  }

  pub fn from_u32(x: u32) -> BigInt<N> {
    let mut xs = [0; N];
    xs[0] = x;
    BigInt { limbs: xs }
  }

  pub fn is_zero(big: &BigInt<N>) -> bool {
    let mut ok : bool = true;
    for i in 0..N {
      if big.limbs[i] != 0 {
        ok = false;
        break;
      }
    }
    ok
  }

  pub fn is_equal(big1: &BigInt<N>, big2: &BigInt<N>) -> bool {
    let mut ok : bool = true;
    for i in 0..N {
      if big1.limbs[i] != big2.limbs[i] {
        ok = false;
        break;
      }
    }
    ok
  }

  pub fn cmp(big1: &BigInt<N>, big2: &BigInt<N>) -> Ordering {
    let mut res : Ordering = Ordering::Equal;
    for i in (0..N).rev() {
      if big1.limbs[i] < big2.limbs[i] {
        res = Ordering::Less;
        break;
      }
      if big1.limbs[i] > big2.limbs[i] {
        res = Ordering::Greater;
        break;
      }
    }
    res
  }

  pub fn is_lt(big1: &BigInt<N>, big2: &BigInt<N>) -> bool {
    BigInt::cmp(&big1, &big2) == Ordering::Less
  }

  pub fn is_gt(big1: &BigInt<N>, big2: &BigInt<N>) -> bool {
    BigInt::cmp(&big1, &big2) == Ordering::Greater
  }

  pub fn is_le(big1: &BigInt<N>, big2: &BigInt<N>) -> bool {
    !BigInt::is_gt(&big1, &big2)
  }

  pub fn is_ge(big1: &BigInt<N>, big2: &BigInt<N>) -> bool {
    !BigInt::is_lt(&big1, &big2)
  }

  pub fn addCarry(big1: &BigInt<N>, big2: &BigInt<N>) -> (BigInt<N>, bool) {
    let mut c  : bool = false;  
    let mut zs : [u32; N] = [0; N];
    for i in 0..N {
      let (z,cout) = addCarry32( big1.limbs[i] , big2.limbs[i] , c);
      zs[i] = z;
      c = cout;
    }
    let big: BigInt<N> = BigInt { limbs: zs }; 
    (big, c)
  }

  pub fn subBorrow(big1: &BigInt<N>, big2: &BigInt<N>) -> (BigInt<N>, bool) {
    let mut c  : bool = false;  
    let mut zs : [u32; N] = [0; N];
    for i in 0..N {
      let (z,cout) = subBorrow32( big1.limbs[i] , big2.limbs[i] , c);
      zs[i] = z;
      c = cout;
    }
    let big: BigInt<N> = BigInt { limbs: zs }; 
    (big, c)
  }

  pub fn add(big1: &BigInt<N>, big2: &BigInt<N>) -> BigInt<N> {
    let (out,_) = BigInt::addCarry(big1,big2);
    out
  }

  pub fn sub(big1: &BigInt<N>, big2: &BigInt<N>) -> BigInt<N> {
    let (out,_) = BigInt::subBorrow(big1,big2);
    out
  }

  pub fn scale(scalar: u32, big2: &BigInt<N>) -> (BigInt<N>, u32) {
    let mut c  : u32 = 0;
    let mut zs : [u32; N] = [0; N];
    for i in 0..N {
      let (lo,hi) = mulAdd32(scalar, big2.limbs[i], c);
      zs[i] = lo;
      c = hi;
    }
    let big: BigInt<N> = BigInt { limbs: zs }; 
    (big, c)
  }

  pub fn scaleAdd(scalar: u32, big2: &BigInt<N>, add: &BigInt<N>) -> (BigInt<N>, u32) {
    let mut c  : u32 = 0;
    let mut zs : [u32; N] = [0; N];
    for i in 0..N {
      let (lo,hi) = mulAddAdd32(scalar, big2.limbs[i], c, add.limbs[i]);
      zs[i] = lo;
      c = hi;
    }
    let big: BigInt<N> = BigInt { limbs: zs }; 
    (big, c)
  }

  pub fn multiply<const M: usize>(big1: &BigInt<N>, big2: &BigInt<M>) -> BigInt<{N+M}> {
    let mut product : [u32; N+M] = [0; N+M];
    let mut state   : [u32; N]   = [0; N];
    for j in 0..M {
      let (scaled,carry) = BigInt::scaleAdd( big2.limbs[j], &big1, &(BigInt { limbs: state }) );
      product[j] = scaled.limbs[0];
      for i in 1..N { state[i-1] = scaled.limbs[i] }
      state[N-1] = carry;
    }
    for i in 0..N { 
      product[i+M] = state[i]
    }
  
    BigInt { limbs: product }
  }

  pub fn mul(big1: &BigInt<N>, big2: &BigInt<N>) -> BigInt<{N+N}> {
    BigInt::multiply(big1,big2)
  }

  // TODO: optimize this!
  pub fn sqr(big: &BigInt<N>) -> BigInt<{N+N}> {
    BigInt::multiply(big,big)
  }

}

//------------------------------------------------------------------------------
