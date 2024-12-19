use std::{collections::HashMap, io::stdin};

fn main() {
	let (patterns, designs) = parse_input();

	let mut cache = HashMap::<&str, usize>::new();

	let (part1, part2) = designs
		.iter()
		.map(|design| possible_designs(&patterns, design, &mut cache))
		.fold((0, 0), |(part1, part2), possible_designs| {
			let part1 = part1 + usize::min(1, possible_designs);
			let part2 = part2 + possible_designs;
			(part1, part2)
		});
	println!("Part 1: {part1}\nPart 2: {part2}");
}

fn possible_designs<'a>(
	patterns: &[String],
	design: &'a str,
	cache: &mut HashMap<&'a str, usize>,
) -> usize {
	let count = patterns
		.iter()
		.map(|pattern| {
			if pattern == design {
				1
			} else if design.starts_with(pattern) {
				let remaining = &design[pattern.len()..];

				if let Some(v) = cache.get(remaining) {
					*v
				} else {
					possible_designs(patterns, remaining, cache)
				}
			} else {
				0
			}
		})
		.sum();
	cache.insert(design, count);
	count
}

fn parse_input() -> (Vec<String>, Vec<String>) {
	let mut lines = stdin().lines().map(Result::unwrap);

	let patterns = lines
		.next()
		.expect("Expect patterns line")
		.split(", ")
		.map(|s| s.to_string())
		.collect();

	lines.next();

	let designs = lines.collect();

	(patterns, designs)
}
