use std::env;

fn main() {
  println!("hello");
  for argument in env::args().skip(1) {
    println!("{}", argument);
  }

  println!("{}", add(1,3));
}

pub fn add(a: i32, b: i32) -> i32 {
  return a + b;
}
