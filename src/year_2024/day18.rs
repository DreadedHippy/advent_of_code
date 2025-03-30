use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, fmt::format};

pub fn day_18_first(input: String) {
	let lines = input.lines().collect::<Vec<_>>();

	let (h, w) = (7, 7);

	let mut grid = vec![vec!['.'; w]; h];

	for line in lines.iter().take(12) {
		let mut s = line.split(",");

		let j = s.next().unwrap().parse::<usize>().unwrap();
		let i = s.next().unwrap().parse::<usize>().unwrap();

		grid[i][j] = '#'
	}

	shortest_path(&grid);

	// for r in &grid {
	// 	println!("{:?}", r);
	// }


}

pub fn shortest_path(grid: &Vec<Vec<char>>) {
	let s = (0, 0);
	let mut steps = 0;

	let mut q = BinaryHeap::new();

	let h = grid.len();
	let w = grid[0].len();
	let mut visited = vec![vec![i32::MAX; w]; h];

	q.push((Reverse(0), 0, 0));

	while !q.is_empty() {
		let (Reverse(steps), i, j) = q.pop().unwrap();

		if visited[i][j] <= steps {continue;}

		visited[i][j] = steps;

		let next = steps + 1;

		if i > 0 && grid[i-1][j] != '#' && next < visited[i-1][j] {
			q.push((Reverse(next), i-1, j));
		}

		if j > 0 && grid[i][j-1] != '#' && next < visited[i][j-1] {
			q.push((Reverse(next), i, j-1));
		}

		if i < h-1 && grid[i+1][j] != '#' && next < visited[i+1][j] {
			q.push((Reverse(next), i+1, j));
		}

		if j < w-1 && grid[i][j+1] != '#' && next < visited[i][j+1] {
			q.push((Reverse(next), i, j+1));
		}
	}

	println!("{:?}", visited[6][6]);
}


pub fn day_18_second(input: String) {
	let lines = input.lines().collect::<Vec<_>>();

	let (h, w) = (71, 71);

	let mut grid = vec![vec!['.'; w]; h];
	let mut lines = lines.into_iter();

	for _ in 0..1024 {
		let line = lines.next().unwrap();
		let mut s = line.split(",");

		let j = s.next().unwrap().parse::<usize>().unwrap();
		let i = s.next().unwrap().parse::<usize>().unwrap();

		grid[i][j] = '#'
	}

	let mut res = shortest_path2(&grid);
	if res == i32::MAX {println!("STOPPED")}
	// let (mut connections, mut all_children) = build_graph(&parents);

	// let mut visited = HashSet::new();
	// let mut count = 1024;
	// let reach = dfs_can_reach(&connections, (0, 0), (70, 70), &mut visited);

	// println!("Can reach? {}", reach);
	// let (mut bx, mut by) = (0, 0);


	while let Some(line) = lines.next() {
		let mut s = line.split(",");

		let j = s.next().unwrap().parse::<usize>().unwrap();
		let i = s.next().unwrap().parse::<usize>().unwrap();

		grid[i][j] = '#';

		let res = shortest_path2(&grid);
		if res == i32::MAX {
			println!("DONE: ");
			println!("{} {}", j, i);
			break;
		}

		// if all_children.remove(&(i, j)) {
		// 	bx = i;
		// 	by = j;

		// 	for p in &parents[i][j] {
		// 		if let Some(x) = connections.get_mut(p) {
		// 			x.remove(&(i, j));
		// 		}
		// 	}

		// 	let reach = dfs_can_reach(&connections, (0, 0), (70, 70), &mut HashSet::new());

		// 	if !reach {break}
		// }

	}

	// println!("{:?}", count);
	// println!("{:?}", (bx, by));
}

pub fn shortest_path2(grid: &Vec<Vec<char>>) -> i32 {
	let s = (0, 0);
	let mut steps = 0;

	let mut q = BinaryHeap::new();

	let h = grid.len();
	let w = grid[0].len();
	let mut visited = vec![vec![i32::MAX; w]; h];

	q.push((Reverse(0), 0, 0, (80, 80)));

	let mut parents = vec![vec![HashSet::new(); w]; h];

	while !q.is_empty() {
		let (Reverse(steps), i, j, (px, py)) = q.pop().unwrap();

		if visited[i][j] < steps {continue;}

		if visited[i][j] == steps {parents[i][j].insert((px, py)); continue;}


		visited[i][j] = steps;
		parents[i][j] = HashSet::from([(px, py)]);

		let next = steps + 1;

		if i > 0 && grid[i-1][j] != '#' && next < visited[i-1][j] {
			q.push((Reverse(next), i-1, j, (i, j)));
		}

		if j > 0 && grid[i][j-1] != '#' && next < visited[i][j-1] {
			q.push((Reverse(next), i, j-1, (i, j)));
		}

		if i < h-1 && grid[i+1][j] != '#' && next < visited[i+1][j] {
			q.push((Reverse(next), i+1, j, (i, j)));
		}

		if j < w-1 && grid[i][j+1] != '#' && next < visited[i][j+1] {
			q.push((Reverse(next), i, j+1, (i, j)));
		}
	}

	// println!("{:?}", visited[70][70]);
	// return parents;
	visited[70][70]
}

pub fn build_graph(parents: &Vec<Vec<HashSet<(usize, usize)>>>) -> (HashMap<(usize, usize), HashSet<(usize, usize)>>, HashSet<(usize, usize)>) {
	let mut connections = HashMap::new();

	connections.insert((0, 0), HashSet::new());
	let mut all_children = HashSet::new();

	for i in 0..parents.len() {
		for j in 0..parents[i].len() {
			for p in &parents[i][j] {
				connections.entry(*p).or_insert(HashSet::new()).insert((i, j));
				all_children.insert((i, j));
			}
		}
	}

	connections.remove(&(usize::MAX, usize::MAX));
	(connections, all_children)
}

pub fn dfs_can_reach(connections: &HashMap<(usize, usize), HashSet<(usize, usize)>>, node: (usize, usize), dest: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> bool {
	if !visited.insert(node) {return false}

	if let Some(c) = connections.get(&node) {
		if c.contains(&dest) {return true}

		for child in c {
			if dfs_can_reach(connections, *child, dest, visited) {
				return true
			}
		}
	}

	return false
}