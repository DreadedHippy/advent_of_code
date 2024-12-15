use std::{collections::VecDeque, fs, io::Write};

use text_to_png::TextRenderer;

pub fn day_14_first(input: String) {
	let mut lines = input.lines().map(|x| x.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
	let mut robots = Vec::new();

	for line in lines {
		let p = line[0].chars().skip(2).collect::<String>();
		let p = p.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		let (px, py) = (p[0], p[1]);

		
		let v = line[1].chars().skip(2).collect::<String>();
		let v = v.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		let (vx, vy) = (v[0], v[1]);

		let robot = Robo {p: (px, py), v: (vx, vy)};

		robots.push(robot);
	}

	let mut q = [[0, 0], [0, 0]];

	let h = 103;
	let w = 101;

	let mut grid = vec![vec![0 ; w as usize]; h as usize];

	for mut r in robots {
		r.nav(100, h, w);

		let (x, y) = r.pos();

		grid[y as usize][x as usize] += 1;

		let mut t: i32 = -1;
		let mut l: i32 = -1;

		if x < (w-1)/2 {
			t = 0
		} else if x > (w-1)/2 {
			t = 1;
		}

		if y < (h-1)/2 {
			l = 0
		} else if y > (h-1)/2 {
			l = 1;
		}

		if t >= 0 && l >= 0 {
			q[t as usize][l as usize] += 1;
		}

	}

	
	// for r in grid {
	// 	println!("{:?}", r);
	// }
	println!("{:?}", q[0][0] * q[0][1] * q[1][0] * q[1][1]);

}

#[derive(Debug, Clone)]
pub struct Robo {
	pub p: (i32, i32),
	pub v: (i32, i32),
}

impl Robo {
	pub fn nav(&mut self, steps: i32, h: i32, w: i32) {
		let mut n_x = self.p.0 + (steps * self.v.0);
		let mut n_y = self.p.1 + (steps * self.v.1);


		if n_x >= 0 {
			n_x %= w;
		} else {
			let a = n_x.abs();
			let v = ((a/w) + 1) * w;
			n_x = (v + n_x) % w;
		}

		if n_y > 0 {
			n_y %= h;
		} else {
			let a = n_y.abs();
			let v = ((a/h) + 1) * h;
			n_y = (v + n_y) % h;
		}


		self.p = (n_x, n_y);
	}

	pub fn pos(&self) -> (i32, i32) {
		self.p
	}
}

pub fn day_14_second(input: String) {
	let mut lines = input.lines().map(|x| x.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
	let mut robots = Vec::new();

	
	let h = 103;
	let w = 101;

	let mut grid = vec![vec![0 ; w as usize]; h as usize];


	for line in lines {
		let p = line[0].chars().skip(2).collect::<String>();
		let p = p.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		let (px, py) = (p[0], p[1]);

		
		let v = line[1].chars().skip(2).collect::<String>();
		let v = v.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		let (vx, vy) = (v[0], v[1]);

		let robot = Robo2 {p: (px, py), v: (vx, vy)};

		grid[py as usize][px as usize] += 1;

		robots.push(robot);
	}

	let mut q = [[0, 0], [0, 0]];

	let mut i = 0;

	loop {
		println!("I: {}", i);
		let mut temp = String::new();

		i += 1;

		for r in &mut robots {
			r.nav(1, h, w, &mut grid);

			let (x, y) = r.pos();
		}

		if flood_fill_max(&grid) >= 50 {
			let mut s = String::new();
			for r in grid.iter() {
				s.push('\n');
				let row = r.iter().map(|x| {
					match x {
						0 => ".".to_string(),
						k => k.to_string()
					}
				}).collect::<String>();
				s.push_str(&row);
			}

			print!("\x1B[2J\x1B[1;1H");
			println!("Iteration: {}", i);
			println!("{}", s);
			break
		}

	}
	println!("{:?}", q[0][0] * q[0][1] * q[1][0] * q[1][1]);

}

#[derive(Debug, Clone)]
pub struct Robo2 {
	pub p: (i32, i32),
	pub v: (i32, i32),
}

impl Robo2 {
	pub fn nav(&mut self, steps: i32, h: i32, w: i32, grid: &mut Vec<Vec<i32>>) {
		let (x, y) = self.pos();
		grid[y as usize][x as usize] -= 1;

		let mut n_x = self.p.0 + (steps * self.v.0);
		let mut n_y = self.p.1 + (steps * self.v.1);



		if n_x >= 0 {
			n_x %= w;
		} else {
			let a = n_x.abs();
			let v = ((a/w) + 1) * w;
			n_x = (v + n_x) % w;
		}

		if n_y > 0 {
			n_y %= h;
		} else {
			let a = n_y.abs();
			let v = ((a/h) + 1) * h;
			n_y = (v + n_y) % h;
		}

		grid[n_y as usize][n_x as usize] += 1;
		self.p = (n_x, n_y);
	}

	pub fn pos(&self) -> (i32, i32) {
		self.p
	}
}

fn flood_fill_max(grid: &Vec<Vec<i32>>) -> usize {
	let h = grid.len();
	let w = grid[0].len();

	let mut visited = vec![vec![false; w]; h];
	let mut max = 0;

	for i in 0..h {
		for j in 0..w {
			if grid[i][j] != 0 && !visited[i][j] {
				max = max.max(flood_fill(i, j, h, w, &mut visited, grid))
			}
		}
	}

	max
}

fn flood_fill(i: usize, j: usize, h: usize, w: usize, visited: &mut Vec<Vec<bool>>, grid: &Vec<Vec<i32>>) -> usize {
	let mut q = VecDeque::new();
	q.push_back((i, j));

	let mut connections = 0;

	while !q.is_empty() {
		let n = q.len();
		connections += n;

		for _ in 0..n {
			let (i, j) = q.pop_front().unwrap();

			if visited[i][j] {continue}

			visited[i][j] = true;

			if i > 0 && grid[i-1][j] > 0 && !visited[i][j] {
				q.push_back((i-1, j));
			}
			if j > 0 && grid[i][j-1] > 0 && !visited[i][j-1] {
				q.push_back((i, j-1));
			}
			if i < h-1 && grid[i+1][j] > 0 && !visited[i+1][j] {
				q.push_back((i+1, j));
			}
			if j < w-1 && grid[i][j+1] > 0 && !visited[i][j+1] {
				q.push_back((i, j+1));
			}
		}
	}

	return connections
}