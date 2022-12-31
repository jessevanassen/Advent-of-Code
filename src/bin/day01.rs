use std::{collections::HashSet, io::stdin};

fn main() -> anyhow::Result<()> {
	let input: Vec<i64> = stdin()
		.lines()
		.flatten()
		.map(|line| {
			line.parse()
				.map_err(anyhow::Error::from)
		})
		.collect::<anyhow::Result<_>>()?;

	let part1: i64 = input.iter().sum();
	println!("Part 1: {part1}");

	let part2 = {
		let mut iter = input.iter().cycle();
		let mut seen = HashSet::new();
		let mut sum = 0;
		loop {
			sum += iter
				.next()
				.expect("Iter should have been infinite");
			if !seen.insert(sum) {
				break sum;
			}
		}
	};

	println!("Part 2: {part2}");

	Ok(())
}
