use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter a");
    io::stdin()
        .read_line(&mut input1)
        .expect("failed to read input");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter b");
    io::stdin()
        .read_line(&mut input2)
        .expect("failed to read input");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter c");
    io::stdin()
        .read_line(&mut input3)
        .expect("failed to read input");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    let d = (b * b) - (4.0 * a * c);
    println!("discriminant is {}",d);

    if d > 0.0 {
        let x1 = (-b + d.sqrt())/(2.0 * a);
        let x2 = (-b - d.sqrt())/(2.0 * a);
        println!("The roots are {} and {}",x1,x2);

    }
    else if d == 0.0{
        let x1 = (-b + d.sqrt())/(2.0 * a);
        println!("The root is {}",x1);

    }
    else if d < 0.0{
        println!("The equation has no roots");
    }
}
