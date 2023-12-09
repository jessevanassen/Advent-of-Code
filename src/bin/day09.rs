use std::io::stdin;

use itertools::Itertools;

fn main() {
	let lines = stdin().lines().flatten();
	let input = lines
		.map(|l| {
			l.split_whitespace()
				.map(|n| n.parse::<i64>().unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	let part1 = input.iter().map(solve).sum::<i64>();
	let part2 = input
		.iter()
		.map(|line| solve(line.iter().rev()))
		.sum::<i64>();
	dbg!(part1, part2);
}

fn solve<'a>(numbers: impl IntoIterator<Item = &'a i64>) -> i64 {
	fn solve(numbers: Vec<i64>) -> i64 {
		let mut patterns: Vec<Vec<i64>> = vec![numbers];

		loop {
			let differences = calculate_differences(patterns.last().unwrap());
			patterns.push(differences);

			if patterns.last().unwrap().iter().all_equal() {
				break;
			}
		}

		patterns.iter().filter_map(|xs| xs.last()).sum::<i64>()
	}

	solve(numbers.into_iter().copied().collect())
}

fn calculate_differences<'a>(numbers: impl IntoIterator<Item = &'a i64>) -> Vec<i64> {
	let windows = numbers.into_iter().tuple_windows();
	let differences = windows.map(|(x, y)| y - x);
	differences.collect()
}
