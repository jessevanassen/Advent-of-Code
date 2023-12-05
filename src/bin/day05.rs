use std::{
	io::{stdin, Read},
	ops::Range,
};

use aoc2023::range::overlaps;

fn main() {
	let input = {
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		buf
	};

	let (seeds, mappings) = parse_input(&input);

	println!("Part 1: {}", {
		let seed_ranges = seeds.iter().map(|&seed| (seed..(seed + 1)));
		min_destination_for_seed_ranges(seed_ranges, &mappings)
	});

	println!("Part 2: {}", {
		let seed_ranges = seeds.chunks(2).map(|c| {
			let start = c[0];
			let length = c[1];
			start..(start + length)
		});
		min_destination_for_seed_ranges(seed_ranges, &mappings)
	});
}

fn min_destination_for_seed_ranges(ranges: impl Iterator<Item = Range<usize>>, mappings: &Mappings) -> usize {
	// Is there a way to do this without allocating a Vec for every step?

	let mut ranges = ranges.collect::<Vec<_>>();

	for mapping in mappings {
		ranges = ranges
			.iter()
			.flat_map(|range| split_range(range, mapping))
			.collect();
	}

	ranges.into_iter().map(|r| r.start).min().unwrap()
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
			let mut rules = group
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
				.collect::<Vec<_>>();
			rules.sort_by_key(|m| m.range.start);
			rules
		})
		.collect();

	(seeds, mappings)
}

type Mappings = Vec<Mapping>;
type Mapping = Vec<MappingRule>;
struct MappingRule {
	range: Range<usize>,
	offset: isize,
}

/// Splits a range into several subranges, based on the given mapping.
/// The mapping's offset is applied to the range that overlaps with the mapping.
fn split_range(range: &Range<usize>, mapping: &Mapping) -> Vec<Range<usize>> {
	let mut rules = mapping
		.iter()
		.filter(|rule| overlaps(range, &rule.range))
		.peekable();

	if rules.peek().is_none() {
		return vec![range.clone()];
	}

	let mut result = Vec::new();

	let first = rules.peek().unwrap();
	if range.start < first.range.start {
		result.push(range.start..first.range.start);
	}

	while let Some(rule) = rules.next() {
		let start = rule.range.start.max(range.start);
		let start = (start as isize + rule.offset) as usize;
		let end = rule.range.end.min(range.end);
		let end = (end as isize + rule.offset) as usize;
		result.push(start..end);

		let next_start = rules
			.peek()
			.map(|rule| rule.range.start.min(range.end))
			.unwrap_or(range.end);

		if rule.range.end < next_start {
			result.push(rule.range.end..next_start);
		}
	}

	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_split_range() {
		let mapping: Mapping = vec![
			MappingRule {
				range: 20..40,
				offset: 1000,
			},
			MappingRule {
				range: 60..80,
				offset: 2000,
			},
		];

		let ranges = split_range(&(45..55), &mapping);
		assert_eq!(ranges, vec![45..55]);

		let ranges = split_range(&(0..40), &mapping);
		assert_eq!(ranges, vec![0..20, 1020..1040]);

		let ranges = split_range(&(20..60), &mapping);
		assert_eq!(ranges, vec![1020..1040, 40..60]);

		let ranges = split_range(&(20..40), &mapping);
		assert_eq!(ranges, vec![1020..1040]);

		let ranges = split_range(&(0..100), &mapping);
		assert_eq!(ranges, vec![0..20, 1020..1040, 40..60, 2060..2080, 80..100]);
	}
}
