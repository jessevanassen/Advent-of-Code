use itertools::Itertools as _;
use std::io::stdin;

fn main() {
	let (mut left, mut right) = parse_input();
	left.sort();
	right.sort();

	let part1 = left
		.iter()
		.zip(right.iter())
		.map(|(&left, &right)| left.abs_diff(right))
		.sum::<u32>();
	println!("Part 1: {part1}");

	let counts = right.iter().counts();

	let part2 = left
		.iter()
		.map(|i| *i as usize * counts.get(i).unwrap_or(&0))
		.sum::<usize>();
	println!("Part 2: {part2}");
}

fn parse_input() -> (Vec<u32>, Vec<u32>) {
	stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| {
			let mut parts = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
			(parts.next().unwrap(), parts.next().unwrap())
		})
		.unzip()
}
