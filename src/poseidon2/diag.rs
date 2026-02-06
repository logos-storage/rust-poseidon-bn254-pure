
//
// internal diffusion matrices (diagonal + constant 1 matrix)
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
fn add4(x: Mont, y: Mont, z: Mont, w: Mont) -> Mont {
  Mont::add(Mont::add(x,y),Mont::add(z,w))
}

#[inline(always)]
fn diag_T2(x: [Mont; 2]) -> [Mont; 2] {
  let s = Mont::add( x[0] , x[1] );
  [ Mont::add( s ,            x[0]   )
  , Mont::add( s , Mont::dbl( x[1] ) )
  ]
}
 
#[inline(always)]
fn diag_T3(x: [Mont; 3]) -> [Mont; 3] {
  let s = add3( x[0], x[1], x[2] );
  [ Mont::add( s ,            x[0]   )
  , Mont::add( s ,            x[1]   )
  , Mont::add( s , Mont::dbl( x[2] ) )
  ]
}

//------------------------------------------------------------------------------
// *** "old" params ***

pub mod old {

  pub mod t2 {
    use super::super::*;

    #[inline(always)]
    pub fn diag(x: [Mont; 2]) -> [Mont; 2] { diag_T2(x) }

  }

  pub mod t3 {
    use super::super::*;

    #[inline(always)]
    pub fn diag(x: [Mont; 3]) -> [Mont; 3] { diag_T3(x) }

  }

  pub mod t4 {
    use crate::bn254::montgomery::*;
    use crate::poseidon2::constants::old::t4::{DIAGONAL};
    use super::super::{add4};

    #[inline(always)]
    pub fn diag(x: [Mont; 4]) -> [Mont; 4] {
      let s = add4( x[0], x[1], x[2], x[3] );
      [ Mont::add( s , Mont::mul( x[0] , DIAGONAL[0] ) )
      , Mont::add( s , Mont::mul( x[1] , DIAGONAL[1] ) )
      , Mont::add( s , Mont::mul( x[2] , DIAGONAL[2] ) )
      , Mont::add( s , Mont::mul( x[3] , DIAGONAL[3] ) )
      ]
    }

  }

}

//------------------------------------------------------------------------------
// *** "new" params ***

pub mod new {

  pub mod t2 {
    use super::super::*;

    #[inline(always)]
    pub fn diag(x: [Mont; 2]) -> [Mont; 2] { diag_T2(x) }

  }

  pub mod t3 {
    use super::super::*;

    #[inline(always)]
    pub fn diag(x: [Mont; 3]) -> [Mont; 3] { diag_T3(x) }

  }

  pub mod t4 {
    use crate::bn254::montgomery::*;
    use crate::poseidon2::constants::new::t4::{DIAGONAL};
    use super::super::{add4};

    #[inline(always)]
    pub fn diag(x: [Mont; 4]) -> [Mont; 4] {
      let s = add4( x[0], x[1], x[2], x[3] );
      [ Mont::add( s , Mont::mul( x[0] , DIAGONAL[0] ) )
      , Mont::add( s , Mont::mul( x[1] , DIAGONAL[1] ) )
      , Mont::add( s , Mont::mul( x[2] , DIAGONAL[2] ) )
      , Mont::add( s , Mont::mul( x[3] , DIAGONAL[3] ) )
      ]
    }
  
  }

}

//------------------------------------------------------------------------------

