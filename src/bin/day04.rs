use std::{io::stdin, ops::RangeInclusive};

use aoc2022::range_utils::{contains, overlaps};

fn main() {
	let pairs = stdin()
		.lines()
		.flatten()
		.map(|line| parse_pair(&line))
		.collect::<Vec<_>>();

	let part1 = pairs
		.iter()
		.filter(|(x, y)| contains(x, y) || contains(y, x))
		.count();
	println!("Part 1: {part1}");

	let part2 = pairs
		.iter()
		.filter(|(x, y)| overlaps(x, y))
		.count();
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
