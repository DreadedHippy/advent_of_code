use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

pub fn day_20_first(input: String) {
	let mut grid = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut s = (0, 0);
	let mut e = (0, 0);

	let h = grid.len();
	let w = grid[0].len();

	for i in 0..h {
		for j in 0..w {
			match grid[i][j] {
				'S' => s = (i, j),
				'E' => e = (i, j),
				_ => {}
			}
		}
	}

	// println!("{:?}", grid);
	// println!("S: {:?}, E: {:?}", s, e);

	let mut count= 0;

	let visited = get_time(&grid, s, e);

	println!("TOTAL TIME: {:?}", visited[e.0][e.1]);
	let max = visited[e.0][e.1];
	println!("START {}", visited[s.0][s.1]);


	let mut c = 0;
	for i in 100..=max {
		c += check_shortcut(&visited, s, e, max, i);
	}

	println!("{c}");


}

pub fn day_20_second(input: String) {
	let mut grid = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut s = (0, 0);
	let mut e = (0, 0);

	let h = grid.len();
	let w = grid[0].len();

	for i in 0..h {
		for j in 0..w {
			match grid[i][j] {
				'S' => s = (i, j),
				'E' => e = (i, j),
				_ => {}
			}
		}
	}

	// println!("{:?}", grid);
	// println!("S: {:?}, E: {:?}", s, e);

	let mut count= 0;

	let visited = get_time(&grid, s, e);

	println!("TOTAL TIME: {:?}", visited[e.0][e.1]);
	let max = visited[e.0][e.1];
	println!("START {}", visited[s.0][s.1]);

	let mut icm = vec![(0, 0); max + 1];
	for i in 0..h {
		for j in 0..w {
			if visited[i][j] <= max {
				icm[visited[i][j]] = (i, j);
			}
		}
	}

	let mut count = 0;
	let max_pico = 20;
	let save = 50;

	let mut counted = vec![vec![false; icm.len()]; icm.len()];

	let mut saved = vec![0; max];

	// for s in 50..=max {
	// 	println!("ITER: {}", s);
		for i in 0..(max + 1) {
			for j in (i+1)..(max+1) {
				if counted[i][j] {continue;}
				
				let d = manhattan_distance(icm[i], icm[j]);
				if d <= max_pico && j >= (i + d) {
					saved[j - (i + d)] += 1;
				}
			}
		}
	// }

	println!("{:?}", saved.iter().skip(100).sum::<usize>());


	// println!("{saved:?}");

}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
	(((b.1 as i32) - (a.1 as i32)).abs() + ((b.0 as i32) - (a.0 as i32)).abs()) as usize
}
pub fn get_time(grid: &Vec<Vec<char>>, s: (usize, usize), e: (usize, usize)) -> Vec<Vec<usize>> {
	let mut q = BinaryHeap::new();

	let (h, w)= (grid.len(), grid[0].len());
	q.push(Reverse((0, s)));

	let mut visited = vec![vec![usize::MAX; w]; h];

	while let Some(Reverse((time, coord))) = q.pop() {
		let (i, j) = coord;

		if visited[i][j] <= time {continue}
		visited[i][j] = time;
		let time = time + 1;

		if i > 0 && visited[i-1][j] > time && grid[i-1][j] != '#' {
			q.push(Reverse((time, (i-1, j))));
		} else if j > 0 && visited[i][j-1] > time && grid[i][j-1] != '#' {
			q.push(Reverse((time, (i, j-1))));
		} else if i < h-1 && visited[i+1][j] > time && grid[i+1][j] != '#' {
			q.push(Reverse((time, (i+1, j))));
		} else if j < w-1 && visited[i][j+1] > time && grid[i][j+1] != '#' {
			q.push(Reverse((time, (i, j+1))));
		}
	}

	visited

}

pub fn check_shortcut(visited: &Vec<Vec<usize>>, s: (usize, usize), e: (usize, usize), max: usize, save: usize) -> usize {
	let mut count = 0;
	
	let mut q = BinaryHeap::new();

	let (h, w)= (visited.len(), visited[0].len());
	q.push(Reverse((0, s)));

	while let Some(Reverse((time, coord))) = q.pop() {
		// println!("Q");
		let (i, j) = coord;

		// if time >= save {
			if i >= 2 && visited[i-2][j] == time + 2 + save {
				count += 1;
			}

			if j >= 2 && visited[i][j-2] == time + 2 + save {
				count += 1
			}

			if i >= 1 && j >= 1 && visited[i-1][j-1] == time + 2 + save {
				count += 1;
			}

			if i < h-1 && j < w-1 && visited[i+1][j+1] == time + 2 + save  {
				count += 1;
			}

			if i < h-2 && visited[i+2][j] == time + 2 + save {
				count += 1;
			}

			if j < w-2 && visited[i][j+2] == time + 2 + save {
				count += 1;
			}

			if i < h-1 && j > 0 && visited[i+1][j-1] == time + 2 + save {
				count += 1;
			}

			if j < w-1 && i > 0 && visited[i-1][j+1] == time + 2 + save {
				count += 1;
			}
		// }

		let time = time + 1;

		// println!("UP: {}, L: {}, R: {}, D: {}", visited[i-1][j], visited[i][j-1], visited[i][j+1], visited[i+1][j]);

		if i > 0 && visited[i-1][j] == time {
			q.push(Reverse((time, (i-1, j))));
		} else if j > 0 && visited[i][j-1] == time {
			q.push(Reverse((time, (i, j-1))));
		} else if i < h-1 && visited[i+1][j] == time {
			q.push(Reverse((time, (i+1, j))));
		} else if j < w-1 && visited[i][j+1] == time {
			q.push(Reverse((time, (i, j+1))));
		}
	}

	// println!("COUNT {:?}", count)
	return count


}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
	time: usize,
	coord: (usize, usize)
}

impl Tile {
	fn  new(time: usize, coord: (usize, usize)) -> Self {
		Self {time, coord}
	}
}



// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// enum Dir {
// 	North,
// 	East,
// 	West,
// 	South
// }

// impl Add<Dir> for (usize, usize) {
// 	type Output = (usize, usize);

// 	fn add(self, rhs: Dir) -> Self::Output {
// 		let (row, col) = self;
// 		match rhs {
// 			Dir::North => (row-1, col),
// 			Dir::South => (row+1, col),
// 			Dir::East => (row, col+1),
// 			Dir::West => (row, col-1),
// 		}
// 	}
// }