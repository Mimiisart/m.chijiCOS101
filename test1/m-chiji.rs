
use std::io;
fn main() {
  let mut voters = 0;
  loop{
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


  println!("Are you a current class rep,(TRUE OR FALSE)");
  io::stdin().read_line(&mut input1).expect("failed to read_line");
  let class_rep:bool = input1.trim().parse().expect("failed to input");

  println!("Are you in 100 level");
  io::stdin().read_line(&mut input2).expect("failed to read_line");
  let level:u32 = input2.trim().parse().expect("failed to input");

  println!("Enter cgpa");
  io::stdin().read_line(&mut input3).expect("failed to read_line");
  let cgpa:f32 = input3.trim().parse().expect("failed to input");

  println!("are you a class rep {} and are you in 100 level {} and  cgpa {}",class_rep, level, cgpa);
    

        if voters==50{
            break;}
    if class_rep && level>100 && cgpa>4.00 {
    println!("Name: {}",name);
    println!("Email: {}",email);
    println!("Department: {}",department);
    println!("state of origin: {}",state_of_origin);
    println!("you are eligible to vote");
    voters+=1;
}
    else if class_rep && level<=100 && cgpa<=4.00{
    println!("Not eligible to vote");
    }
}
}

