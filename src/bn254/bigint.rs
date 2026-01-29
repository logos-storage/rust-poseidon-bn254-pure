

//
// big integers, represented as little-endian arrays of u32-s
//

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use std::fmt;
use std::cmp::{Ordering,min};
use std::ops::{Add,Sub,RangeFull};

use std::random::{RandomSource,Distribution};

use unroll::unroll_for_loops;

use crate::bn254::platform::*;
use crate::bn254::constant::{PRIME_ARRAY};

//------------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BigInt<const N: usize>([u32; N]);

pub type BigInt256 = BigInt<8>;
pub type BigInt512 = BigInt<16>;

//------------------------------------------------------------------------------
// display traits

impl<const N: usize> fmt::Display for BigInt<N> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "0x")?;
    for i in (0..N).rev() {
      write!(f, "{:08x}", self.0[i])?;
    }
    Ok(())
  }
}

impl<const N: usize> BigInt<N> {
  pub fn debug_print(s: &str, A: BigInt<N>) {
    println!("{} = {}", s, A);
  }
}

//------------------------------------------------------------------------------
// standard numeric traits

impl<const N: usize> Add for BigInt<N> {
  type Output = Self;
  fn add(self, other: Self) -> Self { BigInt::add(self,other) }
}

impl<const N: usize> Sub for BigInt<N> {
  type Output = Self;
  fn sub(self, other: Self) -> Self { BigInt::sub(self,other) }
}

impl<const N: usize> PartialOrd for BigInt<N> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(BigInt::cmp(*self, *other)) }
}

impl<const N: usize> Ord for BigInt<N> {
  fn cmp(&self, other: &Self) -> Ordering { BigInt::cmp(*self, *other) }
}

//------------------------------------------------------------------------------
// conversion traits

impl<const N: usize> From<[u32; N]> for BigInt<N> {
  fn from(limbs: [u32; N]) -> Self { BigInt(limbs) }
}

impl<const N: usize> Into<[u32; N]> for BigInt<N> {
  fn into(self: Self) -> [u32; N] { self.0 }
}

//------------------------------------------------------------------------------
// small values

impl<const N: usize> Default for BigInt<N> {
  fn default() -> Self { Self([0; N]) }
}

impl<const N: usize> From<u32> for BigInt<N> {
  fn from(x: u32) -> Self { Self::from_u32(x) }
}

//------------------------------------------------------------------------------
// random trait

impl<const N: usize> Distribution<BigInt<N>> for RangeFull {
  fn sample(&self, source: &mut (impl RandomSource + ?Sized)) -> BigInt<N> {
    BigInt::sample(source)
  }
}

//------------------------------------------------------------------------------
// internal implementations

impl<const N: usize> BigInt<N> {

  #[inline(always)]
  pub const fn to_limbs(big: BigInt<N>) -> [u32; N] { big.0 }

  #[inline(always)]
  pub const fn from_limbs(limbs: [u32; N]) -> BigInt<N> { BigInt(limbs) }
 
  #[inline(always)]
  pub const fn make(ls: [u32; N]) -> BigInt<N> { BigInt(ls) }

  //------------------------------------
  // conversion to/from bytes

  pub fn to_le_bytes(big: BigInt<N>) -> [u8; 4*N] {
    let mut buf : [u8; 4*N] = [0; 4*N];
    for i in 0..N {
      let k = 4*i;
      buf[k..k+4].copy_from_slice(&big.0[i].to_le_bytes());
    }
    buf
  }

  pub fn from_le_bytes(buf : [u8; 4*N]) -> BigInt<N> {
    let mut ws: [u32; N] = [0; N];
    for i in 0..N {
      let k = 4*i;
      let mut xs: [u8; 4] = [0; 4];
      for j in 0..4 { xs[j] = buf[k+j]; }       // stupid rust...
      let w: u32 = u32::from_le_bytes(xs);
      ws[i] = w;
    }
    BigInt(ws)
  }

