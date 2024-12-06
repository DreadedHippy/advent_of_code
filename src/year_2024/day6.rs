use std::collections::{HashMap, HashSet};

pub fn day_6_first(input: String) -> (Vec<Vec<char>>, HashSet<(usize, usize)>, (usize, usize)){
	let mut grid = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut h = grid.len();
	let mut w = grid[0].len();

	let mut i = 0;
	let mut j = 0;

	'outer: while i < h {
		j = 0;
		while j < w {
			match grid[i][j] {
				'#' | '.' => {},
				_ => {break 'outer}
			}

			j += 1;
		}

		i += 1;
	}

	let mut dir = match grid[i][j] {
		'>' => Dir::Right,
		'^' => Dir::Up,
		'v' => Dir::Down,
		'<' => Dir::Left,
		_ => Dir::Up,
	};

	let mut set = HashSet::new();
	let mut start = (i, j);

	set.insert((i, j));

	loop {

		if !move_and_mark(h, w, &mut i, &mut j, &grid, &mut dir, &mut set) {
			break;
		}
	}

	println!("distincts: {}", set.len());
	return (grid, set, start);

}



pub fn day_6_second(input: String) {
	let (mut grid, visited, (i, j)) = day_6_first(input);
	// let mut grid = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut h = grid.len();
	let mut w = grid[0].len();

	let mut dir = match grid[i][j] {
		'>' => Dir::Right,
		'^' => Dir::Up,
		'v' => Dir::Down,
		'<' => Dir::Left,
		_ => Dir::Up,
	};

	let mut loops = 0;

	for (a, b) in visited {
		if grid[a][b] != '.' {continue;}

		grid[a][b] = '#';

		let mut map = vec![vec![0; w]; h];
		let mut ic = i;
		let mut jc = j;
		let mut d = dir.clone();
		let mut positions = 1;

		loop {
			if !move_and_check_loop(h, w, &mut ic, &mut jc, &grid, &mut d, &mut map,&mut loops) {
				break;
			}
		}

		grid[a][b] = '.'
	}

	println!("loop positions: {}", loops)

}

fn move_and_mark(h: usize, w: usize, i: &mut usize, j: &mut usize, grid: &Vec<Vec<char>>, dir: &mut Dir, set: &mut HashSet<(usize, usize)>) -> bool {
	match dir {
		Dir::Up => {
			if *i == 0 { return false }
			
			if grid[*i-1][*j] == '#' {
				*dir = Dir::Right;
				return true;
			}

			*i -= 1;

			set.insert((*i, *j));
			return true
		},
		Dir::Right => {
			if *j == w-1 { return false }
			
			if grid[*i][*j+1] == '#' {
				*dir = Dir::Down;
				return true;
			}

			*j += 1;
			set.insert((*i, *j));
			return true
		},
		Dir::Down => {
			if *i == h-1 { return false }
			
			if grid[*i+1][*j] == '#' {
				*dir = Dir::Left;
				return true;
			}

			*i += 1;
			set.insert((*i, *j));
			return true
		},
		Dir::Left => {
			if *j == 0 { return false }

			// println!("I: {}, J: {}", i, j);
			
			if grid[*i][*j-1] == '#' {
				*dir = Dir::Up;
				return true;
			}

			*j -= 1;
			set.insert((*i, *j));
			return true
		}
	}
}


fn move_and_check_loop(h: usize, w: usize, i: &mut usize, j: &mut usize, grid: &Vec<Vec<char>>, dir: &mut Dir, map: &mut Vec<Vec<i32>>, loops: &mut usize) -> bool {
	if map[*i][*j] & dir.get_bits() != 0 {
		*loops += 1;
		return false
	}

	map[*i][*j] += dir.get_bits();

	match dir {
		Dir::Up => {
			if *i == 0 { return false }
			
			if grid[*i-1][*j] == '#' {
				*dir = Dir::Right;
				return true;
			}

			*i -= 1;
			return true
		},
		Dir::Right => {
			if *j == w-1 { return false }
			
			if grid[*i][*j+1] == '#' {
				*dir = Dir::Down;
				return true;
			}

			*j += 1;
			return true
		},
		Dir::Down => {
			if *i == h-1 { return false }
			
			if grid[*i+1][*j] == '#' {
				*dir = Dir::Left;
				return true;
			}

			*i += 1;
			return true
		},
		Dir::Left => {
			if *j == 0 { return false }

			// println!("I: {}, J: {}", i, j);
			
			if grid[*i][*j-1] == '#' {
				*dir = Dir::Up;
				return true;
			}

			*j -= 1;
			return true
		}
	}
}

#[derive(Clone, PartialEq)]
pub enum Dir {
	Up,
	Left,
	Right,
	Down
}

impl Dir {
	pub fn get_bits(&self) -> i32 {
		match self {
			Self::Up => 1,
			Self::Right => 2,
			Self::Down => 4,
			Self::Left => 8
		}
	}
}