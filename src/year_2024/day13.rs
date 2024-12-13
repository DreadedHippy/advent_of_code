pub fn day_13_first(input: String) {
	let lines = input.lines().collect::<Vec<_>>();

	let mut n = lines.len();

	let mut sum = 0;

	for i in (0..n).step_by(4) {
		let a = lines[i].split(": ").skip(1).next().unwrap();
		let b = lines[i+1].split(": ").skip(1).next().unwrap();
		let p = lines[i+2].split(": ").skip(1).next().unwrap();

		let am = a.split(", ").collect::<Vec<_>>();
		let bm = b.split(", ").collect::<Vec<_>>();
		let pm = p.split(", ").collect::<Vec<_>>();

		let ax = am[0].chars().skip_while(|x| !x.is_numeric()).collect::<String>();
		let ay = am[1].chars().skip_while(|x| !x.is_numeric()).collect::<String>();

		let bx = bm[0].chars().skip_while(|x| !x.is_numeric()).collect::<String>();
		let by = bm[1].chars().skip_while(|x| !x.is_numeric()).collect::<String>();

		let px = pm[0].chars().skip_while(|x| !x.is_numeric()).collect::<String>();
		let py = pm[1].chars().skip_while(|x| !x.is_numeric()).collect::<String>();


		let c = Claw {
			a: (ax.parse::<i128>().unwrap(), ay.parse::<i128>().unwrap()),
			b: (bx.parse::<i128>().unwrap(), by.parse::<i128>().unwrap()),
			p: (px.parse::<i128>().unwrap(), py.parse::<i128>().unwrap()),
		};

		solve_first(c, &mut sum);
	}

	println!("{}", sum)
}

pub fn day_13_second(input: String) {
	let lines = input.lines().collect::<Vec<_>>();

	let mut n = lines.len();

	let mut sum = 0;

	for i in (0..n).step_by(4) {
		let a = lines[i].split(": ").skip(1).next().unwrap();
		let b = lines[i+1].split(": ").skip(1).next().unwrap();
		let p = lines[i+2].split(": ").skip(1).next().unwrap();

		let am = a.split(", ").collect::<Vec<_>>();
		let bm = b.split(", ").collect::<Vec<_>>();
		let pm = p.split(", ").collect::<Vec<_>>();

		let ax = am[0].chars().skip_while(|x| !x.is_numeric()).collect::<String>();
		let ay = am[1].chars().skip_while(|x| !x.is_numeric()).collect::<String>();

		let bx = bm[0].chars().skip_while(|x| !x.is_numeric()).collect::<String>();
		let by = bm[1].chars().skip_while(|x| !x.is_numeric()).collect::<String>();

		let px = pm[0].chars().skip_while(|x| !x.is_numeric()).collect::<String>();
		let py = pm[1].chars().skip_while(|x| !x.is_numeric()).collect::<String>();


		let c = Claw {
			a: (ax.parse::<i128>().unwrap(), ay.parse::<i128>().unwrap()),
			b: (bx.parse::<i128>().unwrap(), by.parse::<i128>().unwrap()),
			p: (px.parse::<i128>().unwrap() + 10000000000000, py.parse::<i128>().unwrap()+ 10000000000000),
		};

		solve_second(c, &mut sum);
	}

	println!("{}", sum)
}

fn solve_first(c: Claw, sum: &mut usize) {
	let mut d = (c.p.0/c.a.0).max(c.p.1/c.a.1) + 1;
	let mut e = 0;

	let (a, b, p) = (c.a, c.b, c.p);

	let mut cost = i128::MAX;

	println!("A begin: {}", d);

	while d > 0 {
		let mut g1 = p.0 - ((d * a.0) + (e* b.0));
		let mut g2 = p.1 - ((d * a.1) + (e* b.1));

		while g1 >= b.0 && g2 >= b.1 {
			e += 1;
			g1 = p.0 - ((d * a.0) + (e* b.0));
			g2 = p.1 - ((d * a.1) + (e* b.1));
		}

		// if d == 80 {
		// 	println!("A now: {}, B now: {}",d,e);
		// 	println!("G1 now: {}, G2 now: {}",g1,g2);
		// }
		
		if g1 == 0 && g2 == 0 {
			cost = cost.min((3 * d) + e)
		}

		d -= 1;
	}

	match cost {
		i128::MAX => {},
		k => {*sum += (k as usize)}
	}
	

}


fn solve_second(c: Claw, sum: &mut u128) {
	let Claw {
		a: (a, c),
		b: (b, d),
		p: (u, v)
	} = c;


	let y = ((u * c) - (v * a)) / ((c * b) - (d * a));
  let x = (u - (b * y)) / a;

  if ((a * x) + (b * y) == u && (c * x) + (d * y) == v)	{
		if x < 0 || y < 0 {
			println!("NEG")
		} else {
			*sum += (3 * x) as u128;
			*sum += y as u128;
		}
	}

}

fn lcm(n1: i128, n2: i128) -> i128 {
	let (mut x, mut y, mut rem) = (0, 0, 0);
	if (n1 > n2) {
			x = n1;
			y = n2;
	}
	else {
			x = n2;
			y = n1;
	}

	rem = x % y;

	while (rem != 0) {
			x = y;
			y = rem;
			rem = x % y;
	}

	n1 * n2 / y
}


#[derive(Debug)]
pub struct Claw {
	pub a: (i128, i128),
	pub b: (i128, i128),
	pub p: (i128, i128),
}