  pub fn to_be_bytes(big: BigInt<N>) -> [u8; 4*N] {
    let mut buf : [u8; 4*N] = [0; 4*N];
    for i in 0..N {
      let k = 4*i;
      buf[k..k+4].copy_from_slice(&big.0[N-1-i].to_be_bytes());
    }
    buf
  }

  pub fn from_be_bytes(buf: [u8; 4*N]) -> BigInt<N> {
    let mut ws: [u32; N] = [0; N];
    for i in 0..N {
      let k = 4*i;
      let mut xs: [u8; 4] = [0; 4];
      for j in 0..4 { xs[j] = buf[k+j]; }   
      let w: u32 = u32::from_be_bytes(xs);
      ws[N-1-i] = w;
    }
    BigInt(ws)
  }

  //------------------------------------
  // decimal printing

  pub fn divmod_small(big: BigInt<N>, modulus: u32) -> (BigInt<N> , u32) {
    let u64_modulus: u64 = modulus as u64;
    let mut carry: u32 = 0;
    let mut qs: [u32; N] = [0; N];
    for i in 0..N {
      let x: u64 = ((carry as u64) << 32) + (big.0[N-1-i] as u64);
      qs[N-1-i] = (x / u64_modulus) as u32;
      carry     = (x % u64_modulus) as u32;
    }
    (BigInt(qs), carry)
  }

  pub fn to_decimal_string(input: BigInt<N>) -> String {
    let mut digits: Vec<u8> = Vec::new();
    let mut big: BigInt<N> = input.clone();
    while( !BigInt::is_zero(big) ) {
      let (q,r) = BigInt::divmod_small(big, 10);
      digits.push( 48 + (r as u8) );
      big = q;
    }
    if digits.len() == 0 {
      digits.push( 48 ); 
    }
    digits.reverse();
    str::from_utf8(&digits).unwrap().to_string()
  }

  pub fn to_hex_string(input: BigInt<N>) -> String {
    format!("{}", input)
  }

  //------------------------------------

  pub fn truncate1(big: BigInt<{N+1}>) -> BigInt<N> {
    // let small: [u32; N] = &big.limbs[0..N];
    let mut small: [u32; N] = [0; N];
    for i in 0..N { small[i] = big.0[i]; }
    BigInt(small)
  }

  pub fn zero() -> BigInt<N> {
    BigInt([0; N])
  }

  pub fn from_u32(x: u32) -> BigInt<N> {
    let mut xs = [0; N];
    xs[0] = x;
    BigInt(xs)
  }

  //------------------------------------
  // comparison

  pub fn is_zero(big: BigInt<N>) -> bool {
    big.0.iter().all(|&x| x == 0)
  }

  pub fn cmp(big1: BigInt<N>, big2: BigInt<N>) -> Ordering {
    let mut res : Ordering = Ordering::Equal;
    for i in (0..N).rev() {
      if big1.0[i] < big2.0[i] {
        res = Ordering::Less;
        break;
      }
      if big1.0[i] > big2.0[i] {
        res = Ordering::Greater;
        break;
      }
    }
    res
  }

  //------------------------------------
  // addition and subtraction

  #[inline(always)]
  #[unroll_for_loops]
  pub fn addCarry(big1: BigInt<N>, big2: BigInt<N>) -> (BigInt<N>, bool) {
    let mut c  : bool = false;  
    let mut zs : [u32; N] = [0; N];
    for i in 0..N {
      let (z,cout) = addCarry32( big1.0[i] , big2.0[i] , c);
      zs[i] = z;
      c = cout;
    }
    let big: BigInt<N> = BigInt(zs);
    (big, c)
  }

