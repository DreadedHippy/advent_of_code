use std::{fs::File, io::Read};

use advent_of_code::{day1, day2, day3, day4};

fn main() {    
	// Day 1
	// let mut input_file = File::open("src/day1/puzzle.txt").expect("Failed to find 'puzzle.txt'");
	// let mut input_string = String::new();
	// input_file.read_to_string(&mut input_string).expect("Failed to read file to string, invalid UTF-8 might be present");


		// day1::solve_puzzle_1(input_string.clone());
		// day1::solve_puzzle_2(input_string);

  //   // Day 2
	// let mut input_file = File::open("src/day2/puzzle.txt").expect("Failed to find 'puzzle.txt'");
	// let mut input_string = String::new();
	// input_file.read_to_string(&mut input_string).expect("Failed to read file to string, invalid UTF-8 might be present");

		// day2::solve_puzzle_1(input_string.clone());
		// day2::solve_puzzle_2(input_string);

	// 	//Day 3
	// let mut input_file = File::open("src/day3/puzzle.txt").expect("Failed to find 'puzzle.txt'");
	// let mut input_string = String::new();
	// input_file.read_to_string(&mut input_string).expect("Failed to read file to string, invalid UTF-8 might be present");

	// 	day3::solve_puzzle_1(input_string.clone());
	// 	day3::solve_puzzle_2(input_string.clone());

		//Day 4
	let mut input_file = File::open("src/day4/puzzle.txt").expect("Failed to find 'puzzle.txt'");
	let mut input_string = String::new();
	input_file.read_to_string(&mut input_string).expect("Failed to read file to string, invalid UTF-8 might be present");

		day4::solve_puzzle_1(input_string);







}
