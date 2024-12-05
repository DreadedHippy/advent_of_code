
pub fn day_4_first(input: String) {
	let grid = input.lines().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
	

	let w = grid[0].len();
	let h = grid.len();
	let mut count = 0;

	for i in 0..h {
			for j in 0..w {
					if grid[i][j] == 'X' {
							search_xmas(&grid, (i, j), &mut count, h, w);
					}
			}
	}

	println!("{}", count);
}

fn search_xmas(grid: &Vec<Vec<char>>, position: (usize, usize), count: &mut usize, h: usize, w: usize) {
	let (i, j) = position;

	if i + 3 < h && (grid[i][j], grid[i+1][j], grid[i+2][j], grid[i+3][j]) == ('X', 'M', 'A', 'S') {
			*count += 1;
	}
	
	if i >= 3 && (grid[i][j], grid[i-1][j], grid[i-2][j], grid[i-3][j]) == ('X', 'M', 'A', 'S') {
			*count += 1;
	}

	if j + 3 < w && (grid[i][j], grid[i][j+1], grid[i][j+2], grid[i][j+3]) == ('X', 'M', 'A', 'S') {
			*count += 1;
	}

	if j >= 3 && (grid[i][j], grid[i][j-1], grid[i][j-2], grid[i][j-3]) == ('X', 'M', 'A', 'S') {
			*count += 1;
	}

	if i+3 < h && j + 3 < w && (grid[i][j], grid[i+1][j+1], grid[i+2][j+2], grid[i+3][j+3]) == ('X', 'M', 'A', 'S') {
			*count += 1;
	}

	if i+3 < h && j >= 3 && (grid[i][j], grid[i+1][j-1], grid[i+2][j-2], grid[i+3][j-3]) == ('X', 'M', 'A', 'S') {
			*count += 1;
	}

	if i>= 3 && j + 3 < w && (grid[i][j], grid[i-1][j+1], grid[i-2][j+2], grid[i-3][j+3]) == ('X', 'M', 'A', 'S') {
			*count += 1;
	}

	if i>= 3 && j >= 3 && (grid[i][j], grid[i-1][j-1], grid[i-2][j-2], grid[i-3][j-3]) == ('X', 'M', 'A', 'S') {
			*count += 1
	}
}

pub fn day_4_second(input: String) {
	let grid = input.lines().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
	

	let w = grid[0].len();
	let h = grid.len();
	let mut count = 0;

	for i in 0..h {
			for j in 0..w {
					if grid[i][j] == 'A' {
							search_x_mas(&grid, (i, j), &mut count, h, w);
					}
			}
	}

	println!("{}", count);
}

fn search_x_mas(grid: &Vec<Vec<char>>, position: (usize, usize), count: &mut usize, h: usize, w: usize) {
	let (i, j) = position;
	if i > 0 && j > 0 && i+1 < h && j + 1 < w {
			let a = (grid[i-1][j+1], grid[i+1][j-1]);
			let b  = (grid[i+1][j+1], grid[i-1][j-1]);



			if (a == ('M', 'S') || a == ('S', 'M')) && (b == ('M', 'S') || b == ('S', 'M')) {
					*count += 1;
			}
	}
}