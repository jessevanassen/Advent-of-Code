use std::{
	io::{stdin, Read},
	ops::Range,
};

fn main() {
	let input = {
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		buf
	};

	let (seeds, mappings) = parse_input(&input);

	let part1 = seeds
		.iter()
		.map(|&seed| mappings.apply(seed))
		.min()
		.unwrap();
	println!("Part 1: {part1}");

	let seed_ranges = seeds
		.chunks(2)
		.map(|c| {
			let start = c[0];
			let length = c[1];
			start..(start + length)
		})
		.collect::<Box<_>>();
	let seed_ranges_contains = |seed| seed_ranges.iter().any(|range| range.contains(&seed));

	let inverted_mappings = mappings.invert();

	// Try all possible destination until we find a matching seed
	let part2 = (0..)
		.find(|&destination| {
			let seed = inverted_mappings.apply(destination);
			seed_ranges_contains(seed)
		})
		.unwrap();

	println!("Part 2: {part2}");
}

fn parse_input(input: &str) -> (Box<[usize]>, Mappings) {
	let mut groups = input.split("\n\n");

	let seeds = groups.next().unwrap().split_once(": ").unwrap().1;
	let seeds = seeds
		.split_whitespace()
		.map(|n| n.parse::<usize>().unwrap())
		.collect::<Box<_>>();

	let mappings = groups
		.map(|group| {
			let rules = group
				.lines()
				.skip(1)
				.map(|line| {
					let mut numbers = line.split_whitespace().map(|n| n.parse::<usize>().unwrap());

					let destination_range_start = numbers.next().unwrap();
					let source_range_start = numbers.next().unwrap();
					let length = numbers.next().unwrap();

					let offset = (destination_range_start as isize) - (source_range_start as isize);

					MappingRule {
						range: source_range_start..(source_range_start + length),
						offset,
					}
				})
				.collect::<Box<_>>();
			Mapping(rules)
		})
		.collect::<Box<_>>();
	let mappings = Mappings(mappings);

	(seeds, mappings)
}

struct Mappings(Box<[Mapping]>);

impl Mappings {
	pub fn apply(&self, value: usize) -> usize {
		self.0
			.iter()
			.fold(value, |value, mapping| mapping.apply(value))
	}

	fn invert(&self) -> Self {
		let mappings = self.0.iter().rev().map(Mapping::invert).collect();
		Self(mappings)
	}
}

struct Mapping(Box<[MappingRule]>);

impl Mapping {
	pub fn apply(&self, value: usize) -> usize {
		self.0
			.iter()
			.find_map(|rule| rule.apply(value))
			.unwrap_or(value)
	}

	fn invert(&self) -> Self {
		let rules = self.0.iter().map(MappingRule::invert).collect();
		Self(rules)
	}
}

struct MappingRule {
	range: Range<usize>,
	offset: isize,
}

impl MappingRule {
	fn apply(&self, value: usize) -> Option<usize> {
		self.range.contains(&value).then(|| {
			let value = value as isize;
			(value + self.offset) as usize
		})
	}

	fn invert(&self) -> MappingRule {
		let start = (self.range.start as isize + self.offset) as usize;
		let end = (self.range.end as isize + self.offset) as usize;
		let range = start..end;

		Self {
			range,
			offset: -self.offset,
		}
	}
}
