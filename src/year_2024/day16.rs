use std::{cmp::Reverse, collections::{BinaryHeap, HashSet, VecDeque}, ops::Add, usize};

pub fn day_16_first(input: String) {
	let mut lines = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();	

	let (mut si, mut sj) = (0, 0);
	let (mut ei, mut ej) = (0, 0);

	let h = lines.len();
	let w = lines[0].len();

	for i in 0..h {
		for j in 0..w {
			match lines[i][j] {
				'E' => {(ei, ej) = (i, j)},
				'S' => {(si, sj) = (i, j)}
				_ => {}
			}
		}
	}

	println!("{} to  {}", lines[si][sj], lines[ei][ej]);

	let c = search_first((si, sj), (ei, ej), h, w, &mut lines);

	println!("{}", c);


}

pub fn search_first(s: (usize, usize), e: (usize, usize), h: usize, w: usize, grid: &mut Vec<Vec<char>>) -> usize {
	let mut visited = vec![vec![usize::MAX; w]; h];

	let mut q = BinaryHeap::new();
	
	let mut dir = Dir::East;
	let mut cost = 0;

	q.push((Reverse(cost), s, dir));


	while !q.is_empty() {
		let (Reverse(c), t, d) = q.pop().unwrap();
		if visited[t.0][t.1] < c {continue}

		visited[t.0][t.1] = c;

		if t.0 > 0 && grid[t.0 - 1][t.1] != '#' && c < visited[t.0 - 1][t.1]{
			let cost = c + 1 + (1000*d.get_rotational_difference(Dir::North));
			q.push((Reverse(cost), (t.0-1, t.1), Dir::North));
			grid[t.0-1][t.1] = '^';
		}

		if t.1 > 0 && grid[t.0][t.1 - 1] != '#' && c < visited[t.0][t.1-1]{
			let cost = c + 1 + (1000*d.get_rotational_difference(Dir::West));
			q.push((Reverse(cost), (t.0, t.1-1), Dir::West));
			grid[t.0][t.1-1] = '<';
		}

		if t.0 < h-1 && grid[t.0 + 1][t.1] != '#' && c < visited[t.0 + 1][t.1]{
			let cost = c + 1 + (1000*d.get_rotational_difference(Dir::South));
			q.push((Reverse(cost), (t.0+1, t.1), Dir::South));
			grid[t.0+1][t.1] = 'v';
		}

		if t.1 < w-1 && grid[t.0][t.1+1] != '#'  && c < visited[t.0][t.1 + 1]{
			let cost = c + 1 + (1000*d.get_rotational_difference(Dir::East));
			q.push((Reverse(cost), (t.0, t.1+1), Dir::East));
			grid[t.0][t.1+1] = '>';
		}
	}

	for r in &visited {
		println!("{:?}", r.iter().map(|&x| match x {usize::MAX => -1, k => k as i32}).collect::<Vec<_>>());
	}

	visited[e.0][e.1]

}

pub fn day_16_second(input: String) {
	let mut lines = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();	

	let (mut si, mut sj) = (0, 0);
	let (mut ei, mut ej) = (0, 0);

	let h = lines.len();
	let w = lines[0].len();

	for i in 0..h {
		for j in 0..w {
			match lines[i][j] {
				'E' => {(ei, ej) = (i, j)},
				'S' => {(si, sj) = (i, j)}
				_ => {}
			}
		}
	}

	println!("{} to  {}", lines[si][sj], lines[ei][ej]);

	let c = search_second((si, sj), (ei, ej), h, w, &mut lines);

	println!("{}", c);
	// let c = get_tile_count(&lines, s, e, min_cost);


}


pub fn search_second(s: (usize, usize), e: (usize, usize), h: usize, w: usize, grid: &mut Vec<Vec<char>>) -> usize {
	let mut visited = vec![vec![(usize::MAX, -1); w]; h];

	let mut q = BinaryHeap::new();
	
	let mut dir = Dir::East;
	let mut cost = 0;

	q.push((Reverse(cost), s, dir, 1));


	while let Some((Reverse(c), t, d, tc))  = q.pop() {
		if visited[t.0][t.1].0 < c {continue}

		visited[t.0][t.1] = (c, tc);

		if t.0 > 0 && grid[t.0 - 1][t.1] != '#' && c < visited[t.0 - 1][t.1].0 {
			let cost = c + 1 + (1000*d.get_rotational_difference(Dir::North));
			q.push((Reverse(cost), (t.0-1, t.1), Dir::North, tc + 1));
		}

		if t.1 > 0 && grid[t.0][t.1 - 1] != '#' && c < visited[t.0][t.1-1].0 {
			let cost = c + 1 + (1000*d.get_rotational_difference(Dir::West));
			q.push((Reverse(cost), (t.0, t.1-1), Dir::West, tc + 1));
		}

		if t.0 < h-1 && grid[t.0 + 1][t.1] != '#' && c < visited[t.0 + 1][t.1].0 {
			let cost = c + 1 + (1000*d.get_rotational_difference(Dir::South));
			q.push((Reverse(cost), (t.0+1, t.1), Dir::South, tc + 1));
		}

		if t.1 < w-1 && grid[t.0][t.1+1] != '#' && c < visited[t.0][t.1 + 1].0 {
			let cost = c + 1 + (1000*d.get_rotational_difference(Dir::East));
			q.push((Reverse(cost), (t.0, t.1+1), Dir::East, tc + 1));
		}
	}

	// for r in &visited {
	// 	println!("{:?}", r.iter().map(|&x| format!("{:>2}", x.0)).collect::<Vec<_>>());
	// }

	get_tile_count(&grid, s, e, visited[e.0][e.1].0);
	visited[e.0][e.1].0


}



