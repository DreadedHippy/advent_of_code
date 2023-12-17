use std::collections::{HashSet, HashMap};



pub fn solve_puzzle_1(input_string: String) {
	let mut sum_of_points: usize = 0;

	for line in input_string.lines() {
		let info = &line.split(":").map(String::from).collect::<Vec<String>>()[1];



		let separated_info = info.split("|").map(|x| x.trim()).collect::<Vec<&str>>();
		let (winning_numbers, numbers_in_hand) = (separated_info[0].split_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>(), separated_info[1].split_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>());

		let mut set_of_numbers: HashSet<i32> = HashSet::from_iter(winning_numbers.into_iter());

		let mut matched_numbers = 0;

		for number_in_hand in numbers_in_hand {
			// If hashset already contained this value
			if !set_of_numbers.insert(number_in_hand) {
				matched_numbers+=1;
			}
		}

		let points_worth = if matched_numbers == 0 {0} else { 2_i32.pow(matched_numbers - 1)};

		sum_of_points+= points_worth as usize;


	}

	println!("{}", sum_of_points);
}


pub fn solve_puzzle_2(input_string: String) {


	let line_count = input_string.lines().count();
	let mut frequencies: Vec<usize> = Vec::new();

	for _ in 1..=line_count {
		frequencies.push(1);
	}


	for line in input_string.lines() {
		let info_string = &line.split(":").map(String::from).collect::<Vec<String>>();
		let (card_info, info) = (&info_string[0], &info_string[1]);

		let card_id = card_info.split_whitespace().skip(1).next().unwrap().parse::<i32>().unwrap();

		// println!("{}", card_id);



		let separated_info = info.split("|").map(|x| x.trim()).collect::<Vec<&str>>();
		let (winning_numbers, numbers_in_hand) = (separated_info[0].split_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>(), separated_info[1].split_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>());

		let mut set_of_numbers: HashSet<i32> = HashSet::from_iter(winning_numbers.into_iter());

		let mut matched_numbers = 0;
		for number_in_hand in numbers_in_hand {
			// If hashset already contained this value
			if !set_of_numbers.insert(number_in_hand) {
				matched_numbers+=1;
			}
		}

		// Take it back 1 step to alter the actual index;
		let c = card_id - 1;
		// frequencies.push(0);

		if matched_numbers > 0 {
			for card in c+1..=(c+matched_numbers) {
				frequencies[card as usize]+= frequencies[c as usize]
			}
		}
	}

	println!("{}", frequencies.iter().sum::<usize>());
}