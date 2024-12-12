use std::{collections::HashMap, fmt::format};

pub fn day_11_first(input: String) {
	let lines = input.lines().map(|x| x.split_whitespace().map(String::from).collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut l = lines[0].clone();
	let mut memo = HashMap::new();

	let mut sum: u128 = 0;

	for n in l {
		sum += transform(&"0".to_string(), &n, 0, 25, &mut memo) as u128;
	}

	println!("{:?}", sum);
}

pub fn day_11_second(input: String) {
	let lines = input.lines().map(|x| x.split_whitespace().map(String::from).collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut l = lines[0].clone();
	let mut memo = HashMap::new();

	let mut sum: u128 = 0;

	for n in l {
		sum += transform(&"0".to_string(), &n, 0, 75, &mut memo) as u128;
	}

	println!("{:?}", sum);
}

fn transform(o: &String, curr: &String, od: usize, limit: usize, memo: &mut HashMap<(String, usize), usize>) -> usize {
	// println!("{:?} {} {}",  curr, od, limit);
	if od >= limit {
		return 1;
	}

	if let Some(&v) = memo.get(&(curr.to_string(), limit-od)) {
		return v
	}

	let mut ov = 0;
	let mut nv = 0;

	if curr == "0" {
		let next = "1".to_string();
		// ov += transform(o, &next,od + 1, limit, memo); // overall
		nv += transform(&next, &next, od + 1, limit, memo); // overall
	} else if curr.len() & 1 == 0 {
		let l = curr[0..curr.len()/2].to_string();
		let mut r = curr[(curr.len()/2)..].chars().skip_while(|&x| x == '0').collect::<String>();

		if r.is_empty() {
			r = "0".to_string()
		}

		for next in [l, r] {
			// ov += transform(o,&next, od + 1, limit, memo); // overall
			nv += transform(&next,&next, od + 1, limit, memo) // overall
		}

	} else {
		let num = curr.parse::<u64>().unwrap();
		let next = format!("{}", num * 2024);
		// ov += transform(o, &next, od + 1, limit, memo); // overall
		nv += transform(&next, &next, od + 1, limit, memo); // overall
	}

	// memo.insert((o.clone(), od + 1), ov);
	memo.insert((curr.clone(), limit - od), nv);
	return nv
}