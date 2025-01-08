struct Laptop{
    name:String,
    price:i32,
    quantity:u32,
}
fn main() {
    let hp = Laptop{
        name:String::from("hp"),
        price:650000,
        quantity:10,
    };
    let ibm = Laptop{
        name:String::from("ibm"),
        price:755000,
        quantity:6,
    };
    let toshiba= Laptop{
        name:String::from("toshiba"),
        price:550000,
        quantity:10,
    };
    let dell = Laptop{
        name:String::from("ibm"),
        price:850000,
        quantity:4,
    };
    let sum = hp.price*3 + ibm.price*3 + toshiba.price*3 +dell.price*3;
    println!("Sum is {}",sum);
}
