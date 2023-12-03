
#[derive(Debug)]
enum CubeVariant {
	Red,
	Blue,
	Green
}

#[derive(Debug)]
struct CubeGroup(i32, CubeVariant);
type CubeSet = Vec<CubeGroup>;
type CubeCollection = Vec<CubeSet>;

#[derive(Debug)]
struct Game {
	game_id: i32,
	cube_collection: CubeCollection
}

type PuzzleInput = Vec<Game>;


pub fn solve_puzzle_1(input_string: String) {
	let puzzle_input = parse_puzzle(input_string);

	let mut sum_of_possible_ids = 0;

	for input in puzzle_input {
		let is_possible = input.cube_collection.iter().flatten().map(|group| 
			{
				match &group {
					CubeGroup(x, CubeVariant::Red) => x <= &12,
					CubeGroup(x, CubeVariant::Green) => x <= &13,
					CubeGroup(x, CubeVariant::Blue) => x <= &14,
				}
			}
		).all(|x| x == true);

		if is_possible {
			sum_of_possible_ids+= input.game_id
		}
	}

	println!("DAY 2, PUZZLE 1 ANSWER: {}", sum_of_possible_ids);


}


pub fn solve_puzzle_2(input_string: String) {
	let puzzle_input = parse_puzzle(input_string);

	let mut total_power_of_sets: usize = 0;

	for input in puzzle_input {
		let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);

		for cube_set in input.cube_collection {
			for group in cube_set {
				match group {
					CubeGroup(n, CubeVariant::Red) => {min_red = min_red.max(n)},
					CubeGroup(n, CubeVariant::Blue) => {min_blue = min_blue.max(n)},
					CubeGroup(n, CubeVariant::Green) => {min_green = min_green.max(n)},
				}
			}
		}

		total_power_of_sets+= (min_red * min_green * min_blue) as usize
	}

	println!("DAY 2, PUZZLE 2 ANSWER: {}", total_power_of_sets);


}

fn parse_puzzle(input_string: String) -> PuzzleInput {
	let mut puzzle_input: PuzzleInput = Vec::new();

	for line in input_string.lines() {
		let info: Vec<String> = line.split(":").map(String::from).collect();

		// println!("{:?}", info);
		let game_id = info[0].split_whitespace().last().map(|x| x.parse::<i32>().unwrap()).unwrap();

		let mut cube_collection: Vec<CubeSet> = Vec::new();

		let cubes = info[1].trim();

		let collections = cubes.split(";").map(|set| set.trim().to_string()).collect::<Vec<String>>();
		
		for collection in collections {
			let set = collection.split(",").map(|c| c.trim().to_string()).collect::<Vec<String>>();

			let mut cube_set: Vec<CubeGroup> = Vec::new();
			for group in set {
				let cube_info = group.split_whitespace().map(|info| info.trim().to_string()).collect::<Vec<String>>();

				let cubequantity = cube_info[0].parse::<i32>().unwrap();
				let cubevariant = match cube_info[1].as_str() {
					"red" => CubeVariant::Red,
					"green" => CubeVariant::Green,
					"blue" => CubeVariant::Blue,
					_ => {println!("SOMETHING WENT WRONG"); CubeVariant::Red}
				};

				let res: CubeGroup = CubeGroup(cubequantity, cubevariant);
				cube_set.push(res);

				// println!("GROUP: {}",group);
			}

			cube_collection.push(cube_set)
		}

		// println!("{:?}",cube_set);

		let game: Game = Game { game_id, cube_collection };
		puzzle_input.push(game);

	};

	puzzle_input
}