
fn main() {
	//arrays
	let fruits =["banana","apple","pear"];
	println!("fruits are {:?}",fruits);
	println!("fruit is {:?}",fruits[0]);

	//tuple
	let human:(String,i32,bool)=("Alice".to_string(),30,false);
	println!("Human Tuple:{:?}",human);

	//slices:[1,2,3,4,5]
	let number_slices:&[i32]=&[1,2,3,4,5];
	println!("Numner slices: {:?}",number_slices);

	//slices
	let book_slices:&[&str]=&["it","harry potter"];
	println!("book slices: {:?}",book_slices);

     let mut stone_cold:String = String::from("hell, ");
	 stone_cold.push_str("Yeah!");
	 println!("stone cold is {}",stone_cold);

	 let string:String = String::from("hell, ");
	 let slice:&str=&string[0..5];

	 
}