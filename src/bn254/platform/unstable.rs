
// "unstable" (in rust parlance) version

#![allow(dead_code)]
#![allow(non_snake_case)]

//------------------------------------------------------------------------------

#[inline(always)]
pub fn boolToU32(c: bool) -> u32 {
  if c { 1 } else { 0 }
}

#[inline(always)]
pub fn boolToMSB32(c: bool) -> u32 {
  if c { 0x_8000_0000 } else { 0 }
}

//------------------------------------------------------------------------------

#[inline(always)]
pub fn addCarry32_(x: u32, y: u32) -> (u32,bool) {
  u32::overflowing_add(x,y)
}

#[inline(always)]
pub fn subBorrow32_(x: u32, y: u32) -> (u32,bool) {
  u32::overflowing_sub(x,y)
}

#[inline(always)]
pub fn addCarry32(x: u32, y: u32, cin: bool) -> (u32,bool) {
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
pub fn u64AddAdd32(xy: (u32,u32), a: u32, b: u32) -> (u32,u32) {
  let lo0 = xy.0;
  let hi0 = xy.1;
  
  // add `a`
  let (lo1,c) = u32::overflowing_add(lo0,a  );
  let (hi1,_) = u32::   carrying_add(hi0,0,c);
  
  // add `b`
  let (lo2,d) = u32::overflowing_add(lo1,b  );
  let (hi2,_) = u32::   carrying_add(hi1,0,d);

  (lo2,hi2)
}

//------------------------------------------------------------------------------

// rust, please just go home, you are drunk...
#[inline(always)]
pub fn rotRight32By1(cin: bool, x: u32) -> (u32, bool) {
  unsafe {
    let cout : bool = (x & 1) != 0;
    let y    : u32  = u32::unchecked_shr(x,1) | u32::unchecked_shl(boolToU32(cin),31);
    (y, cout)
  }
} 

#[inline(always)]
pub fn rotLeft32By1(x: u32, cin: bool) -> (bool, u32) {
  unsafe {
    let cout : bool = (x & 0x_8000_0000) != 0;
    let y    : u32  = u32::unchecked_shl(x,1) | boolToU32(cin);
    (cout, y)
  }
} 

//------------------------------------------------------------------------------
