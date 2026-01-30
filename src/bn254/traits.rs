
// rust have things like independent Neg, Add, Sub, Mul, all going to 
// arbritrary whatever third types (...), but they missed this one, hah!

pub trait Zero {
  fn zero() -> Self;
  fn is_zero(x: Self) -> bool;
}

pub trait One {
  fn one() -> Self;
  fn is_one(x: Self) -> bool;
}