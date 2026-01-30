
// field properties

#![allow(dead_code)]
#![allow(non_snake_case)]

use std::cmp::{Eq};
use std::ops::{Neg,Add,Sub,Mul,Div};

//------------------------------------------------------------------------------

// pub trait Group = Copy + Clone + Default + From<u32> + Eq + Neg<Output=Self> + Add<Output=Self> + Sub<Output=Self>;
// pub trait Ring  = Group + Mul<Output=Self>;
// pub trait Field = Ring  + Div<Output=Self>;

pub trait Group : Copy + Clone + Default + From<u32> + Eq + Neg<Output=Self> + Add<Output=Self> + Sub<Output=Self> {}
pub trait Ring  : Group + Mul<Output=Self> {}
pub trait Field : Ring  + Div<Output=Self> {}

//------------------------------------------------------------------------------

fn zero<A: Group>() -> A {
  A::default()
}

fn small<A: Group>(x: u32) -> A {
  A::from(x)
}

// wtf rust?
fn xneg<A: Neg<Output=A>>(x: A ) -> A {
  -x
}

//------------------------------------------------------------------------------

pub fn prop_left_additive_unit<A: Group>(x: A) -> bool {
  zero::<A>() + x == x
}

pub fn prop_right_additive_unit<A: Group>(x: A) -> bool {
  x + zero::<A>() == x
}

pub fn prop_sub_zero<A: Group>(x: A) -> bool {
  x - zero::<A>() == x
}

pub fn prop_zero_sub<A: Group>(x: A) -> bool {
  zero::<A>() - x == xneg(x)
}

pub fn prop_add_commutative<A: Group>(x: A, y: A) -> bool {
  x + y == y + x
}

pub fn prop_sub_anticommutative<A: Group>(x: A, y: A) -> bool {
  x - y == xneg( y - x )
}

pub fn prop_neg_involutive<A: Group>(x: A) -> bool {
  xneg( xneg(x) ) == x
}

pub fn prop_add_sub<A: Group>(x: A, y: A) -> bool {
  (x + y) - y == x
}

pub fn prop_sub_add<A: Group>(x: A, y: A) -> bool {
  (x - y) + y == x
}

pub fn prop_sub_neg_add<A: Group>(x: A, y: A) -> bool {
  x - xneg(y) == x + y
}

pub fn prop_sub_add_neg<A: Group>(x: A, y: A) -> bool {
  x - y == x + xneg(y)
}

//------------------------------------------------------------------------------

pub fn prop_twice<A: Ring>(x: A) -> bool {
  x + x == x * small::<A>(2)
}

pub fn prop_thrice<A: Ring>(x: A) -> bool {
  x + x + x == x * small::<A>(3)
}

//------------------------------------------------------------------------------

pub fn prop_left_multiplicative_unit<A: Ring>(x: A) -> bool {
  small::<A>(1) * x == x
}

pub fn prop_right_multiplicative_unit<A: Ring>(x: A) -> bool {
  x * small::<A>(1)  == x
}

pub fn prop_mul_commutative<A: Ring>(x: A, y: A) -> bool {
  x * y == y * x
}

pub fn prop_mul_neg<A: Ring>(x: A, y: A) -> bool {
  xneg(x * y) == xneg(x) * y 
}

pub fn prop_distributive_add<A: Ring>(x: A, y: A, z: A) -> bool {
  (x + y) * z == x * z + y * z
}

pub fn prop_distributive_sub<A: Ring>(x: A, y: A, z: A) -> bool {
  (x - y) * z == x * z - y * z
}

//------------------------------------------------------------------------------
