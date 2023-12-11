use std::collections::{HashMap, HashSet};

type NumberCoordinates = (i32, (i32, i32));
type NumberInformation = (NumberCoordinates, i32);

type GearCoordinates = (i32, i32);

const AROUND_COORDINATES: [(i32, i32); 8] = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];

pub fn solve_puzzle_1(input_string: String){
	let input_as_grid = input_string.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();


	let mut all_number_information = Vec::new();

	for (row, line) in input_string.lines().enumerate() {
		let mut nums_position: [i32; 2] = [-1, -1];
		let mut number_value = 0;

		for (char_index, character) in line.chars().enumerate() {
			if character.is_digit(10){
				let char_as_number = character as u32 - '0' as u32;
				if nums_position[0] == -1 {
					nums_position = [char_index as i32; 2];
				} else {
					nums_position[1] = char_index as i32
				}

				number_value = (number_value*10) + char_as_number;
				
			} else {
				if nums_position != [-1, -1] {
					let number_coordinates: NumberCoordinates = (row as i32, (nums_position[0], nums_position[1]));
					let number_info: NumberInformation = (number_coordinates, number_value as i32);

					all_number_information.push(number_info);

					nums_position = [-1, -1];
					number_value = 0;
				}
			}
		}
	}

	/////////// CORRECT

	let line_length = input_as_grid[0].len() as i32;

	let mut sum_of_values: usize = 0;

	'move_through_number_information: for number_information in all_number_information {
		let ((row, (start, end)), value) = number_information;

		for x_coordinate in start..=end {
			
			for around_coordinate in AROUND_COORDINATES {
				let (new_x_coordinate, new_y_coordinate) = (around_coordinate.0 + x_coordinate, row + around_coordinate.1);

				if new_x_coordinate < 0 || new_y_coordinate < 0 || new_x_coordinate > line_length - 1 || new_y_coordinate > (input_as_grid.len() - 1) as i32 {
					continue;
				}	else {
					let character = input_as_grid[new_y_coordinate as usize][new_x_coordinate as usize];

					if !character.is_digit(10) && character != '.' {
						sum_of_values += value as usize;

						continue 'move_through_number_information;
					}

				}
			}
		}
	}

	println!("{}", sum_of_values);

	// println!("{:?}", all_number_information);
}


pub fn solve_puzzle_2(input_string: String){
	let input_as_grid = input_string.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();


	let mut all_number_information = Vec::new();

	for (row, line) in input_string.lines().enumerate() {
		let mut nums_position: [i32; 2] = [-1, -1];
		let mut number_value = 0;

		for (char_index, character) in line.chars().enumerate() {
			if character.is_digit(10){
				let char_as_number = character as u32 - '0' as u32;
				if nums_position[0] == -1 {
					nums_position = [char_index as i32; 2];
				} else {
					nums_position[1] = char_index as i32
				}

				number_value = (number_value*10) + char_as_number;
				
			} else {
				if nums_position != [-1, -1] {
					let number_coordinates: NumberCoordinates = (row as i32, (nums_position[0], nums_position[1]));
					let number_info: NumberInformation = (number_coordinates, number_value as i32);

					all_number_information.push(number_info);

					nums_position = [-1, -1];
					number_value = 0;
				}
			}
		}
	}

	/////////// CORRECT

	let line_length = input_as_grid[0].len() as i32;

	let mut sum_of_values: usize = 0;

	let mut engine_parts_map: HashMap<GearCoordinates, HashSet<NumberInformation>> = HashMap::new();

	'move_through_number_information: for number_information in all_number_information {
		let ((row, (start, end)), value) = number_information;

		for x_coordinate in start..=end {
			
			for around_coordinate in AROUND_COORDINATES {
				let (new_x_coordinate, new_y_coordinate) = (around_coordinate.0 + x_coordinate, row + around_coordinate.1);

				if new_x_coordinate < 0 || new_y_coordinate < 0 || new_x_coordinate > line_length - 1 || new_y_coordinate > (input_as_grid.len() - 1) as i32 {
					continue;
				}	else {
					let character = input_as_grid[new_y_coordinate as usize][new_x_coordinate as usize];

					if character == '*' {
						engine_parts_map.entry((new_x_coordinate, new_y_coordinate)).or_insert(HashSet::new()).insert(number_information);

						// continue 'move_through_number_information;
					}

				}
			}
		}
	}

	// println!("{:?}", engine_parts_map);

	let gear_ratios: Vec<Vec<i32>> = engine_parts_map.
		into_values()
		.filter(|x| x.len() == 2)
		.map(
			|x| x.into_iter().map(
				|x| x.1)
			.collect::<Vec<i32>>()
		)
		.collect();

	let mut sum_of_gear_ratios: usize = 0;


	for ratio in &gear_ratios {
		sum_of_gear_ratios += ratio[0] as usize * ratio[1] as usize;
	}
	// .fold(1, |acc, v| acc + (v[0] * v[1]));

	// println!("{:?}", gear_ratios);

	println!("{}", sum_of_gear_ratios);

	// println!("{:?}", all_number_information);
}