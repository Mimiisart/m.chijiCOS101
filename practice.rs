//OWNERSHIP IN RUST
//1-EACH VALUE IN RUST HAS A VARIABLE THAT ITS OWNS
//E.G
fn main(){
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("length of '{}' is {}.",s1,len);
}
fn calculate_length(s:&String) -> usize {s.len()}
//there can only be one owner at a time
let s2 = s1
