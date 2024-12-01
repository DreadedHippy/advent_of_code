use std::{fs::File, io::Read};
pub mod year_2024;

fn main() {
	year_2024::solve();
}

pub fn read_puzzle_input(s: &str) -> String {
	let mut input_file = File::open(s).expect("Failed to find 'puzzle.txt'");
	let mut input_string = String::new();
	input_file.read_to_string(&mut input_string).expect("Failed to read file to string, invalid UTF-8 might be present");

	return input_string

}
