
#![allow(unused)]

use std::time::Instant;

use rust_poseidon_bn254_pure::bn254::bigint::*;
use rust_poseidon_bn254_pure::bn254::constant::*;
use rust_poseidon_bn254_pure::bn254::montgomery::*;
use rust_poseidon_bn254_pure::bn254::field::*;

use rust_poseidon_bn254_pure::poseidon2::permutation::*;

use rust_poseidon_bn254_pure::poseidon::permutation::*;

//------------------------------------------------------------------------------

type Big = BigInt<8>;

const BIG1  : Big  = BigInt::make( [ 0x096113a8 , 0x5734d4ed , 0xef9d7088 , 0xf5b8189d , 0x9941cff9 , 0x233b0905 , 0x1d027fb4 , 0x7279de57 ] );
const BIG2  : Big  = BigInt::make( [ 0x7efecde8 , 0xa15ee255 , 0x10e87829 , 0x0f02eb31 , 0x74348ba5 , 0x6c7965aa , 0xdba5852c , 0x997936ef ] );
const BIG3  : Big  = BigInt::make( [ 0x928f0277 , 0xbfe4f704 , 0x382292f7 , 0x4e01efdc , 0x387b826d , 0x432e0d72 , 0x25c1e982 , 0x2437a9f1 ] );

const FELT1 : Felt = Felt::unsafe_make( [ 0x20dc4a9f , 0x00d47967 , 0xca5c5b8a , 0xd03cc3b9 , 0xbf51bf7f , 0x5ab6f194 , 0x6024036b , 0x22ac1ee6 ] );
const FELT2 : Felt = Felt::unsafe_make( [ 0xe34d39b8 , 0x1f9c48a6 , 0xeb5f4c17 , 0x703bcf35 , 0xa362c094 , 0x596d982a , 0x7b59b4fa , 0x2063f06a ] );
const FELT3 : Felt = Felt::unsafe_make( [ 0x31bfd5d3 , 0xa1c33f88 , 0x728b9d4d , 0x188cc945 , 0xf8f492c5 , 0xa574aefc , 0xd3bbaebc , 0x07d89e99 ] );

const MONT1 : Mont = Mont::unsafe_make( [ 0xc7a4b4fc , 0xf7cb6585 , 0xc62c6b29 , 0x2216c484 , 0xc0338416 , 0x6a74e6c0 , 0xcdabd868 , 0x095fad8e ] );
const MONT2 : Mont = Mont::unsafe_make( [ 0x9c974559 , 0x0b62d2a1 , 0xa55c6560 , 0xf1226480 , 0x5947fe1e , 0x5f830d50 , 0x42e6d6bc , 0x01a6741e ] );
const MONT3 : Mont = Mont::unsafe_make( [ 0x24a2de63 , 0xfbb9d8d0 , 0x671492ce , 0x9c15ed08 , 0x73d11ffa , 0xfacdfcdc , 0x8bf5eb71 , 0x2e493a4b ] );

//------------------------------------------------------------------------------
 
