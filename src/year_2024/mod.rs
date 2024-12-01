use std::collections::HashMap;

use crate::read_puzzle_input;

#[allow(unused)]
macro_rules! cina {
    ($t:tt) => {{
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).expect("fail");
        temp.split_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}


#[allow(unused)]
macro_rules! cin {
    ($t:tt) => {{
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).expect("fail");
        temp.trim().parse::<$t>().unwrap()
    }};
}

#[allow(unused)]
macro_rules! cins {
    () => {{
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).expect("fail");
        temp.trim().to_string()
    }};
}

#[allow(unused)]
macro_rules! cinc {
    () => {{
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).expect("fail");
        temp.trim().chars()
        .collect::<Vec<char>>()
    }};
}

#[allow(unused)]
macro_rules! tuple {
    [$xs:expr; 2] => {
        { let value = $xs; (value[0], value[1]) }
    };
    [$xs:expr; 3] => {
        { let value = $xs; (value[0], value[1], value[2]) }
    };
    [$xs:expr; 4] => {
        { let value = $xs; (value[0], value[1], value[2], value[3]) }
    };
    [$xs:expr; $t:ty ;2] => {
        { let value = $xs; (value[0] as $t, value[1] as $t) }
    };
    [$xs:expr; $t:ty; 3] => {
        { let value = $xs; (value[0] as $t, value[1] as $t, value[2] as $t) }
    };
    [$xs:expr; $t:ty; 4] => {
        { let value = $xs; (value[0] as $t, value[1] as $t, value[2] as $t, value[3] as $t) }
    };
}

pub fn solve() {
	let input = read_puzzle_input("src/year_2024/day1.txt");
	// day_1_first(input);
	day_1_second(input);
}

fn day_1_first(input: String) {
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

fn day_1_second(input: String) {
	
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