fn get_tile_count(grid: &Vec<Vec<char>>, s: (usize, usize), e: (usize, usize), min_cost: usize) {
	let (h, w) = (grid.len(), grid[0].len());
	
	let mut to_visit = vec![vec![[min_cost, min_cost]; w]; h];
	let mut tiles = HashSet::new();

	let mut pq = BinaryHeap::new();

	to_visit[s.0][s.1][Dir::East.get_axis()] = 0;

	pq.push(Reverse((s, Dir::East, 0, Some(vec![]))));

	while let Some((Reverse((pos, direction, cost, history)))) = pq.pop() {
		let mut history = history.unwrap();
		history.push(pos);

		let (row, col) = pos;
		
		if cost > to_visit[row][col][direction.get_axis()] || cost > min_cost {
			continue;
		}

		if pos == e {
			if cost == min_cost {
				tiles.extend(history);
			}
			
			continue;
		}



		for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
			let mut next_dir = dir;
			let (next_row, next_col) = pos + next_dir;

			let mut next_cost = cost;
			next_cost += (next_dir.get_rotational_difference(direction) * 1000) + 1; 

			if grid[next_row][next_col] == '#' {continue;}

			if next_cost <= to_visit[next_row][next_col][direction.get_axis()] {
				to_visit[next_row][next_col][direction.get_axis()] = next_cost;

				pq.push(Reverse(((next_row, next_col), next_dir, next_cost, Some(history.clone()))));

			}
		}



	}

	println!("Tiles len: {}", tiles.len());

	// for r in &to_visit {
	// 	println!("{:?}", r.iter().map(|x| format!("{:>4} {:>4}", x[0], x[1])).collect::<Vec<_>>());
	// }


	// let mut unique = HashSet::new();

	// while !q.is_empty() {
	// 	println!("{:?}", q);
	// 	let (steps, (i, j)) = q.pop().unwrap();
		
	// 	let ns = steps - 1;
	// 	if !unique.insert((i, j)) {continue}

	// 	if i > 0 && grid[i-1][j].1 == ns && grid[i-1][j].0 < grid[i][j].0 {
	// 		q.push((ns, (i-1, j)));
	// 	}

	// 	if j > 0 && grid[i][j-1].1 == ns && grid[i][j-1].0 < grid[i][j].0 {
	// 		q.push((ns, (i, j-1)));
	// 	}

	// 	if i < h-1 && grid[i+1][j].1 == ns && grid[i+1][j].0 < grid[i][j].0 {
	// 		q.push((ns, (i+1, j)));
	// 	}

	// 	if j < w-1 && grid[i][j+1].1 == ns && grid[i][j+1].0 < grid[i][j].0{
	// 		q.push((ns, (i, j+1)));
	// 	}
	// }

	// println!("COUNT: {}", unique.len());
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
	North,
	East,
	West,
	South
}

impl Add<Dir> for (usize, usize) {
	type Output = (usize, usize);

	fn add(self, rhs: Dir) -> Self::Output {
		let (row, col) = self;
		match rhs {
			Dir::North => (row-1, col),
			Dir::South => (row+1, col),
			Dir::East => (row, col+1),
			Dir::West => (row, col-1),
		}
	}
}

impl PartialOrd for Dir {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for Dir {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      std::cmp::Ordering::Equal
    }
}

impl Dir {
	fn get_rotational_difference(&self, mut other: Dir) -> usize {
		let mut d = 0;

		while *self != other {
			other.anti_clockwise_rotate();
			d += 1;
		}

		if d == 3 {d = 1}

		d
	}

	fn anti_clockwise_rotate(&mut self) {
		match self {
			Dir::North => *self = Dir::West,
			Dir::East => *self = Dir::North,
			Dir::West => *self = Dir::South,
			Dir::South => *self = Dir::East,
		}
	}

	fn get_axis(&self) -> usize {
		match self {
			Dir::North | Dir::South => 0,
			Dir::East | Dir::West => 1,
		}
	}
}