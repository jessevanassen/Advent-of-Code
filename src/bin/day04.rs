use std::{io::stdin, ops::RangeInclusive};

use aoc2022::range_utils::{contains, overlaps};

fn main() {
	let (part1, part2) = stdin()
		.lines()
		.flatten()
		.map(|line| parse_pair(&line))
		.fold((0, 0), |acc, pair| {
			let contains = contains(&pair.0, &pair.1) || contains(&pair.1, &pair.0);
			let overlaps = overlaps(&pair.0, &pair.1);
			(acc.0 + contains as i32, acc.1 + overlaps as i32)
		});

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}

fn parse_pair(line: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
	let mut values = line
		.split(&['-', ','])
		.map(|v| v.parse().unwrap());

	(
		(values.next().unwrap())..=(values.next().unwrap()),
		(values.next().unwrap())..=(values.next().unwrap()),
	)
}
