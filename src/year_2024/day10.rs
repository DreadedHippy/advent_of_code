use std::collections::HashSet;

pub fn day_10_first(input: String) {
	let grid =  input.lines().map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

	let h = grid.len();
	let w = grid[0].len();
	let mut count = 0;

	for i in 0..h {
		for j in 0..w {
			if grid[i][j] == 0 {
				let mut visited = vec![vec![false; w]; h];
				get_trail(i, j, h, w, &grid, &mut visited, &mut count);
			}
		}
	}

	println!("{}", count);
}

pub fn day_10_second(input: String) {
	let grid =  input.lines().map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

	let h = grid.len();
	let w = grid[0].len();
	let mut count = 0;

	for i in 0..h {
		for j in 0..w {
			if grid[i][j] == 0 {
				get_trail_rating(i, j, h, w, &grid, &mut count);
			}
		}
	}

	println!("{}", count);
}

fn get_trail(i: usize, j: usize, h: usize, w: usize, grid: &Vec<Vec<u32>>, visited: &mut Vec<Vec<bool>>, count: &mut usize) {
	if visited[i][j] {return}

	visited[i][j] = true;

	let v = grid[i][j] + 1;
	
	if v == 10 {
		*count += 1;
		return
	}
		

	if j < w-1 && grid[i][j+1] == v && visited[i][j+1] == false {
		get_trail(i, j+1, h, w, grid, visited, count);
	}

	if i < h-1 && grid[i+1][j] == v && visited[i+1][j] == false {
		get_trail(i+1, j, h, w, grid, visited, count);
	}

	if j > 0 && grid[i][j-1] == v && visited[i][j-1] == false {
		get_trail(i, j-1, h, w, grid, visited, count);
	}

	if i > 0 && grid[i-1][j] == v && visited[i-1][j] == false {
		get_trail(i-1, j, h, w, grid, visited, count);
	}


}

fn get_trail_rating(i: usize, j: usize, h: usize, w: usize, grid: &Vec<Vec<u32>>, count: &mut usize) {

	let v = grid[i][j] + 1;
	
	if v == 10 {
		*count += 1;
		return
	}
		

	if j < w-1 && grid[i][j+1] == v {
		get_trail_rating(i, j+1, h, w, grid, count);
	}

	if i < h-1 && grid[i+1][j] == v {
		get_trail_rating(i+1, j, h, w, grid,  count);
	}

	if j > 0 && grid[i][j-1] == v {
		get_trail_rating(i, j-1, h, w, grid,  count);
	}

	if i > 0 && grid[i-1][j] == v {
		get_trail_rating(i-1, j, h, w, grid,  count);
	}


}