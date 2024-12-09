use std::collections::{HashMap, HashSet};

pub fn day_8_first(input: String) {
	let lines = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut map = HashMap::new();

	let h = lines.len();
	let w = lines[0].len();

	for i in 0..h {
		for j in 0..w {
			match lines[i][j] {
				'.' => {},
				k => {
					map.entry(k).or_insert(Vec::new()).push((i as i32, j as i32));
				}
			}
		}
	}

	let h = h as i32;
	let w = w as i32;

	let mut ans = HashSet::new();

	for (k, mut v) in map {
		while let Some(c) = v.pop() {
			for o in &v {
				let a = (2 * o.0 - c.0, 2 *o.1 - c.1);
				let b = (2 * c.0 - o.0, 2 * c.1 - o.1);

				if a.0 >= 0 && a.0 < h && a.1 >= 0 && a.1 < w {
					ans.insert((a.0, a.1));
				}

				if b.0 >= 0 && b.0 < h && b.1 >= 0 && b.1 < w {
					ans.insert((b.0, b.1));
				}
			}
		}
	}

	println!("{}", ans.len())
}

pub fn day_8_second(input: String) {
	let mut lines = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut map = HashMap::new();

	let h = lines.len();
	let w = lines[0].len();

	for i in 0..h {
		for j in 0..w {
			match lines[i][j] {
				'.' => {},
				k => {
					map.entry(k).or_insert(Vec::new()).push((i as i32, j as i32));
				}
			}
		}
	}

	let h = h as i32;
	let w = w as i32;

	let mut ans = HashSet::new();

	for (k, mut v) in map {
		while let Some(c) = v.pop() {
			for o in &v {
				ans.insert((o.0, o.1));
				ans.insert((c.0, c.1));

				let a = (o.0 - c.0, o.1 - c.1);
				let mut curr = (o.0 + a.0, o.1 + a.1);

				while curr.0 >= 0 && curr.0 < h && curr.1 >= 0 && curr.1 < w {
					lines[curr.0 as usize][curr.1 as usize] = '#';
					ans.insert((curr.0, curr.1));
					curr.0 += a.0;
					curr.1 += a.1;
				}

				let b = (c.0 - o.0, c.1 - o.1);
				let mut curr = (c.0 + b.0, c.1 + b.1);

				while curr.0 >= 0 && curr.0 < h && curr.1 >= 0 && curr.1 < w {
					ans.insert((curr.0, curr.1));
					curr.0 += b.0;
					curr.1 += b.1;
				}
			}
		}
	}

	println!("{}", ans.len());
}