use std::collections::HashMap;

// use day4::day_4_second;

use day5::{day_5_first, day_5_second};

#[allow(unused)]
use crate::read_puzzle_input;

pub mod day5;
pub mod day4;

pub fn solve() {
	// let input = read_puzzle_input("src/year_2024/day1.txt");
	// day_1_first(input);
	// let input = read_puzzle_input("src/year_2024/day1.txt");
	// day_1_second(input);
    
	// let input = read_puzzle_input("src/year_2024/day2.txt");
    // day_2_first(input);
    // let input = read_puzzle_input("src/year_2024/day2.txt");
    // day_2_second(input);
    
	// let input = read_puzzle_input("src/year_2024/day3.txt");
    // day_3_first(input);
	// let input = read_puzzle_input("src/year_2024/day3.txt");
    // day_3_second(input);

    // let input = read_puzzle_input("src/year_2024/day4.txt");
    // day_4_first(input);
    // let input = read_puzzle_input("src/year_2024/day4.txt");
    // day_4_second(input);

    // let input = read_puzzle_input("src/year_2024/day5.txt");
    // day_5_first(input);
    let input = read_puzzle_input("src/year_2024/day5.txt");
    day_5_second(input);

}

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
fn day_2_first(input: String) {
	let lines = input.lines()
		.map(|x| x.split_whitespace().map(|n: &str| n.parse::<i32>().unwrap()).collect::<Vec<_>>()
	).collect::<Vec<_>>();

    let mut res = 0;

	for l in lines {
        if l.len() == 1 {res += 1} else {
            if l[0] > l[1] {
                res += l.iter().zip(l.iter().skip(1)).all(|(a, b)|  (a-b) >= 1 && (a-b) <= 3) as usize
            } else if l[0] < l[1] {
                res += l.iter().zip(l.iter().skip(1)).all(|(a, b)|  (b-a) >= 1 && (b-a) <= 3) as usize
            }
        }
	}

    println!("{}", res);

}

#[allow(unused)]
fn day_2_second(input: String) {
	let lines = input.lines()
		.map(|x| x.split_whitespace().map(|n: &str| n.parse::<i32>().unwrap()).collect::<Vec<_>>()
	).collect::<Vec<_>>();

    let mut res = 0;

	for l in lines {
        if l.len() == 1 {res += 1}
        else {
            
            let mut g = false;
                
            g = g || l.iter().zip(l.iter().skip(1)).all(|(a, b)|  (b-a) >= 1 && (b-a) <= 3);
            g  = g || l.iter().zip(l.iter().skip(1)).all(|(a, b)|  (a-b) >= 1 && (a-b) <= 3);
            
            for i in 0..l.len() {
                let temp = l.iter().enumerate().filter(|(x, _)| *x != i).map(|(_, &v)| v).collect::<Vec<_>>();
                g = g || temp.iter().zip(temp.iter().skip(1)).all(|(a, b)|  (b-a) >= 1 && (b-a) <= 3);
            }

            for i in 0..l.len() {
                let temp = l.iter().enumerate().filter(|(x, _)|*x != i).map(|(_, &v)| v).collect::<Vec<_>>();
                g = g || temp.iter().zip(temp.iter().skip(1)).all(|(a, b)|  (a-b) >= 1 && (a-b) <= 3);
            }

            res += g as usize
        }
	}

    println!("{}", res);

}

#[allow(unused)]
fn day_3_first(input: String) {
    let input = input.chars().collect::<Vec<char>>();
    let mut res = 0;

    let n = input.len();
    let mut i = 0;

    while i < n {
        match input[i] {
            'm' => check(i, &input, n, &mut res),
            _ => {}
        }

        i += 1;
    }

    println!("{}", res);
}

#[allow(unused)]
fn day_3_second(input: String) {
    let input = input.chars().collect::<Vec<char>>();
    let mut res = 0;
    let mut x = true;

    let n = input.len();
    let mut i = 0;

    while i < n {
        match input[i] {
            'm' => if x {check(i, &input, n, &mut res)},
            'd' => flag(i, &input, n, &mut x),
            _ => {}
        }

        i += 1;
    }

    println!("{}", res);
}

fn flag(i: usize, input: &Vec<char>, n: usize, x: &mut bool) {
    if i + 3 < n && input[i..=(i+3)].iter().collect::<String>() == "do()".to_string() {
        *x = true;
        return
    }

    if i + 6 < n && input[i..=(i+6)].iter().collect::<String>() == "don't()".to_string() {
        *x = false;
    }
}

fn check(i:usize, input: &Vec<char>, n: usize, res: &mut usize) {
    if i + 2 >= n { return }
    if &input[i..=(i+2)] == &['m', 'u', 'l'] {
        let j = i + 3;
        parse_brackets(j, input, n, res);
    }
}

fn parse_brackets(j: usize, input: &Vec<char>, n: usize, res: &mut usize) {

    if j >= n || input[j] != '(' {return}

    let mut c = 0;

    while c <= 8 && j + c < n && input[j + c] != ')' {
        c += 1;
    }

    if j + c >= n || input[j+c] != ')' {return}

    let s = &input[(j+1)..(j+c)].iter().collect::<String>();

    let vals = s.split(',').map(|x| x.parse::<i32>()).collect::<Vec<_>>();

    if vals.len() != 2 {
        return
    }

    match (vals[0].clone(), vals[1].clone()) {
        (Ok(a),Ok(b)) => {
            *res += (a * b) as usize
        },
        _ => {}
    }
}