  #[inline(always)]
  #[unroll_for_loops]
  pub fn subBorrow(big1: BigInt<N>, big2: BigInt<N>) -> (BigInt<N>, bool) {
    let mut c  : bool = false;  
    let mut zs : [u32; N] = [0; N];
    for i in 0..N {
      let (z,cout) = subBorrow32( big1.0[i] , big2.0[i] , c );
      zs[i] = z;
      c = cout;
    }
    let big: BigInt<N> = BigInt(zs); 
    (big, c)
  }

  pub fn add(big1: BigInt<N>, big2: BigInt<N>) -> BigInt<N> {
    let (out,_) = BigInt::addCarry(big1,big2);
    out
  }

  pub fn sub(big1: BigInt<N>, big2: BigInt<N>) -> BigInt<N> {
    let (out,_) = BigInt::subBorrow(big1,big2);
    out
  }

  //------------------------------------
  // multiplication

  pub fn scale(scalar: u32, big2: BigInt<N>) -> (BigInt<N>, u32) {
    let mut c  : u32 = 0;
    let mut zs : [u32; N] = [0; N];
    for i in 0..N {
      let (lo,hi) = mulAdd32(scalar, big2.0[i], c);
      zs[i] = lo;
      c = hi;
    }
    let big: BigInt<N> = BigInt(zs); 
    (big, c)
  }

  #[inline(always)]
  #[unroll_for_loops]
  pub fn scaleAdd(scalar: u32, vector: BigInt<N>, add: BigInt<N>) -> (BigInt<N>, u32) {
    let mut c  : u32 = 0;
    let mut zs : [u32; N] = [0; N];
    for i in 0..N {
      let (lo,hi) = mulAddAdd32(scalar, vector.0[i], c, add.0[i]);
      zs[i] = lo;
      c = hi;
    }
    let big: BigInt<N> = BigInt(zs); 
    (big, c)
  }

  #[inline(always)]
  #[unroll_for_loops]
  pub fn multiply<const M: usize>(big1: BigInt<N>, big2: BigInt<M>) -> BigInt<{N+M}> {
    let mut product : [u32; N+M] = [0; N+M];
    let mut state   : [u32; N]   = [0; N];
    for j in 0..M {
      let (scaled,carry) = BigInt::scaleAdd( big2.0[j], big1, BigInt(state) );
      product[j] = scaled.0[0];
      for i in 1..N { state[i-1] = scaled.0[i] }
      state[N-1] = carry;
    }
    for i in 0..N { 
      product[i+M] = state[i]
    }
  
    BigInt(product)
  }

  #[inline(always)]
  pub fn mul(big1: BigInt<N>, big2: BigInt<N>) -> BigInt<{N+N}> {
    BigInt::multiply(big1,big2)
  }

  // x*y + z
  #[inline(always)]
  pub fn mulAdd(big1: BigInt<N>, big2: BigInt<N>, big3: BigInt<N>) -> BigInt<{N+N}> {
    // first compute the product
    let mut product : [u32; N+N] = [0; N+N];
    let mut state   : [u32; N]   = [0; N];
    for j in 0..N {
      let (scaled,carry) = BigInt::scaleAdd( big2.0[j], big1, BigInt(state) );
      product[j] = scaled.0[0];
      for i in 1..N { state[i-1] = scaled.0[i] }
      state[N-1] = carry;
    }
    for i in 0..N { 
      product[i+N] = state[i]
    }
  
    // then add the third number
    let mut carry: bool = false;  
    for i in 0..N {
      let (z,c)  = addCarry32( product[i] , big3.0[i] , carry );
      carry      = c; 
      product[i] = z;
    }
    // continue carrying
    for i in N..(N+N) {
      let (z,c)  = addCarry32( product[i] , 0 , carry );
      carry      = c; 
      product[i] = z;
    }

    BigInt(product)
  }

