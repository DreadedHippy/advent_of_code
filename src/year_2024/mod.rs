#![allow(unused)]
use std::{collections::HashMap, time::Instant};

// use day4::day_4_second;

use day5::{day_5_first, day_5_second};
use day1::{day_1_first, day_1_second};
use day2::{day_2_first, day_2_second};
use day3::{day_3_first, day_3_second};
use day6::{day_6_first, day_6_second};
use day7::{day_7_first, day_7_second};
use day8::{day_8_first, day_8_second};

#[allow(unused)]
use crate::read_puzzle_input;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

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
    // let input = read_puzzle_input("src/year_2024/day5.txt");
    // day_5_second(input);

    // let input = read_puzzle_input("src/year_2024/day6.txt");
    // day_6_first(input);    
    // let input = read_puzzle_input("src/year_2024/day6.txt");
    // day_6_second(input);

    // let input = read_puzzle_input("src/year_2024/day7.txt");
    // day_7_first(input);    
    // let input = read_puzzle_input("src/year_2024/day7.txt");
    // day_7_second(input);

    let input = read_puzzle_input("src/year_2024/day8.txt");
    day_8_second(input);

}
