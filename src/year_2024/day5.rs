use std::collections::{HashMap, HashSet, VecDeque};

pub fn day_5_first(input: String) {
	let mut  lines = input.lines();


	let mut ordering = HashMap::new();

	while let Some(line) = lines.next() {
		if line.is_empty() {break;}
		let s = line.split("|").collect::<Vec<_>>();

		let (a, b) = (s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap());

		ordering.entry(a).or_insert(HashSet::new()).insert(b);
	}

	let mut instructions = Vec::new();

	while let Some(line) = lines.next() {
		let ins = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		instructions.push(ins);
	}

	let mut total = 0;

	for ins in instructions {
		let l = ins.len();

		let valid = traverse(0, l, &ordering, &ins);

		if valid {
			total += ins[l/2]
		}
	}

	println!("TOTAL: {}", total);


}

fn traverse(i: usize, n: usize, graph: &HashMap<i32, HashSet<i32>>, ins: &Vec<i32>) -> bool {
	if i >= n-1 {return true}

	let node = ins[i];

	if let Some(c) = graph.get(&node) {
		if c.contains(&ins[i+1]) {
			return traverse(i + 1, n, graph, ins);
		} else {
			return false
		}
	}

	false
}



pub fn day_5_second(input: String) {
	let mut  lines = input.lines();


	let mut ordering = HashMap::new();

	while let Some(line) = lines.next() {
		if line.is_empty() {break;}
		let s = line.split("|").collect::<Vec<_>>();

		let (a, b) = (s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap());

		ordering.entry(a).or_insert(HashSet::new()).insert(b);
	}

	let mut instructions = Vec::new();

	while let Some(line) = lines.next() {
		let ins = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		instructions.push(ins);
	}

	let mut total = 0;

	for mut ins in instructions {
		let l = ins.len();

		let valid = traverse(0, l, &ordering, &ins);

		if valid {
			// total += ins[l/2]
		} else {
			total += reorder(&ordering, &ins);
		}
	}

	println!("TOTAL: {}", total);


}

pub fn reorder(graph: &HashMap<i32, HashSet<i32>>, ins: &Vec<i32>) -> i32 {
	// println!("INS: {:?}", ins);
	let mut ins_s = HashSet::from_iter(ins.iter().cloned());
	let mut ins_s_c = ins_s.clone();
	let mut q = Vec::new();

	while q.len() < ins.len() {
		// println!("{:?} {:?}", q, ins_s_c);

		for &c in &ins_s_c {

			if let Some(conn) = graph.get(&c) {
				// println!("C.CONN => {} {:?}", c, conn);
				if conn.intersection(&ins_s).count() == 0 {
					q.push(c);
					// println!("PUSHED: {}", c);
					ins_s_c.remove(&c);
					ins_s.remove(&c);
					break;
				}
			} else {
				q.push(c);
				// println!("PUSHED: {}", c);
				ins_s_c.remove(&c);
				ins_s.remove(&c);
				break;
			}
		}
	}

	// println!("{:?}", q);

	return q[ins.len()/2];


}