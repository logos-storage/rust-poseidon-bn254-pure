
use rust_poseidon1::bigint::*;
use rust_poseidon1::constant::*;
//use rust_poseidon1::montgomery::*;

//------------------------------------------------------------------------------

const BIG1 : BigInt<8> = BigInt::make( [ 0x191eb5c7 , 0xe4db8a57 , 0xb0151d71 , 0xdc3f0f00 , 0x76b2f85c , 0xc80322cb , 0x42d60bd1 , 0x3142edd0 ] );
const BIG2 : BigInt<8> = BigInt::make( [ 0x8cbc1b97 , 0x03e4f372 , 0x0b0c5fe2 , 0x9c417d29 , 0x987ee952 , 0x0ac1f5a6 , 0x086f97e3 , 0x7d7cf5c0 ] );
const BIG3 : BigInt<8> = BigInt::make( [ 0xb1efa2d1 , 0xf9fff49b , 0x406c5062 , 0xa931be03 , 0xe185427e , 0x2317351b , 0x195a120a , 0xbbfc0dc5 ] );

//------------------------------------------------------------------------------
 
fn main() {

  BigInt::print("BIG1",&BIG1);
  BigInt::print("BIG2",&BIG2);
  BigInt::print("BIG3",&BIG3);

  println!("");

  let (x,c) = BigInt::addCarry( &BIG1, &BIG2 );
  let (y,d) = BigInt::addCarry( &BIG2, &BIG3 );
  let (z,e) = BigInt::addCarry( &BIG3, &BIG1 );

  println!("B1+B2 = {} + {}", &x, c);  
  println!("B2+B3 = {} + {}", &y, d);  
  println!("B3+B1 = {} + {}", &z, e);  

  let u: BigInt<16> = BigInt::<8>::mul( &BIG1, &BIG2 );
  println!("B1*B2 = {}", &u);  
}