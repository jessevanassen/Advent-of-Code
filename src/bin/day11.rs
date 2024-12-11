use std::{collections::HashMap, io::stdin};

use aoc2024::digits::count_digits;

type Cache = HashMap<(u64, usize), usize>;

fn main() {
	let numbers = parse_input();

	let mut cache = HashMap::new();

	println!("Part 1: {}", solve(&numbers, 25, &mut cache));
	println!("Part 2: {}", solve(&numbers, 75, &mut cache));
}

fn solve(numbers: &[u64], iterations: usize, cache: &mut Cache) -> usize {
	numbers
		.iter()
		.map(|&number| solve_number(number, iterations, cache))
		.sum()
}

fn solve_number(number: u64, remaining: usize, cache: &mut Cache) -> usize {
	if remaining == 0 {
		return 1;
	}

	if let Some(result) = cache.get(&(number, remaining)) {
		return *result;
	}

	let mut solve_next = |number| solve_number(number, remaining - 1, cache);

	let result = if number == 0 {
		solve_next(number + 1)
	} else {
		let digits = count_digits(number);
		if digits % 2 == 0 {
			let [x, y] = split_number(number);
			solve_next(x) + solve_next(y)
		} else {
			solve_next(number * 2024)
		}
	};

	cache.insert((number, remaining), result);

	result
}

fn split_number(number: u64) -> [u64; 2] {
	let digits = count_digits(number);
	let factor = 10u64.pow(digits / 2);

	let left = number / factor * factor;
	let right = number - left;
	let left = left / factor;

	[left, right]
}

fn parse_input() -> Vec<u64> {
	let line = stdin().lines().next().unwrap().unwrap();
	line.split_ascii_whitespace()
		.map(|num| num.parse().unwrap())
		.collect()
}
