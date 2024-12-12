use std::{collections::{BTreeSet, HashSet, VecDeque}, i32};

pub fn day_12_first(input: String) {
	let lines = input.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

	let mut visited = vec![vec![false; lines[0].len()]; lines.len()];

	let w = lines[0].len();
	let h = lines.len();

	let mut sum = 0;

	for i in 0..h {
		for j in 0..w {
			if !visited[i][j] {
				let s = get_stats(i, j, h, w, &mut visited, &lines, lines[i][j]);
				println!("C: {}, S: {:?}", lines[i][j], s);
				sum += s.0 * s.1;

			}
		}
	}

	println!("{:?}", sum)
}

pub fn day_12_second(input: String) {
	let lines = input.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

	let mut visited = vec![vec![false; lines[0].len()]; lines.len()];

	let w = lines[0].len();
	let h = lines.len();

	let mut sum = 0;

	for i in 0..h {
		for j in 0..w {
			if !visited[i][j] {
				let s = get_stats_discount(i, j, h, w, &mut visited, &lines, lines[i][j]);
				// println!("C: {}, S: {:?}", lines[i][j], s);
				sum += s.0 * s.1;

			}
		}
	}

	println!("{:?}", sum)
}

fn get_stats(i: usize, j: usize, h: usize, w: usize, visited: &mut Vec<Vec<bool>>, grid: &Vec<Vec<char>>, c: char) -> (usize, usize) {
	let mut q = VecDeque::new();
	let mut area = 0;
	let mut p = 0;

	q.push_back((i, j));

	while !q.is_empty() {
		let n = q.len();

		for _ in 0..n {
			let (a, b) = q.pop_front().unwrap();
			if visited[a][b] {continue}
			visited[a][b] = true;
			area += 1;

			if a > 0 && grid[a-1][b] == c {
				if !visited[a-1][b] {
					q.push_back((a-1, b));
				}
			} else {
				p += 1;
			}

			if b > 0 && grid[a][b-1] == c {
				if !visited[a][b-1] {
					q.push_back((a, b-1));
				}
			} else {
				p += 1;
			}

			if a < h-1 && grid[a+1][b] == c {
				if !visited[a+1][b] {
					q.push_back((a+1, b));
				}
			} else {
				p += 1;
			}

			if b < w-1 && grid[a][b+1] == c {
				if !visited[a][b+1] {
					q.push_back((a, b+1));
				}
			} else {
				p += 1;
			}
		}
	}

	return (p, area);

}

fn get_stats_discount(i: usize, j: usize, h: usize, w: usize, visited: &mut Vec<Vec<bool>>, grid: &Vec<Vec<char>>, c: char) -> (usize, usize) {
	let mut q = VecDeque::new();
	let mut area = 0;
	let mut p = BTreeSet::new();

	q.push_back((i, j));

	while !q.is_empty() {
		let n = q.len();

		for _ in 0..n {
			let (a, b) = q.pop_front().unwrap();
			if visited[a][b] {continue}
			visited[a][b] = true;
			area += 1;

			if a > 0 && grid[a-1][b] == c {
				if !visited[a-1][b] {
					q.push_back((a-1, b));
				}
			} else {
				p.insert(((a as i32-1) as i32, b as i32, 0));
			}

			if b > 0 && grid[a][b-1] == c {
				if !visited[a][b-1] {
					q.push_back((a, b-1));
				}
			} else {
				p.insert((a as i32, (b as i32 -1) as i32, 1));
			}

			if a < h-1 && grid[a+1][b] == c {
				if !visited[a+1][b] {
					q.push_back((a+1, b));
				}
			} else {
				p.insert(((a+1) as i32, b as i32, 2));
			}

			if b < w-1 && grid[a][b+1] == c {
				if !visited[a][b+1] {
					q.push_back((a, b+1));
				}
			} else {
				p.insert((a as i32, (b+1) as i32, 3));
			}
		}
	}


	let mut perim = 0;

	while let Some(c) = p.pop_first() {
		perim += 1;
		dfs_sides( c, &mut p);
	}


	return (perim, area);

}


pub fn dfs_sides(c: (i32, i32, i32), q: &mut BTreeSet<(i32, i32, i32)>) {
	q.remove(&c);
	if q.contains(&(c.0-1, c.1, c.2)) {
		dfs_sides((c.0-1, c.1, c.2), q);
	}
	if q.contains(&(c.0+1, c.1, c.2)) {
		dfs_sides((c.0+1, c.1, c.2), q);
	}
	if q.contains(&(c.0, c.1-1, c.2)) {
		dfs_sides((c.0, c.1-1, c.2), q);
	}
	if q.contains(&(c.0, c.1+1, c.2)) {
		dfs_sides((c.0, c.1+1, c.2), q);
	}
}