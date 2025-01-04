
use std::io;
fn main() {
  for x in 0..50{
    let mut input1 = String::new();
  let mut input2 = String::new();
  let mut input3 = String::new();
  let mut name =String::new();
  let mut email = String::new();
  let mut department = String::new();
  let mut state_of_origin = String::new();

  println!("Enter name");
  io::stdin().read_line(&mut name).expect("failed to read_line");
  let name = name.trim();
  
  println!("Enter email");
  io::stdin().read_line(&mut email).expect("failed to read_line");
  let email = email.trim();
  
  println!("department");
  io::stdin().read_line(&mut department).expect("failed to read_line");
  let department = department.trim();

  println!("Enter state of origin");
  io::stdin().read_line(&mut state_of_origin).expect("failed to read_line");
  let state_of_origin = state_of_origin.trim();


  println!("Are you a current class rep,(true OR false)");
  io::stdin().read_line(&mut input1).expect("failed to read_line");
  let class_rep:bool = input1.trim().parse().expect("failed to input");

  println!("What level are you in");
  io::stdin().read_line(&mut input2).expect("failed to read_line");
  let level:u32 = input2.trim().parse().expect("failed to input");

  println!("Enter cgpa");
  io::stdin().read_line(&mut input3).expect("failed to read_line");
  let cgpa:f32 = input3.trim().parse().expect("failed to input");

  {

    if class_rep==true && level>100 && cgpa>4.00 {
    println!("Name: {}",name);
    println!("Email: {}",email);
    println!("Department: {}",department);
    println!("state of origin: {}",state_of_origin);
     println!("you are eligible to vote");
    }
    
     else {
    println!("Not eligible to vote");
    }
    }
    }
  }



