use std::io::Write;
fn main() {
   let announce = "week 9 - Ruat File Input\n";
   let dept ="Department of computer science";

   let mut file =std::fs::File::create("data.txt").expect("create failed");
   file.write_all("Welcome to Rust Programming\n"
   .as_bytes()).expect("Write failed");
file.write_all(announce.as_bytes()).expect("write failed");
file.write_all(dept.as_bytes()).expect("write failed");
println!("\nData written to file.");

}
