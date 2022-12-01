use aoc2022::extensions::Pipe;
use std::io::stdin;

fn main() {
	let calories = stdin()
		.lines()
		.flatten()
		.map(|line| line.parse().ok())
		.map_self(SumPartition::new)
		.collect::<Vec<_>>()
		.apply_to_self(|it| it.sort_by(|x, y| Ord::cmp(y, x)));

	let part1 = calories[0];
	println!("Part 1: {part1}");

	let part2: u64 = calories[..3].iter().sum();
	println!("Part 2: {part2}");
}

struct SumPartition<T>(T);

impl<T> SumPartition<T>
where
	T: Iterator<Item = Option<u32>>,
{
	pub fn new(iter: T) -> Self {
		Self(iter)
	}
}

impl<T: Iterator<Item = Option<u32>>> Iterator for SumPartition<T> {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		let mut sum: u64 = 0;
		while let Some(value) = self.0.next()? {
			sum += value as u64;
		}
		Some(sum)
	}
}
