use std::io;
fn main() {

  let mut input1 = String::new();
  let mut input2 = String::new();

  println!("menu\t\t\tprice\n");
  println!("P=Poundo yam/Edikaiko soup\t-N3200\n");
  println!("F=Fried rice & chicken\t\t-N3000\n");
  println!("A=Amala & ewedu soup\t\t-N2500\n");
  println!("E=Eba & egusi soup\t\t-N2000\n");
  println!("W=white rice & stew\t\t-N2500\n");

  let P:&str =  "Poundo yam/Edikaiko soup";
  let F:&str = "Fried rice & chicken";
  let A:&str = "Amala & ewedu soup";
  let E:&str = "Eba & Egusi soup";
  let W:&str = "White rice & stew";
  
  println!("What would you like to order");
    io::stdin()
        .read_line(&mut input1)
        .expect("failed to read input");
    let a:&str = input1.trim();


    
    println!("how many portions");
    io::stdin()
        .read_line(&mut input2)
        .expect("failed to read input");
    let b:u32 = input2.trim().parse().expect("Not available");


    if a =="P"{
      let price=3200 * b;
      if price > 10000 {
        let discount_price = price - (price * 5/100);
        println!("your total bill after discount is N{}",discount_price);
      }
      else {
        println!("your total bill is N{}",price);
      }
    }
    else if a =="F"{
      let price=3000 * b;
      if price > 10000 {
        let discount_price = price - (price * 5/100);
        println!("your total bill after discount is N{}",discount_price);
      }
      else {
        println!("your total bill is N{}",price);
      } 
    }
    else if a =="A"{
      let price=2500 * b;
      if price > 10000 {
        let discount_price = price - (price * 5/100);
        println!("your total bill after discount is N{}",discount_price);
      }
      else {
        println!("your total bill is N{}",price);
      }

    }
    else if a =="E"{
      let price=2000 * b;
      if price > 10000 {
        let discount_price = price - (price * 5/100);
        println!("your total bill after discount is N{}",discount_price);
      }
      else {
        println!("your total bill is N{}",price);
      }

    }
    else if a =="W"{
      let price=2500 * b;
      if price > 10000 {
        let discount_price = price - (price * 5/100);
        println!("your total bill after discount is N{}",discount_price);
      }
      else {
        println!("your total bill is N{}",price);
      }
    }

}