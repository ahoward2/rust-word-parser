use std::fs;

fn main() {

	let contents = fs::read_to_string("../words.txt").expect("Something went wrong reading the file");

	println!("\nFull content of the text file: \n\n{}",contents);

	let split = contents.split(" ");

	let vec = split.collect::<Vec<&str>>();
	
	println!("Word count: {}", vec.len());

}
