use std::io;
fn main() {
    println!("Welcome");
   println!("Select an equation :");
   println!("Area of trapezium formula = (height/2*(base1+base2)"); 
   println!("Area of rhombus formula = 1/2 *diagonal1 *diagonal2");
   println!("Area of parallelogram formula = base * altitude");
   println!("Area of cube formula = 6 *(length of side)^2");
   println!("Volume of cylinder formula = pie * radius^2*height");

   let mut input1 = String::new();
   

    println!("Enter trapezium/rhombus/parallelogram/cube/cylinder");
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let answer: &str = input1.trim();
    //a is height b is base1 c is base2
    fn area_of_trapezium() {
        let mut a = String::new();
        let mut b = String::new();
        let mut c = String::new();


        println!("Enter height");
        io::stdin().read_line(&mut a).expect("failed to read input");
        let h:f32=a.trim().parse().expect("invalid");

        println!("Enter base1");
        io::stdin().read_line(&mut b).expect("failed to read input");
        let b1:f32=b.trim().parse().expect("invalid");

        println!("Enter base2");
        io::stdin().read_line(&mut c).expect("failed to read input");
        let b2:f32=a.trim().parse().expect("invalid");

        let answer = (h/2.0)*(b1 + b2);
        println!("Your answer is {}",answer);
    }

    fn area_of_rhombus() {
        let mut a= String::new();
        let mut b = String::new();
        


        println!("Enter diagonal1");
        io::stdin().read_line(&mut a).expect("failed to read input");
        let d1:f32=a.trim().parse().expect("invalid");

        println!("Enter diagonal2");
        io::stdin().read_line(&mut b).expect("failed to read input");
        let d2:f32=b.trim().parse().expect("invalid");

            let answer = (1.0/2.0)*d1*d2;
        println!("Your answer is {}",answer);
    }

    fn area_of_parallelogram() {
        let mut a= String::new();
        let mut b = String::new();
        


        println!("Enter base");
        io::stdin().read_line(&mut a).expect("failed to read input");
        let base:f32=a.trim().parse().expect("invalid");

        println!("Enter altitude");
        io::stdin().read_line(&mut b).expect("failed to read input");
        let altitude:f32=b.trim().parse().expect("invalid");

        let answer = base * altitude;
        println!("Your answer is {}",answer);
    }

    fn area_of_cube() {
        let mut a= String::new();

        println!("Enter length of the side");
        io::stdin().read_line(&mut a).expect("failed to read input");
        let l:f32=a.trim().parse().expect("invalid");


        let answer = 6.0 * (l).powf(2.0);
        println!("Your answer is {}",answer);
        }

    fn volume_of_cylinder() {
        let mut a = String::new();
        let mut b = String::new();
        


        println!("Enter radius");
        io::stdin().read_line(&mut a).expect("failed to read input");
        let r:f32=a.trim().parse().expect("invalid");

        println!("Enter height");
        io::stdin().read_line(&mut b).expect("failed to read input");
        let h:f32=b.trim().parse().expect("invalid");

        let answer = 3.142*(r).powf(2.0)* h;
        println!("Your answer is {}",answer);
    }

    if answer.to_lowercase() == "trapezium"{
        area_of_trapezium();
    } else if answer.to_lowercase() == "parallelogram"{
        area_of_parallelogram();
    } else if answer.to_lowercase() == "cube" {
        area_of_cube();
    } else if answer.to_lowercase() == "cylinder" {
        volume_of_cylinder();
    } else if answer.to_lowercase() == "rhombus"{
        area_of_rhombus();
    }
}
