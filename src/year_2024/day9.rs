use std::collections::VecDeque;

pub fn day_9_first(input: String) {
	let lines = input.lines().map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();


	let mut id = 0;
	let mut s = 1;

	let mut res = Vec::new();
	let mut n_len = 0;

	for l in lines {
		for c in l {
			match s {
				0 => {
					res.append(&mut vec![-1; c as usize]);
				},
				1 => {
					res.append(&mut vec![id as i32; c as usize]);

					n_len += c as usize;
					id +=1 ;
				},
				_ => {}
			}

			s ^= 1;
		}
	}

	// println!("{:?}", res);
	let mut res = res.into_iter().collect::<VecDeque<_>>();
	let n = res.len();
	let mut r = Vec::new();

	let mut i = 0;
	let mut j = n-1;

	while r.len() < n_len {
		match res[i] {
			-1 => {
				while let Some(c) = res.pop_back() {
					if c != -1 {
						r.push(c);
						break;
					}
				}
				
				j-=1;
			},
			k => {
				r.push(k)
			}
		}
		i += 1;
	}

	println!("{:?}", r);

	let mut total = 0;

	for i in 0..n_len {
		total += r[i] as u128 * i as u128;
	}

	println!("{:?}", total);
	
}

pub fn day_9_second(input: String) {
	let lines = input.lines().map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();


	let mut id = 0;
	let mut s = 1;

	let mut res = Vec::new();
	let mut n_len = 0;

	for l in lines {
		for c in l {
			match s {
				0 => {
					res.push((-1, c as usize));
				},
				1 => {
					res.push((id, c as usize));

					n_len += c as usize;
					id +=1 ;
				},
				_ => {}
			}

			s ^= 1;
		}
	}

	// println!("{:?}", res);
	let mut res = res.into_iter().collect::<VecDeque<_>>();
	let n = res.len();


	let mut end = VecDeque::new();

	// println!("{:?}", res);


	while let Some(b) = res.pop_back() {
		match b {
			(-1, k) => {
				if k > 0 {
					end.push_front((-1, k));
				}
			},
			// _ => {}
			(num, seg) => {
				let l = res.len();
				let mut added = false;

				for i in 0..l {
					match res[i] {
						(-1, k) => {
							if k >= seg {
								res.remove(i);

								if k - seg > 0 {
									res.insert(i, (-1, k - seg));
								}

								res.insert(i, (num, seg));
								end.push_front((-1, seg));
								added = true;
								break;
							}
						},
						_ => {}
					}
				}

				if !added {
					end.push_front((num, seg));
				}
			}
		}
	}

	// println!("{:?}", end);

	let mut total = 0;
	let mut r = Vec::new();

	for e in end {
		r.append(&mut vec![e.0; e.1]);
	}

	

	for i in 0..r.len() {
		if r[i] == -1 {continue;}
		total += r[i] as u128 * i as u128;
	}

	println!("{:?}", total);
	
}