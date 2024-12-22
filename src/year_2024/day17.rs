use std::collections::HashSet;

pub fn day_17_first(input: String) {
	let mut l = input.lines().collect::<Vec<_>>();

	let program = l.pop().unwrap().split(":").skip(1).next().unwrap();

	let program = program.split(",")
	.map(|x| x.trim().parse::<u128>().unwrap())
	.collect::<Vec<_>>();

	let mut registers = Vec::new();

	for line in l {
		if line.is_empty() {continue;}
		let v = line.split(":").skip(1).next().unwrap().trim().parse::<u128>().unwrap();
		registers.push(v);
	}

	let mut d = Device::new(registers[0], registers[1], registers[2], program);

	println!("{:?}", d);

	println!("{}", d.start().trim())
}

pub fn day_17_second(input: String) {
	let mut l = input.lines().collect::<Vec<_>>();

	let program = l.pop().unwrap().split(":").skip(1).next().unwrap();

	let program = program.split(",")
	.map(|x| x.trim().parse::<usize>().unwrap())
	.collect::<Vec<_>>();

	let mut registers = Vec::new();

	for line in l {
		if line.is_empty() {continue;}
		let v = line.split(":").skip(1).next().unwrap().trim().parse::<u128>().unwrap();
		registers.push(v);
	}

	let mut q = Quine::new(program);

	println!("{}", q.find_quine())
}

pub struct Quine{
	program: Vec<usize>
}

impl Quine {
	pub fn new(program: Vec<usize>) -> Self {
		Self {program}
	}
	
	fn find_quine(&mut self) -> usize {
		let mut quines: HashSet<usize> = HashSet::default();
		quines.insert(0);
		for num in self.program.iter().rev() {
				let mut new_quines = HashSet::default();
				for curr in quines {
						for i in 0..8 {
								let new = (curr << 3) + i;
								if Self::get_out(new) == *num {
									new_quines.insert(new);
								}
						}
				}
				quines = new_quines;
		}

		*quines.iter().min().unwrap()
	}

	fn get_out(a: usize) -> usize {
		let partial = ((a % 8) ^ 2);
		return ((partial ^ 7) ^ (a >> partial)) % 8
	}
}

#[derive(Debug, Clone)]
pub struct Device {
	c: ComboOperand,
	l: LiteralOperand,
	program: Vec<u128>,
	needle: usize,
	output: Vec<u128>,
}

impl Device {
	pub fn new(a: u128, b: u128, c: u128, program: Vec<u128>) -> Self {
		Self {
			c: ComboOperand::new(vec![0, 1, 2, 3, a, b, c]),
			l: LiteralOperand::new(vec![0, 1, 2, 3, 4, 5, 6, 7]),
			program,
			needle: 0,
			output: Vec::new()
		}
	}

	pub fn start(&mut self) -> String {
		while self.needle < self.program.len() - 1 {
			self.run_ins();
		}
		// println!("Final state");
		// println!("{:?}", self);
		// println!("-------");
		self.out()
	}

	pub fn out(&self) -> String {
		let mut res = self.output.iter().map(|x| x.to_string() + ",").collect::<String>();
		res.pop();
		res
	}

	pub fn run_ins(&mut self) {
		let mut jumped = false;
		let n = self.needle;
		let (opcode, operand) = (self.program[n], self.program[n+1]);

		match opcode {
			0 => {
				let num = self.c.read_a();
				let shift = self.c.get(operand);
				let v = num.checked_shr(shift as u32).unwrap_or(0);
				self.c.write_to_a(v);
			},
			1 => {
				let result = self.c.read_b() ^ self.l.get(operand);
				self.c.write_to_b(result);
			},
			2 => {
				let result = self.c.get(operand) % 8;
				self.c.write_to_b(result);
			},
			3 => {
				if self.c.read_a() != 0 {
					self.needle = operand as usize;
					jumped = true;
				}
			},
			4 => {
				let res = self.c.read_c() ^ self.c.read_b();
				self.c.write_to_b(res);
			},
			5 => {
				let res = self.c.get(operand) % 8;
				self.output.push(res);
			},
			6 => {
				let num = self.c.read_a();
				let shift = self.c.get(operand);
				let v = num.checked_shr(shift as u32).unwrap_or(0);
				self.c.write_to_b(v);
			},
			7 => {
				let num = self.c.read_a();
				let shift = self.c.get(operand);
				let v = num.checked_shr(shift as u32).unwrap_or(0);
				self.c.write_to_c(v);
			},
			_ => {}
		}

		if !jumped {
			self.needle += 2;
		}
	}
}

#[derive(Debug, Clone)]
pub struct ComboOperand(Vec<u128>);
#[derive(Debug, Clone)]
pub struct LiteralOperand(Vec<u128>);

impl LiteralOperand {
	pub fn new(v: Vec<u128>) -> Self {
		Self(v)
	}


	pub fn get(&self, i: u128) -> u128 {
		return i
	}
}

impl ComboOperand {
	pub fn new(v: Vec<u128>) -> Self {
		Self(v)
	}

	pub fn get(&self, p: u128) -> u128 {
		self.0[p as usize]
	}

	pub fn write_to_a(& mut self, v: u128) { self.0[4] = v}
	pub fn write_to_b(& mut self, v: u128) { self.0[5] = v}
	pub fn write_to_c(& mut self, v: u128) { self.0[6] = v}

	pub fn read_a(&self) -> u128 {self.0[4]}
	pub fn read_b(&self) -> u128 {self.0[5]}
	pub fn read_c(&self) -> u128 {self.0[6]}
	
}

pub enum Instruction {
	Adv(i32, i32),
	Bxl(i32, i32),
	Bst(i32, i32),
	Jnz(i32, i32),
	Bxc(i32, i32),
	Out(i32, i32),
	Bdv(i32, i32),
	Cdv(i32, i32),
}



