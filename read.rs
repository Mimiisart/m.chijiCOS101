fn main(){

    tell_height(182);
   human_id(  "mimi",
   14
   ,156.7);
   
}
fn tell_height(height:u32){
println!("tell height: {}cm",height);
}

fn human_id(name:&str,age:u32,height:f32){
    println!("My name is {},i am {} years old and {}cm tall",name,age,height);


    let price:i32=5;
    let qty:i32 =10;
    let mult = price * qty;
    println!("result is {}",mult);

   
}