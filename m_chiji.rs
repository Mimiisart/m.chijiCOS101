

use std::io;
fn main() {
   let mut name = String::new();
    let mut paper= String::new();

println!("Enter name");
    io::stdin()
        .read_line(&mut name)
        .expect("failed to readline");
    let name= name.trim();

println!("how many paper have you published");
    io::stdin()
        .read_line(&mut paper)
        .expect("failed to readline");
 let paper:i32= paper.trim().parse().expect("wrong");

    println!("name {} and paper {}",name,paper);

    if paper>=3 && paper<=5{
        println!("incentive is N500,000");
    }
    else if paper>=5 && paper<10{
        println!("incentive is N800,000");

    }
    else if paper>10{ 
        println!("incentive is N100,000,000");

    }
    else if paper<3{
        println!("incentive is N100,000 ");
    }