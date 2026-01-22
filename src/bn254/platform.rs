
#![allow(dead_code)]
#![allow(non_snake_case)]

//------------------------------------------------------------------------------
// unstable version

#[inline(always)]
pub fn addCarry32_(x: u32, y: u32) -> (u32,bool) {
  u32::overflowing_add(x,y)
}

#[inline(always)]
pub fn subBorrow32_(x: u32, y: u32) -> (u32,bool) {
  u32::overflowing_sub(x,y)
}

#[inline(always)]
pub fn addCarry32(x :u32, y: u32, cin: bool) -> (u32,bool) {
  u32::carrying_add(x,y,cin)
}

#[inline(always)]
pub fn subBorrow32(x: u32, y: u32, cin: bool) -> (u32,bool) {
  u32::borrowing_sub(x,y,cin)
}

#[inline(always)]
pub fn mulTrunc32(x: u32, y: u32) -> u32 {
  u32::wrapping_mul(x,y)
}

#[inline(always)]
pub fn mulExt32(x: u32, y: u32) -> (u32,u32) {
  u32::widening_mul(x,y)
}

#[inline(always)]
pub fn mulAdd32(x: u32, y: u32, a: u32) -> (u32,u32) {
  u32::carrying_mul(x,y,a)
}

#[inline(always)]
pub fn mulAddAdd32(x: u32, y: u32, a: u32, b: u32) -> (u32,u32) {
  u32::carrying_mul_add(x,y,a,b)
}

#[inline(always)]
pub fn takeApart64(x: u64) -> (u32,u32) {
  let lo: u32 = (x & 0x_FFFF_FFFF) as u32;
  let hi: u32 = (x >> 32         ) as u32;
  (lo,hi)
}

//------------------------------------------------------------------------------
// portable version

/*

const U32_MASK: u64 = 0xFFFFFFFF;

pub fn addCarry32_(x: u32, y: u32) -> (u32,bool) {
  let z: u32  = u32::wrapping_add(x, y);
  let c: bool = z < x;
  (z, c) 
}

pub fn subBorrow32_(x: u32, y: u32) -> (u32,bool) {
  let z: u32  = u32::wrapping_sub(x, y);
  let c: bool = z > x;
  (z, c) 
}

pub fn addCarry32(x :u32, y: u32, cin: bool) -> (u32,bool) {
  let z: u32  = u32::wrapping_add(u32::wrapping_add(x, y), boolToU32(cin));
  let c: bool = if cin { z <= x } else { z < x };
  (z, c)
}

pub fn subBorrow32(x: u32, y: u32, cin: bool) -> (u32,bool) {
  let z: u32  = u32::wrapping_sub(u32::wrapping_sub(x, y), boolToU32(cin));
  let c: bool = if cin { z >= x } else { z > x };
  (z, c) 
}

pub fn extMul32(x: u32, y: u32) -> (u32,u32) {
  let z: u64 = (x as u64) * (y as u64);
  let hi = (z >> 32)      as u32;
  let lo = (z & U32_MASK) as u32;
  (lo, hi)
}

pub fn mulAdd32(x: u32, y: u32, a: u32) -> (u32,u32) {
  let z: u64 = (x as u64) * (y as u64) + (a as u64);
  let hi = (z >> 32)      as u32;
  let lo = (z & U32_MASK) as u32;
  (lo, hi)
}

pub fn mulAddAdd32(x: u32, y: u32, a: u32, b: u32) -> (u32,u32) {
  let z: u64 = (x as u64) * (y as u64) + (a as u64) + (b as u64);
  let hi = (z >> 32)      as u32;
  let lo = (z & U32_MASK) as u32;
  (lo, hi)
}

*/

//------------------------------------------------------------------------------

