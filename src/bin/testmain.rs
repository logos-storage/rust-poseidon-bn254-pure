
use rust_poseidon1::bigint::*;
use rust_poseidon1::constant::*;
use rust_poseidon1::montgomery::*;
use rust_poseidon1::field::*;

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

  BigInt::print("PRIME",&FIELD_PRIME);

  println!("");

  BigInt::print("R1",&BIG_R1);
  BigInt::print("R2",&BIG_R2);
  BigInt::print("R3",&BIG_R3);

  println!("");

  BigInt::print("BIG1",&BIG1);
  BigInt::print("BIG2",&BIG2);
  BigInt::print("BIG3",&BIG3);

  println!("");

  Mont::print_internal("MONT1",&MONT1);
  Mont::print_internal("MONT2",&MONT2);
  Mont::print_internal("MONT3",&MONT3);
  Mont::print_standard("MONT1",&MONT1);
  Mont::print_standard("MONT2",&MONT2);
  Mont::print_standard("MONT3",&MONT3);

  println!("");

  Felt::print("FELT1",&FELT1);
  Felt::print("FELT2",&FELT2);
  Felt::print("FELT3",&FELT3);

  println!("");
  println!("bigint:");
  println!("");

  let (x,c) = BigInt::addCarry( &BIG1, &BIG2 );
  let (y,d) = BigInt::addCarry( &BIG2, &BIG3 );
  let (z,e) = BigInt::addCarry( &BIG3, &BIG1 );

  println!("B1+B2 = {} + {}", &x, c);  
  println!("B2+B3 = {} + {}", &y, d);  
  println!("B3+B1 = {} + {}", &z, e);  
  println!("-----");

  let u: BigInt<16> = BigInt::<8>::mul( &BIG1, &BIG2 );
  println!("B1*B2 = {}", &u);  

  println!("");
  println!("montgomery:");
  println!("");

  println!("M1+M2 = {}", Mont::add( &MONT1, &MONT2) );  
  println!("M2+M3 = {}", Mont::add( &MONT2, &MONT3) );  
  println!("M3+M1 = {}", Mont::add( &MONT3, &MONT1) );  
  println!("-----");

  println!("M1*M2 = {}", Mont::mul( &MONT1, &MONT2) );  
  println!("M1*M2 = {}", Mont::mul( &MONT1, &MONT2) );  
  println!("M1*M2 = {}", Mont::mul( &MONT1, &MONT2) );  

  println!("");
  println!("felt (standard repr):");
  println!("");

  println!("F1+F2 = {}", Felt::add( &FELT1, &FELT2) );  
  println!("F2+F3 = {}", Felt::add( &FELT2, &FELT3) );  
  println!("F3+F1 = {}", Felt::add( &FELT3, &FELT1) );  
  println!("-----");

  println!("F1*F2 = {}", Felt::mul( &FELT1, &FELT2) );  
  println!("F1*F2 = {}", Felt::mul( &FELT1, &FELT2) );  
  println!("F1*F2 = {}", Felt::mul( &FELT1, &FELT2) );  

}

