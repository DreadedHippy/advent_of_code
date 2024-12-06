use std::collections::HashMap;

pub fn day_1_first(input: String) {
	let lines = input.lines()
		.map(|x| x.split_whitespace().map(|n: &str| n.parse::<i32>().unwrap()).collect::<Vec<_>>()
	).collect::<Vec<_>>();

	let mut a = Vec::with_capacity(lines.len());
	let mut b = Vec::with_capacity(lines.len());

	for l in lines{
		a.push(l[0]);
		b.push(l[1]);
	}

	a.sort_unstable();
	b.sort_unstable();

	println!("{}", a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs() as usize).sum::<usize>() );

}

pub fn day_1_second(input: String) {
	
	let lines = input.lines()
		.map(|x| x.split_whitespace().map(|n: &str| n.parse::<i32>().unwrap()).collect::<Vec<_>>()
	).collect::<Vec<_>>();

	let mut a_map = HashMap::new();

	let mut b_map = HashMap::new();

	let mut max = 0;

	for l in lines {
		max = max.max(l[0]).max(l[1]);
		*a_map.entry(l[0]).or_insert(0) += 1;
		*b_map.entry(l[1]).or_insert(0) += 1
	}

	let mut sum: u64 = 0;

	for i in 1..=max {
		sum += (i * a_map.get(&i).unwrap_or(&0) *  b_map.get(&i).unwrap_or(&0)) as u64;
	}

	println!("{}", sum);

}