use std::collections::BTreeSet;

pub fn day_15_first(input: String) {
	let mut lines = input.lines().collect::<Vec<_>>();

	
	let ins = lines.pop().unwrap();
	

	let (mut a, mut b) = (0, 0);
	let n = lines.len();

	let w = lines[0].len();
	
	let lines = lines.into_iter()
	.map(|x| x.chars().collect::<Vec<_>>())
	.take(n-1).collect::<Vec<_>>();

	let h = lines.len();
	let w = lines[0].len();

	let hs = (0..h).into_iter().collect::<BTreeSet<_>>();
	let ws = (0..w).into_iter().collect::<BTreeSet<_>>();
	let mut hg = vec![hs.clone(); w];
	let mut wg = vec![ws.clone(); h];

	let mut wwg = vec![hs; w];
	let mut hwg = vec![ws; h];

	

	for i in 0..h {
		for j in 0..w {
			match lines[i][j] {
				'@' => {
					a = i;
					b = j;
					hg[j].remove(&i);
					wg[i].remove(&j);
					hwg[j].remove(&i);
					wwg[i].remove(&j);
				},
				'.' => {
					hwg[j].remove(&i);
					wwg[i].remove(&j);
				},
				'#' => {
					hg[j].remove(&i);
					wg[i].remove(&j);
				},
				'O' => {
					hg[j].remove(&i);
					wg[i].remove(&j);
					hwg[j].remove(&i);
					wwg[i].remove(&j);
				},
				_ => {}
			}
		}
	}

	

	// println!("{:?}", ins);
	// println!("{:?}", lines);

	// for w in &wg {
	// 	println!("{:?}", w);
	// }

	
	// for row in &lines {
	// 	println!("{:?}", row);
	// }


	// println!("---");
	let mut warehouse = WareHouse::new((a, b),hg, wg, lines, hwg, wwg);

	for c in ins.chars() {
		warehouse.ins(c);
	}


	let mut res = 0;
	for i in 0..h {
		for j in 0..w {
			if warehouse.grid[i][j] == 'O' {
				res += (100 * i) + j;
			}
		}
	}

	println!("{:?}", res);

}

pub struct WareHouse {
	pos: (usize, usize),
	hs: Vec<BTreeSet<usize>>,
	ws: Vec<BTreeSet<usize>>,
	grid: Vec<Vec<char>>,
	hws: Vec<BTreeSet<usize>>,
	wws: Vec<BTreeSet<usize>>,
}

impl WareHouse {

	pub fn new(pos: (usize, usize), hs: Vec<BTreeSet<usize>>, ws: Vec<BTreeSet<usize>>, grid: Vec<Vec<char>>, hws: Vec<BTreeSet<usize>>, wws: Vec<BTreeSet<usize>>) -> Self {
		Self { pos, hs, ws, grid, hws, wws }
	}

	pub fn ins(&mut self, c: char) {
		let grid = &mut self.grid;
		match c {
			'<' => {
				let (i, j) = self.pos;

				let nf = self.ws[i].range(..j).next_back().cloned();

				if let Some(p) = nf {
					if self.wws[i].range(p..j).count() > 0 {return}
					self.ws[i].remove(&p);
					self.ws[i].insert(j);

					self.hs[j].insert(i);
					self.hs[p].remove(&i);

					if p < j-1 {
						grid[i][p] = 'O';
					}

					grid[i][j] = '.';
					grid[i][j-1] = '@';
					self.pos = (i, j-1);
				}
			},
			'^' => {
				let (i, j) = self.pos;

				let nf = self.hs[j].range(..i).next_back().cloned();

				if let Some(p) = nf {
					if self.hws[j].range(p..i).count() > 0 {return}
					self.hs[j].remove(&p);
					self.hs[j].insert(i);

					self.ws[i].insert(j);
					self.ws[p].remove(&j);

					if p < i-1 {
						grid[p][j] = 'O';
					}

					grid[i][j] = '.';
					grid[i-1][j] = '@';
					self.pos = (i-1, j);
				}
			},
			'v' => {
				let (i, j) = self.pos;

				let nf = self.hs[j].range((i+1)..).next().cloned();

				if let Some(p) = nf {
					if self.hws[j].range(i..p).count() > 0 {return}
					self.hs[j].remove(&p);
					self.hs[j].insert(i);

					self.ws[i].insert(j);
					self.ws[p].remove(&j);

					if p > i+1 {
						grid[p][j] = 'O';
					}

					grid[i][j] = '.';
					grid[i+1][j] = '@';
					self.pos = (i+1, j);
				}
			},
			'>' => {
				
				let (i, j) = self.pos;

				let nf = self.ws[i].range((j+1)..).next().cloned();

				if let Some(p) = nf {
					if self.wws[i].range(j..p).count() > 0 {return}
					self.ws[i].remove(&p);
					self.ws[i].insert(j);

					self.hs[j].insert(i);
					self.hs[p].remove(&i);

					if p > j+1 {
						grid[i][p] = 'O';
					}

					grid[i][j] = '.';
					grid[i][j+1] = '@';
					self.pos = (i, j+1);
				}

			},
			_ => {}
		}
	}
}
