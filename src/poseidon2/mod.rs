
pub mod constants;
pub mod permutation;
pub mod mds;
pub mod diag;

pub use permutation::{Params,Poseidon2Params};

//------------------------------------------------------------------------------

// helpers
pub mod aux {
  
  use crate::bn254::field::{Felt};

  pub fn kat_input<const T: usize>() -> [Felt; T] {
    let mut xs: [Felt; T] = [Default::default(); T];
    for i in 0..T {
      xs[i] = Felt::from_u32(i as u32);
    }
    xs
  }

  pub fn print_state<const T: usize>( prefix: &str, xs: [Felt; T] ) {
    for i in 0..T {
      println!(" - {}[{}] -> {}" , prefix, i , xs[i] );
    }
  }

}

//------------------------------------------------------------------------------
// "old" set of constants

pub mod old {
  use crate::bn254::field::{Felt};
  pub use crate::poseidon2::permutation::{Params,Poseidon2Params};

  pub fn permute<const T: usize>(input: [Felt; T]) -> [Felt; T] where Params: Poseidon2Params<false,T> {
    crate::poseidon2::permutation::permute::<false,T>( input )  
  }

  pub fn compress<const K: usize>(input: [Felt; K]) -> Felt where Params: Poseidon2Params<false,{K+1}> {
    crate::poseidon2::permutation::compress::<false,K>( input )
  }

  pub fn hash1(a: Felt) -> Felt {
    compress::<1>([ a ])
  }
  
  pub fn hash2(a: Felt, b: Felt) -> Felt {
    compress::<2>([ a, b ])
  }
  
  pub fn hash3(a: Felt, b: Felt, c: Felt) -> Felt {
    compress::<3>([ a, b, c ])
  }

  //--------------------------------------------------------
  // tests for the "old" permutations

  #[cfg(test)]
  mod test {

    use crate::bn254::field::{Felt};
    use super::super::aux::*;
    use super::*;

    #[test]
    fn old_permute2_kat() {
      let out: [Felt; 2] = permute::<2>( kat_input::<2>() );
      print_state::<2>( "output" , out );
      assert_eq!( Felt::to_hex_string( out[0] ) , "0x1713924640bec577e44f2c0c4b3c73339c135cf4678b6ede5bce727791e9c1ef" );
      assert_eq!( Felt::to_hex_string( out[1] ) , "0x1ddcaa93e296bbc6cb89bf4052134cba5c1e35c4367e0d4d9344b576cf532c04" );
    }

    #[test]
    fn old_permute3_kat() {
      let out: [Felt; 3] = permute::<3>( kat_input::<3>() );
      print_state::<3>( "output" , out );
      assert_eq!( Felt::to_hex_string( out[0] ) , "0x30610a447b7dec194697fb50786aa7421494bd64c221ba4d3b1af25fb07bd103" );
      assert_eq!( Felt::to_hex_string( out[1] ) , "0x13f731d6ffbad391be22d2ac364151849e19fa38eced4e761bcd21dbdc600288" );
      assert_eq!( Felt::to_hex_string( out[2] ) , "0x1433e2c8f68382c447c5c14b8b3df7cbfd9273dd655fe52f1357c27150da786f" );
    }

    #[test]
    fn old_permute4_kat() {
      let out: [Felt; 4] = permute::<4>( kat_input::<4>() );
      print_state::<4>( "output" , out );
      assert_eq!( Felt::to_hex_string( out[0] ) , "0x2dae4aa60bf00f42e7e409b7d79112ef8c82dc98c8703a18f349cc1acb0c0a01" );
      assert_eq!( Felt::to_hex_string( out[1] ) , "0x024badd303fa99d176db98904313d803889ca6520ff89ccd821d82e128982cf0" );
      assert_eq!( Felt::to_hex_string( out[2] ) , "0x2621eb27814db8e95fbb3d33bc7050e9a15b68bc56897a99440343bd53415fe4" );
      assert_eq!( Felt::to_hex_string( out[3] ) , "0x1e894ea2894a467e113d91475ce583b82c6b421633989cfb2fb98d2008c7283c" );
    }

    #[test]
    fn old_hash1() {
      let hash: Felt = hash1( Felt::from_u32(111) );
      println!("hash = {}", hash);
      assert_eq!( Felt::to_hex_string( hash ) , "0x02bd8af9e7ea86b0861a82b55c865efc07b450e829a728c1afbceda702a4e4d1" );
    }

    #[test]
    fn old_hash2() {
      let hash: Felt = hash2( Felt::from_u32(111) , Felt::from_u32(222) );
      println!("hash = {}", hash);
      assert_eq!( Felt::to_hex_string( hash ) , "0x23063d9199d3aa163ccc0dc8342dddc6abb95c549ceb2593bb453971ee576834" );
    }

