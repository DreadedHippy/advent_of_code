pub fn day_7_first(input: String) {
	let lines = input.lines().map(|x| {
		let mut iter = x.split(":");
		let a = iter.next().unwrap().parse::<u64>().unwrap();
		let mut nums = iter.next().unwrap();

		let nums = nums.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

		(a, nums)
	}).collect::<Vec<_>>();

	// println!("{:?}", lines);



	let mut count = 0;

	'outer: for line in lines {
		let mut s = Symbols::new(line.1.len() - 1);

		if parse(&line, &s) {
			count += line.0;
			continue;
		}

		while s.next() {
			if parse(&line, &s) {
				count += line.0;
				continue 'outer;
			}
		}
	}

	println!("{}", count)
}

pub fn day_7_second(input: String) {
	let lines = input.lines().map(|x| {
		let mut iter = x.split(":");
		let a = iter.next().unwrap().parse::<u64>().unwrap();
		let mut nums = iter.next().unwrap();

		let nums = nums.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

		(a, nums)
	}).collect::<Vec<_>>();
	
	let mut count = 0;

	'outer: for line in lines {
		let mut s = Symbols2::new(line.1.len() - 1);

		if parse2(&line, &s) {
			count += line.0;
			continue;
		}

		while s.next() {
			if parse2(&line, &s) {
				count += line.0;
				continue 'outer;
			}
		}
	}

	println!("{}", count)

}


pub struct Symbols2(Vec<i32>);

impl Symbols2 {
	pub fn new(n: usize) -> Self {
		Symbols2(vec![0; n])
	}

	pub fn next(&mut self) -> bool {
		let n = self.0.len();
		let mut changed = false;

		for i in (0..n).rev() {
			if self.0[i] == 0 || self.0[i] == 1{
				self.0[i] += 1;

				for j in i+1..n {
					self.0[j] = 0;
				}

				changed = true;
				break;				
			}
		}


		changed
	}
}


pub fn parse2(line: &(u64, Vec<u64>), s: &Symbols2) -> bool {
	let mut a = line.1[0];
	let ans = line.0;

	let n = s.0.len();

	for i in 0..s.0.len() {
		match s.0[i] {
			0 => {a += line.1[i+1]},
			1 => {a *= line.1[i+1]},
			2 => {a = format!("{}{}",a,line.1[i+1]).parse::<u64>().unwrap()},
			_ => {}
		}

		if a > ans {
			break;
		}
	}

	a == ans
}

pub struct Symbols(Vec<i32>);

impl Symbols {
	pub fn new(n: usize) -> Self {
		Symbols(vec![0; n])
	}

	pub fn next(&mut self) -> bool {
		let n = self.0.len();
		let mut changed = false;

		for i in (0..n).rev() {
			if self.0[i] == 0 {
				self.0[i] += 1;

				for j in i+1..n {
					self.0[j] = 0;
				}

				changed = true;
				break;				
			}
		}


		changed
	}
}

pub fn parse(line: &(u64, Vec<u64>), s: &Symbols) -> bool {
	let mut a = line.1[0];
	let ans = line.0;

	let n = s.0.len();

	for i in 0..s.0.len() {
		match s.0[i] {
			0 => {a += line.1[i+1]},
			1 => {a *= line.1[i+1]},
			_ => {}
		}

		if a > ans {
			break;
		}
	}

	a == ans
}