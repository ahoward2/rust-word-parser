use std::fs;
use std::path::Path;

fn main() {

	let path = Path::new("../text-files");

	for entry in fs::read_dir(path).expect("Unable to read directory") {

		let entry = entry.expect("Unable to get entry");
		
		let entry_path = entry.path();

		let entry_name = entry.file_name();

		let unwrapped_name = entry_name.to_str().unwrap();

		let contents = fs::read_to_string(entry_path).expect("Something went wrong reading the file");

		println!("\nFull content of the file {name}: \n\n{file_contents}", name=unwrapped_name, file_contents=contents);

		let new_content = &contents.replace("\n", "");

		let split = new_content.split(" ");

		let vec = split.collect::<Vec<&str>>();

		println!("Word count: {}", vec.len());
	}
}