    #[test]
    fn old_hash3() {
      let hash: Felt = hash3( Felt::from_u32(111) , Felt::from_u32(222) , Felt::from_u32(333) );
      println!("hash = {}", hash);
      assert_eq!( Felt::to_hex_string( hash ) , "0x199e9230763587614801abcca27e80a7a87bd1a32be92dc8e0b64274b1a04c67" );
    }

  }

}

//------------------------------------------------------------------------------
// "new" set of constants

pub mod new {
  use crate::bn254::field::{Felt};
  pub use crate::poseidon2::permutation::{Params,Poseidon2Params};

  pub fn permute<const T: usize>(input: [Felt; T]) -> [Felt; T] where Params: Poseidon2Params<true,T> {
    crate::poseidon2::permutation::permute::<true,T>( input )
  }

  pub fn compress<const K: usize>(input: [Felt; K]) -> Felt where Params: Poseidon2Params<true,{K+1}> {
    crate::poseidon2::permutation::compress::<true,K>( input )
  }

  pub fn hash1(a: Felt) -> Felt {
    compress::<1>([ a ])
  }
  
  pub fn hash2(a: Felt, b: Felt) -> Felt {
    compress::<2>([ a, b ])
  }
  
  pub fn hash3(a: Felt, b: Felt, c: Felt) -> Felt {
    compress::<3>([ a, b, c ])
  }

  //--------------------------------------------------------
  // tests for the "new" permutations

  #[cfg(test)]
  mod test {

    use crate::bn254::field::{Felt};
    use super::super::aux::*;
    use super::*;

    #[test]
    fn new_permute2_kat() {
      let out: [Felt; 2] = permute::<2>( kat_input::<2>() );
      print_state::<2>( "output" , out );
      assert_eq!( Felt::to_hex_string( out[0] ) , "0x1d01e56f49579cec72319e145f06f6177f6c5253206e78c2689781452a31878b" );
      assert_eq!( Felt::to_hex_string( out[1] ) , "0x0d189ec589c41b8cffa88cfc523618a055abe8192c70f75aa72fc514560f6c61" );
    }

    #[test]
    fn new_permute3_kat() {
      let out: [Felt; 3] = permute::<3>( kat_input::<3>() );
      print_state::<3>( "output" , out );
      assert_eq!( Felt::to_hex_string( out[0] ) , "0x0bb61d24daca55eebcb1929a82650f328134334da98ea4f847f760054f4a3033" );
      assert_eq!( Felt::to_hex_string( out[1] ) , "0x303b6f7c86d043bfcbcc80214f26a30277a15d3f74ca654992defe7ff8d03570" );
      assert_eq!( Felt::to_hex_string( out[2] ) , "0x1ed25194542b12eef8617361c3ba7c52e660b145994427cc86296242cf766ec8" );
    }

    #[test]
    fn new_permute4_kat() {
      let out: [Felt; 4] = permute::<4>( kat_input::<4>() );
      print_state::<4>( "output" , out );
      assert_eq!( Felt::to_hex_string( out[0] ) , "0x01bd538c2ee014ed5141b29e9ae240bf8db3fe5b9a38629a9647cf8d76c01737" );
      assert_eq!( Felt::to_hex_string( out[1] ) , "0x239b62e7db98aa3a2a8f6a0d2fa1709e7a35959aa6c7034814d9daa90cbac662" );
      assert_eq!( Felt::to_hex_string( out[2] ) , "0x04cbb44c61d928ed06808456bf758cbf0c18d1e15a7b6dbc8245fa7515d5e3cb" );
      assert_eq!( Felt::to_hex_string( out[3] ) , "0x2e11c5cff2a22c64d01304b778d78f6998eff1ab73163a35603f54794c30847a" );
    }

    #[test]
    fn new_hash1() {
      let hash: Felt = hash1( Felt::from_u32(111) );
      println!("hash = {}", hash);
      assert_eq!( Felt::to_hex_string( hash ) , "0x04039c0cf1c60a357ab0fe3acf0687aa773da2ad16531da75c014fb54eac55e2" );
    }

    #[test]
    fn new_hash2() {
      let hash: Felt = hash2( Felt::from_u32(111) , Felt::from_u32(222) );
      println!("hash = {}", hash);
      assert_eq!( Felt::to_hex_string( hash ) , "0x0af73d8a3f066649dbdb14e1c9923a0d1cdd822d22f1e8dabcec606131706093" );
    }

    #[test]
    fn new_hash3() {
      let hash: Felt = hash3( Felt::from_u32(111) , Felt::from_u32(222) , Felt::from_u32(333) );
      println!("hash = {}", hash);
      assert_eq!( Felt::to_hex_string( hash ) , "0x1e21998b4bed0485cbf36d22656d04b5b49c4d2018afd95dae20c837e7c7f653" );
    }

  }

}

//------------------------------------------------------------------------------