fn main() {

  BigInt::debug_print("PRIME", FIELD_PRIME);

  println!("");

  BigInt::debug_print("R1", BIG_R1);
  BigInt::debug_print("R2", BIG_R2);
  BigInt::debug_print("R3", BIG_R3);

  println!("");

  BigInt::debug_print("BIG1", BIG1);
  BigInt::debug_print("BIG2", BIG2);
  BigInt::debug_print("BIG3", BIG3);

  println!("");

  Mont::debug_print_internal("MONT1", MONT1);
  Mont::debug_print_internal("MONT2", MONT2);
  Mont::debug_print_internal("MONT3", MONT3);
  println!("-----");
  Mont::debug_print_standard("MONT1", MONT1);
  Mont::debug_print_standard("MONT2", MONT2);
  Mont::debug_print_standard("MONT3", MONT3);

  println!("");

  Felt::debug_print("FELT1", FELT1);
  Felt::debug_print("FELT2", FELT2);
  Felt::debug_print("FELT3", FELT3);

  println!("");
  println!("bigint:");
  println!("");

  let (x,c) = BigInt::addCarry( BIG1, BIG2 );
  let (y,d) = BigInt::addCarry( BIG2, BIG3 );
  let (z,e) = BigInt::addCarry( BIG3, BIG1 );

  println!("B1+B2 = {} + {}", x, c);  
  println!("B2+B3 = {} + {}", y, d);  
  println!("B3+B1 = {} + {}", z, e);  
  println!("-----");

  let u: BigInt<16> = BigInt::<8>::mul( BIG1, BIG2 );
  println!("B1*B2 = {}", u);  

  println!("");
  println!("montgomery:");
  println!("");

  println!("M1+M2 = {}", MONT1 + MONT2 );  
  println!("M2+M3 = {}", MONT2 + MONT3 );  
  println!("M3+M1 = {}", MONT3 + MONT1 );  
  println!("-----");

  println!("M1*M2 = {}", MONT1 * MONT2 );  
  println!("M2*M3 = {}", MONT2 * MONT3 );  
  println!("M3*M1 = {}", MONT3 * MONT1 );  

  println!("");
  println!("felt (standard repr):");
  println!("");

  println!("F1+F2 = {}", FELT1 + FELT2 );  
  println!("F2+F3 = {}", FELT2 + FELT3 );  
  println!("F3+F1 = {}", FELT3 + FELT1 );  
  println!("-----");

  println!("F1*F2 = {}", FELT1 * FELT2 );  
  println!("F2*F3 = {}", FELT2 * FELT3 );  
  println!("F3*F1 = {}", FELT3 * FELT1 );  

  //----------------------------------------------------------------------------

  println!("");
  println!("poseidon2 KAT:");
  println!("");

  let input  = [ Felt::from_u32(0) , Felt::from_u32(1) , Felt::from_u32(2) ];
  let output = permute_felt( input );

  println!("x  = {}", input[0] );
  println!("y  = {}", input[1] );
  println!("z  = {}", input[2] );

  println!("~> ");

  // expected output:
  //
  // x' = 0x30610a447b7dec194697fb50786aa7421494bd64c221ba4d3b1af25fb07bd103 
  // y' = 0x13f731d6ffbad391be22d2ac364151849e19fa38eced4e761bcd21dbdc600288 
  // z' = 0x1433e2c8f68382c447c5c14b8b3df7cbfd9273dd655fe52f1357c27150da786f 
  //
  println!("x' = {}", output[0] );
  println!("y' = {}", output[1] );
  println!("z' = {}", output[2] );

  println!("");
  println!("poseidon2 iterated 10,000 times:");
  println!("");

  let now = Instant::now();
  let mut state: [Felt; 3] = input.clone(); 
  for _i in 0..10000 {
    state = permute_felt(state);
  }

  // expected output:
  //
  // x'' = 0x27f23fcc813ee313937d46b6d5bab2df03fcb3cf1829f0332ba9f9968509f130
  // y'' = 0x138d88ea0ece1c9618254fe2146a6120080e16128467187bf1448e80f31eee3f
  // z'' = 0x1e51d60083aa3e8fa189e1c72844c5e09225f5977a834f53b471bf0de0dd59eb
  //
  println!("x'' = {}", state[0] );
  println!("y'' = {}", state[1] );
  println!("z'' = {}", state[2] );
  let elapsed = now.elapsed();
  println!("Elapsed: {:.3?}", elapsed);


  //----------------------------------------------------------------------------

  println!("");
  println!("sanity checking comparison with the prime");
  let one : Big = BigInt::from_u32(1);
  let a: Big = FIELD_PRIME + one;
  let b: Big = FIELD_PRIME      ; 
  let c: Big = FIELD_PRIME - one;
  println!("a = {}", a );
  println!("b = {}", b );
  println!("c = {}", c );
  println!("{} , {} , {}" , 
      BigInt::is_lt_prime(a) , 
      BigInt::is_lt_prime(b) , 
      BigInt::is_lt_prime(c) );

  //----------------------------------------------------------------------------

  {
    println!("");
    println!("conversion to/from bytes");
    let a  = FELT1;
    let xs = Felt::to_le_bytes(a);
    let b  = Felt::unsafe_from_le_bytes(xs);
    println!("a = {}",a);
    println!("b = {}",b);
    println!("le = {:?}",xs);
  }

  {
    let a  = FELT2;
    let ys = Felt::to_be_bytes(a);
    let b  = Felt::unsafe_from_be_bytes(ys);
    println!("a = {}",a);
    println!("b = {}",b);
    println!("be = {:?}",ys);
  }

  //----------------------------------------------------------------------------

/*
  println!("underlying repr = {:?} ", MONT1);
  println!("in hex = {}", MONT1);
  println!("in dec = {}", Mont::to_decimal_string(MONT1));
*/

  //  expected results:
  //
  //  compress1 = 18586133768512220936620570745912940619677854269274689475585506675881198879027
  //  compress2 = 7853200120776062878684798364095072458815029376092732009249414926327459813530
  //  compress3 = 6542985608222806190361240322586112750744169038454362455181422643027100751666
  //  compress4 = 18821383157269793795438455681495246036402687001665670618754263018637548127333

  let in1: Felt = Felt::from_u32(1);
  let out1 = compress_1(in1);
  println!("compress(1) = {}", Felt::to_decimal_string(out1) );

  let in2: [Felt; 2] = [ Felt::from_u32(1) , Felt::from_u32(2) ];
  let out2 = compress_2(in2);
  println!("compress(2) = {}", Felt::to_decimal_string(out2) );

  let in3: [Felt; 3] = [ Felt::from_u32(1) , Felt::from_u32(2) , Felt::from_u32(3) ];
  let out3 = compress_3(in3);
  println!("compress(3) = {}", Felt::to_decimal_string(out3) );

  let in4: [Felt; 4] = [ Felt::from_u32(1) , Felt::from_u32(2) , Felt::from_u32(3) , Felt::from_u32(4) ];
  let out4 = compress_4(in4);
  println!("compress(4) = {}", Felt::to_decimal_string(out4) );

}

