
use crate::bigint::*;

//------------------------------------------------------------------------------
// field prime (BN254 scalar field)

pub const FIELD_PRIME : BigInt<8> = BigInt { 
  limbs: [ 0xf0000001 , 0x43e1f593 , 0x79b97091 , 0x2833e848 
         , 0x8181585d , 0xb85045b6 , 0xe131a029 , 0x30644e72 ] };
 
pub const PRIME_PLUS_1 : BigInt<8> = BigInt { 
  limbs: [ 0xf0000002 , 0x43e1f593 , 0x79b97091 , 0x2833e848 
         , 0x8181585d , 0xb85045b6 , 0xe131a029 , 0x30644e72 ] };

pub const HALF_PRIME_PLUS_1 : BigInt<8> = BigInt { 
  limbs: [ 0xf8000001 , 0xa1f0fac9 , 0x3cdcb848 , 0x9419f424 
         , 0x40c0ac2e , 0xdc2822db , 0x97098d01 , 0x01832273 ] }; 

//------------------------------------------------------------------------------
// montgomery constants

pub const MONT_R1 : BigInt<8> = BigInt {
  limbs: [ 0x4ffffffb , 0xac96341c , 0x9f60cd29 , 0x36fc7695 
         , 0x7879462e , 0x666ea36f , 0x9a07df2f , 0x0e0a77c1 ] };

pub const MONT_R2 : BigInt<8> = BigInt {
  limbs: [ 0xae216da7 , 0x1bb8e645 , 0xe35c59e3 , 0x53fe3ab1 
         , 0x53bb8085 , 0x8c49833d , 0x7f4e44a5 , 0x0216d0b1 ] };

pub const MONT_R3 : BigInt<8> = BigInt {
  limbs: [ 0xb4bf0040 , 0x5e94d8e1 , 0x1cfbb6b8 , 0x2a489cbe 
         , 0xa19fcfed , 0x893cc664 , 0x7fcc657c , 0x0cf8594b ] };

//------------------------------------------------------------------------------
