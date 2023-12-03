use core::panic;
use std::{fs::File, io::Read};

const NUMBERS_AS_LETTERS: [&str; 10] = [
	"ze", "on", "tw", "th", "fo", "fi", "si", "se", "ei", "ni"
];

pub fn solve_puzzle_1() {
	// Read input file
	let mut input_file = File::open("src/day1/puzzle1.txt").expect("Failed to find 'input.txt'");
	let mut input_string = String::new();

	// input_file.read_to_string(buf)

	input_file.read_to_string(&mut input_string).expect("Failed to read file to string, invalid UTF-8 might be present");

	let mut sum_of_numbers: usize = 0;

	for line in input_string.lines() {
		// println!("{}", line);

		let first_number = line.chars().filter(|x| x.to_digit(10) != None ).map(|x| x.to_digit(10).unwrap()).take(1).collect::<Vec<u32>>()[0];
		let last_number = line.chars().rev().filter(|x| x.to_digit(10) != None ).map(|x| x.to_digit(10).unwrap()).take(1).collect::<Vec<u32>>()[0];

		sum_of_numbers += ((first_number * 10) + last_number) as usize;
	}

	println!("SUM: {}", sum_of_numbers);
}

pub fn solve_puzzle_2() {
	// Read input file
	let mut input_file = File::open("src/day1/puzzle2.txt").expect("Failed to find 'input.txt'");
	let mut input_string = String::new();

	// input_file.read_to_string(buf)

	input_file.read_to_string(&mut input_string).expect("Failed to read file to string, invalid UTF-8 might be present");

	let mut sum_of_numbers: usize = 0;



	for line in input_string.lines() {

		let character_vector: Vec<char> = line.chars().collect();
		let mut first_number: u32 = 0;

		let mut word = String::new();

		for character in character_vector.iter() {
			match character.to_owned() {
				'0' | '1' | '2' | '3' | '4' | '5'| '6'| '7' | '8' | '9' => {
					first_number = character.to_owned() as u32 - '0' as u32;
					break;
				},
				_ => {
					word.push(character.to_owned());
					if let Some(n) = check_if_contains_number(&word, false) {
						first_number = n;
						word = String::new();
						break;
					}
				}
			}
		}


		let mut last_number: u32 = 0;

		let mut word = String::new();

		for character in character_vector.iter().rev() {
			let c = character.to_owned();
			match c {
				'0' | '1' | '2' | '3' | '4' | '5'| '6'| '7' | '8' | '9' => {
					last_number = c as u32 - '0' as u32;
					break;
				},
				_ => {
					word.push(c);
					if let Some(n) = check_if_contains_number(&word, true) {
						last_number = n;
						break;
					}
				}
			}
		}

		sum_of_numbers+= ((first_number * 10) + last_number) as usize;
	}

	println!("SUM: {}", sum_of_numbers);
}

fn check_if_contains_number(word: &String, reversed: bool) -> Option<u32> {

	let mut word = word.to_owned();
	if reversed {
		word = word.chars().rev().collect::<String>();
	}
	if word.contains("zero") {
		return Some(0)
	} else if word.contains("one") {
		return Some(1)
	} else if word.contains("two") {
		return Some(2)
	} else if word.contains("three") {
		return Some(3)
	} else if word.contains("four") {
		return Some(4)
	} else if word.contains("five") {
		return Some(5)
	} else if word.contains("six") {
		return Some(6)
	} else if word.contains("seven") {
		return Some(7)
	} else if word.contains("eight") {
		return Some(8)
	} else if word.contains("nine") {
		return Some(9)
	} else {return None}
}