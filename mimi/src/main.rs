//pass by reference
let _x:i32 = 5;
let _r:&i32 = &_x;
println!("value of x is {}",_x);
println!("value of x is {}",_r);