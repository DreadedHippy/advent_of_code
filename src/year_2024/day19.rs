use std::{collections::{HashMap, HashSet}, fmt::format};

pub fn day_19_first(input: String) {
	let mut lines = input.lines().map(|x| x.to_string());

	let first = lines.next().unwrap();

	let mut map = HashMap::new();

	let mut available = first.split(",").map(|x| x.trim().to_string()).collect::<Vec<_>>();

	for word in available {
		let fc = word.chars().next().unwrap();
		map.entry(fc).or_insert(Vec::new()).push(word);
	}

	let mut count = 0;
	while let Some(l) = lines.next() {
		if l.is_empty() {continue;}
		println!("Checking word: {}", l);
		count += dfs_search(&map, String::new(), &l, &mut HashSet::new()) as usize;
	}

	println!("{:?}", count);
}


pub fn dfs_search(map: &HashMap<char, Vec<String>>, current: String, word: &String, visited: &mut HashSet<String>) -> bool {
	if !visited.insert(current.clone()) {
		return false
	}

	if current.len() > word.len() {
		return false
	}

	if !word.starts_with(&current) {
		return false;
	}

	if current == *word {
		return true
	}


	let next_checks = word.chars().skip(current.len()).next().unwrap();

	if let Some(c) = map.get(&next_checks) {
		for child in c {
			let new_current = format!("{}{}", current, child);
			if dfs_search(map, new_current, word, visited) {
				return true
			}
		}
	}

	return false;


}

pub fn day_19_second(input: String) {
	let mut lines = input.lines().map(|x| x.to_string());

	let first = lines.next().unwrap();

	let mut map = HashMap::new();

	let mut available = first.split(",").map(|x| x.trim().to_string()).collect::<Vec<_>>();

	for word in available {
		let fc = word.chars().next().unwrap();
		map.entry(fc).or_insert(Vec::new()).push(word);
	}

	let mut count = 0;
	while let Some(l) = lines.next() {
		if l.is_empty() {continue;}
		count += dfs_search2(&map, String::new(), &l, &mut HashSet::new(), &mut HashMap::new());
	}

	println!("{:?}", count);
}

pub fn dfs_search2(map: &HashMap<char, Vec<String>>, current: String, word: &String, visited: &mut HashSet<String>,score: &mut HashMap<String, usize>) -> usize {
	if current == *word {
		return 1
	}

	if let Some(s) = score.get(&current) {
		return *s;
	}

	if current.len() >= word.len() {
		return 0
	}

	if !word.starts_with(&current) {
		return 0
	}

	let mut count = 0;
	let next = word.chars().skip(current.len()).next().unwrap();

	if let Some(children) = map.get(&next) {
		for (i, c) in children.iter().enumerate() {
			let next_current = format!("{}{}", current, c);

			count += dfs_search2(map, next_current, word, visited, score);
		}
	}

	score.insert(current, count);
	return count;


}
