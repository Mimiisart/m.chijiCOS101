fn main() {
   //initialize a mutable tuple
   let mut mountain_heights= ("Everest",8848,"Fishtail",6993);

   println!("Original tuple = {:?}",mountain_heights);

   //change 3rd and 4th element of a mutable tuple\
   mountain_heights.2 = "lhotse";
   mountain_heights.3 = 8516;

   println!("changed tuple = {:?}",mountain_heights);
}