  // x*y + (z << 256)
  #[inline(always)]
  pub fn mulAddShifted(big1: BigInt<N>, big2: BigInt<N>, big3: BigInt<N>) -> BigInt<{N+N}> {
    // first compute the product
    let mut product : [u32; N+N] = [0; N+N];
    let mut state   : [u32; N]   = [0; N];
    for j in 0..N {
      let (scaled,carry) = BigInt::scaleAdd( big2.0[j], big1, BigInt(state) );
      product[j] = scaled.0[0];
      for i in 1..N { state[i-1] = scaled.0[i] }
      state[N-1] = carry;
    }
    for i in 0..N { 
      product[i+N] = state[i]
    }
  
    // then add the third number, shifted
    let mut carry: bool = false;  
    for i in 0..N {
      let (z,c)    = addCarry32( product[i+N] , big3.0[i] , carry );
      carry        = c; 
      product[i+N] = z;
    }

    BigInt(product)
  }

  // TODO: optimize this?!
  pub fn sqr_naive(big: BigInt<N>) -> BigInt<{N+N}> {
    BigInt::multiply(big,big)
  }

  #[inline(always)]
  pub fn sqr(big: BigInt<N>) -> BigInt<{N+N}> {
    BigInt::multiply(big,big)
  }

  //------------------------------------
  // random

  pub fn sample(source: &mut (impl RandomSource + ?Sized)) -> BigInt<N> {
    let mut xs: [u32; N] = [0; N];
    for i in 0..N {
      xs[i] = RangeFull.sample(source);
    }
    BigInt::make(xs)
  }

}

// -----------------------------------------------------------------------------
// routines specialized to the prime number

impl BigInt256 {

  #[inline(always)]
  #[unroll_for_loops]
  pub fn is_lt_prime(big: BigInt256) -> bool {
    let mut less: bool = false;
    for i in (0..8).rev() {
      if big.0[i] < PRIME_ARRAY[i] {
        less = true;
        break;
      }
      if big.0[i] > PRIME_ARRAY[i] {
        break;
      }
    }
    less
  }

  #[inline(always)]
  pub fn is_ge_prime(big: BigInt256) -> bool {
    !BigInt256::is_lt_prime(big)
  }

  #[inline(always)]
  #[unroll_for_loops]
  pub fn add_prime(big: BigInt256) -> (BigInt256, bool) {
    let mut c  : bool = false;  
    let mut zs : [u32; 8] = [0; 8];
    for i in 0..8 {
      let (z,cout) = addCarry32( big.0[i] , PRIME_ARRAY[i] , c );
      zs[i] = z;
      c = cout;
    }
    let big: BigInt256 = BigInt(zs);
    (big, c)
  }

  #[inline(always)]
  #[unroll_for_loops]
  pub fn subtract_prime(big: BigInt256) -> (BigInt256, bool) {
    let mut c  : bool = false;  
    let mut zs : [u32; 8] = [0; 8];
    for i in 0..8 {
      let (z,cout) = subBorrow32( big.0[i] , PRIME_ARRAY[i] , c );
      zs[i] = z;
      c = cout;
    }
    let big: BigInt256 = BigInt(zs); 
    (big, c)
  }

  #[inline(always)]
  pub fn subtract_prime_if_necessary(big: BigInt256) -> BigInt256 {
    if BigInt256::is_lt_prime(big) {
      big
    }
    else {
      let (corrected, _) = BigInt256::subtract_prime(big);
      corrected
    }
  }

  //------------------------------------
  // ramndom

  fn sample_masked(source: &mut (impl RandomSource + ?Sized)) -> BigInt256 {
    let mut xs: [u32; 8] = [0; 8];
    for i in 0..8 {
      xs[i] = RangeFull.sample(source);
    }
    xs[7] = xs[7] & 0x_3FFF_FFFF;
    BigInt::make(xs)
  }

  // rejection sampling
  pub fn sample_mod_prime(source: &mut (impl RandomSource + ?Sized)) -> BigInt256 {
    let mut x: BigInt256 = BigInt256::sample_masked(source);
    while( !BigInt256::is_lt_prime(x) ) {
      x = BigInt256::sample_masked(source);
    }
    x
  }

}  


