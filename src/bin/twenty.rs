
use rust_poseidon_bn254_pure::bn254::field::*;
use rust_poseidon_bn254_pure::poseidon2::permutation::*;

fn main() {

  println!("iterating Poseidon2 twenty times");

  let input  = [ Felt::from_u32(0) , Felt::from_u32(1) , Felt::from_u32(2) ];

  println!("x  = {}", input[0] );
  println!("y  = {}", input[1] );
  println!("z  = {}", input[2] );

  let mut state: [Felt; 3] = input.clone(); 
  for _i in 0..20 {
    state = permute_felt(state);
  }
  println!("x' = {}", state[0] );
  println!("y' = {}", state[1] );
  println!("z' = {}", state[2] );

}