
//
// MDS matrices (used in the external rounds and the standalaone linear layer)
//

#![allow(dead_code)]
#![allow(non_snake_case)]

//------------------------------------------------------------------------------

use crate::bn254::montgomery::*;

#[inline(always)]
fn add3(x: Mont, y: Mont, z: Mont) -> Mont {
  Mont::add(Mont::add(x,y),z)
}

#[inline(always)]
fn times4(x: Mont) -> Mont {
  Mont::dbl(Mont::dbl(x))
}

//------------------------------------------------------------------------------

pub mod t2 {
  use crate::bn254::montgomery::*;

  #[inline(always)]
  pub fn mds(x: [Mont; 2]) -> [Mont; 2] {
    let s = Mont::add( x[0], x[1] );
    [ Mont::add( s , x[0] )
    , Mont::add( s , x[1] )
    ]
  }

}

pub mod t3 {
  use super::*;

  #[inline(always)]
  pub fn mds(x: [Mont; 3]) -> [Mont; 3] {
    let s = add3( x[0], x[1], x[2] );
    [ Mont::add( s , x[0] )
    , Mont::add( s , x[1] )
    , Mont::add( s , x[2] )
    ]
  }

}

pub mod t4 {
  use super::*;

  pub fn mds(x: [Mont; 4]) -> [Mont; 4] {
    let t0 =            x[0]   + x[1] ;
    let t1 =            x[2]   + x[3] ;
    let t2 = Mont::dbl( x[1] ) +  t1  ;
    let t3 = Mont::dbl( x[3] ) +  t0  ;
    let t4 =    times4(  t1  ) +  t3  ;
    let t5 =    times4(  t0  ) +  t2  ;
    let t6 =             t3    +  t5  ;
    let t7 =             t2    +  t4  ;
    [ t6 , t5 , t7, t4]
  }

}

//------------------------------------------------------------------------------

