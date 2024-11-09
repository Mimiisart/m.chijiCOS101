use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter experience");
    io::stdin()
        .read_line(&mut input1)
        .expect("failed to readline");
    let experience: bool = input1.trim().parse().expect("Not a valid boolean");

    println!("Enter age");
    io::stdin()
        .read_line(&mut input2)
        .expect("failed to readline");
    let age: u32 = input2.trim().parse().expect("Not a valid number");

    println!("Experience {} and Age {}", experience, age);

    if experience && age >= 40 {
        println!("incentive is 1,560,000");
    } else if experience && age >= 30 && age < 40 {
        println!("incentive is 1,480,000");
    } else if experience && age <= 29 {
        println!("incentive is 1,300,000");
    } else if !experience {
        println!("incentive is 100,000");
    }
}
