

use std::io;

fn main() {
    println!("Enter a number");
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).expect("Not a valid string");
  let mut num:f32 = input1.trim().parse().expect("Not a valid number");


  while  num < 10.0 {
    println!("outside loop number value is {}", num);
    num+=1.0;
  }
  println!("outside loop number value is {}", num);
}
