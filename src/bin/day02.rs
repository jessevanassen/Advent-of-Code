use std::io::stdin;

use itertools::Itertools;

fn main() {
	let (part1, part2) = parse_input().fold((0, 0), |counts, report| {
		(
			counts.0 + is_safe(report.iter()) as usize,
			counts.1 + is_safe_dampened(&report) as usize,
		)
	});
	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}

fn is_safe_dampened(input: &[u8]) -> bool {
	(0..input.len()).any(|i| {
		let iter = input
			.iter()
			.enumerate()
			.filter_map(|(j, x)| (i != j).then_some(x));
		is_safe(iter)
	})
}

fn is_safe<'a>(input: impl Iterator<Item = &'a u8> + Clone) -> bool {
	fn all<'a>(input: impl Iterator<Item = &'a u8>, predicate: impl Fn(&u8, &u8) -> bool) -> bool {
		input.tuple_windows().all(|(x, y)| predicate(x, y))
	}

	fn all_increasing<'a>(input: impl Iterator<Item = &'a u8>) -> bool {
		all(input, |x, y| x < y)
	}

	fn all_decreasing<'a>(input: impl Iterator<Item = &'a u8>) -> bool {
		all(input, |x, y| x > y)
	}

	fn has_safe_distances<'a>(input: impl Iterator<Item = &'a u8>) -> bool {
		all(input, |x, y| (1..=3).contains(&x.abs_diff(*y)))
	}

	(all_increasing(input.clone()) || all_decreasing(input.clone()))
		&& has_safe_distances(input.clone())
}

fn parse_input() -> impl Iterator<Item = Vec<u8>> {
	stdin().lines().map(Result::unwrap).map(|line| {
		line.split_whitespace()
			.map(|n| n.parse().unwrap())
			.collect()
	})
}
