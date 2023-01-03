use std::io::stdin;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input: Vec<Vec<u32>> = stdin()
		.lines()
		.flatten()
		.map(|line| {
			line.split_ascii_whitespace()
				.map(|d| d.parse())
				.collect::<Result<_, _>>()
		})
		.collect::<Result<_, _>>()?;

	let part1 = input
		.iter()
		.map(|row| {
			use itertools::MinMaxResult::*;
			match row.iter().copied().minmax() {
				NoElements => panic!(),
				OneElement(x) => x,
				MinMax(min, max) => max - min,
			}
		})
		.sum::<u32>();
	println!("Part 1: {part1}");

	let part2 = input
		.iter()
		.map(|row| {
			let mut combinations = (0..row.len())
				.flat_map(|x| (0..row.len()).map(move |y| (x, y)))
				.filter(|(x, y)| x != y)
				.map(|(x, y)| (row[x], row[y]));
			let (x, y) = combinations
				.find(|(x, y)| x % y == 0)
				.unwrap();
			x / y
		})
		.sum::<u32>();
	println!("Part 2: {part2}");

	Ok(())
}
