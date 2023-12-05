use std::{
	io::{stdin, Read},
	ops::Range,
};

#[derive(Debug, Clone)]
struct Mapping {
	range: Range<usize>,
	offset: isize,
}

type Mappings = [Mapping];

fn main() {
	let input = {
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		buf
	};

	let (seeds, mappings) = parse_input(&input);

	let find_seed_destination = |seed: usize| {
		mappings
			.iter()
			.fold(seed, |source, mapping| find_mapping(mapping, source))
	};

	let part1 = seeds
		.iter()
		.map(|&seed| find_seed_destination(seed))
		.min()
		.unwrap();
	println!("Part 1: {part1}");

	let part2 = seeds
		.chunks(2)
		.flat_map(|c| {
			let start = c[0];
			let length = c[1];
			start..(start + length)
		})
		.map(find_seed_destination)
		.min()
		.unwrap();
	println!("Part 2: {part2}");
}

fn parse_input(input: &str) -> (Box<[usize]>, Box<[Box<Mappings>]>) {
	let mut groups = input.split("\n\n");

	let seeds = groups
		.next()
		.unwrap()
		.split_once(": ")
		.unwrap()
		.1
		.split_whitespace()
		.map(|n| n.parse::<usize>().unwrap())
		.collect::<Box<_>>();

	let mappings = groups
		.map(|group| {
			group
				.lines()
				.skip(1)
				.map(|line| {
					let mut numbers = line.split_whitespace().map(|n| n.parse::<usize>().unwrap());
					let destination_range_start = numbers.next().unwrap();
					let source_range_start = numbers.next().unwrap();
					let length = numbers.next().unwrap();
					let offset = (destination_range_start as isize) - (source_range_start as isize);

					Mapping {
						range: source_range_start..(source_range_start + length),
						offset,
					}
				})
				.collect::<Box<_>>()
		})
		.collect::<Box<_>>();

	(seeds, mappings)
}

fn find_mapping(mapping: &Mappings, number: usize) -> usize {
	mapping
		.iter()
		.find(| Mapping { range, .. }| range.contains(&number))
		.map(| Mapping { offset, .. }| ((number as isize) + offset) as usize)
		.unwrap_or(number)
}
