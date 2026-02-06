
use rust_poseidon_bn254_pure::bn254::field::*;
use rust_poseidon_bn254_pure::poseidon2;

fn main() {

  println!("iterating Poseidon2 twenty times");

  let input = poseidon2::aux::kat_input::<3>();

  println!("x  = {}", input[0] );
  println!("y  = {}", input[1] );
  println!("z  = {}", input[2] );

  let mut state: [Felt; 3] = input.clone(); 
  for _i in 0..20 {
    state = poseidon2::old::permute(state);
  }
  println!("x' = {}", state[0] );
  println!("y' = {}", state[1] );
  println!("z' = {}", state[2] );

}