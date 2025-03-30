use std::{collections::HashMap, hash::Hash};

pub fn day_21_first(input: String) {
	let lines = input.lines().collect::<Vec<_>>();

	let mut kp1 = HashMap::new();

	kp1.insert('7', (0, 0));
	kp1.insert('8', (0, 1));
	kp1.insert('9', (0, 2));
	kp1.insert('4', (1, 0));
	kp1.insert('5', (1, 1));
	kp1.insert('6', (1, 2));
	kp1.insert('1', (2, 0));
	kp1.insert('2', (2, 1));
	kp1.insert('3', (2, 2));
	kp1.insert('0', (3, 1));
	kp1.insert('A', (3, 2));

	

	let mut dpad = HashMap::new();
	dpad.insert('^', (0, 1));
	dpad.insert('<', (1, 0));
	dpad.insert('v', (1, 1));
	dpad.insert('>', (1, 2));
	dpad.insert('A', (0, 2));

	let mut outcome = 0;

	for line in lines {
		let res = get_codes(line.chars().collect(), &kp1, (3, 2));

		// println!("{:?}", res);
		let res = get_codes_dpad(res, &dpad, (0, 2));
		let res = get_codes_dpad(res, &dpad, (0, 2));

		println!("{}: {}", line,  res.iter().collect::<String>());

		println!("{} * {}", line.chars().take(3).collect::<String>().parse::<usize>().unwrap(), res.len());

		outcome += line.chars().take(3).collect::<String>().parse::<usize>().unwrap() * res.len();
	}

	println!("{:?}", outcome);




}

pub fn get_codes(ins: Vec<char>, keypad: &HashMap<char, (i32, i32)>, start: (i32, i32)) -> Vec<char> {
	let mut curr = start;

	let mut codes = Vec::new();
	for symbol in ins {
		let other = keypad.get(&symbol).expect("Did not find symbol coords in keypad").clone();
		let mut i = calc_displacement(curr, other);

		codes.append(&mut i);
		codes.push('A');

		curr = other;
	}

	return codes
}


pub fn get_codes_dpad(ins: Vec<char>, keypad: &HashMap<char, (i32, i32)>, start: (i32, i32)) -> Vec<char> {
	let mut last = Vec::new();
	let mut split = Vec::new();

	for c in ins {
		if c == 'A' {
			if !last.is_empty() {split.push(last)}
			split.push(vec!['A']);
			last = Vec::new();
		} else {
			last.push(c);
		}
	}

	if !last.is_empty() {
		split.push(last);
	}

	println!("{:?}", split);

	let mut curr = start;
	let mut codes = Vec::new();


	for mut sections in split {
		sections.sort_by(|a, b| {
			let a = keypad.get(a).unwrap().clone();
			let b = keypad.get(b).unwrap().clone();

			return mhd(a, curr).cmp(&mhd(b, curr))
		});

		for symbol in sections {
			let other = keypad.get(&symbol).expect("Did not find symbol coords in keypad").clone();
			let mut i = calc_displacement(curr, other);
	
			codes.append(&mut i);
			codes.push('A');
	
			curr = other;
		}

	}

	return codes
}

pub fn day_21_first_test_example(input: String) {
	let lines = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut m = HashMap::new();

	m.insert((0, 1), '^');
	m.insert((0, 2), 'A');
	m.insert((1, 0), '<');
	m.insert((1, 1), 'v');
	m.insert((1, 2), '>');

	for line in lines {
		let res = reverse(&m, (0, 2), line);
		println!("{:?}", res);
		let res = reverse(&m, (0, 2), res);
		println!("{:?}", res);
	}
}
fn reverse(m: &HashMap<(i32, i32), char>, start: (i32, i32), ins: Vec<char>) -> Vec<char>{
	let (mut y, mut x) = start;

	let mut res = Vec::new();

	
	for c in ins {
		match c {
			'<' => {x-=1;},
			'>' => {x+=1;},
			'v' => {y+=1;},
			'^' => {y-=1;},
			'A' => {res.push(m.get(&(y, x)).unwrap().clone());},
			_ => {}
		}
	}

	return res
}

pub fn calc_displacement(a: (i32, i32), b: (i32, i32)) -> Vec<char> {
	// from a to b;
	let mut ins = Vec::new();

	let y = a.0 - b.0;

	if y > 0 { ins.extend(std::iter::repeat('^').take(y as usize));}
	if y < 0 { ins.extend(std::iter::repeat('v').take(y.abs() as usize));}

	let x = a.1 - b.1;

	if x > 0 { ins.extend(std::iter::repeat('<').take(x as usize));}
	if x < 0 { ins.extend(std::iter::repeat('>').take(x.abs() as usize));}

	return ins

	
}

pub fn mhd(a: (i32, i32), b: (i32, i32)) -> i32 {
	return (a.0 - b.0).abs() + (a.1 - b.1).abs()
}