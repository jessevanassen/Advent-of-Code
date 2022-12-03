use std::io::stdin;

use aoc2022::extensions::Pipe;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Rucksack(u64);

impl FromIterator<u8> for Rucksack {
	fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
		iter.into_iter().by_ref().collect()
	}
}

impl<'a> FromIterator<&'a u8> for Rucksack {
	fn from_iter<T: IntoIterator<Item = &'a u8>>(iter: T) -> Self {
		let mut value = 0u64;
		for item in iter {
			value |= 1 << item;
		}
		Self(value)
	}
}

impl Rucksack {
	fn iter(&self) -> impl Iterator<Item = u8> + '_ {
		(1..=(26 + 26)).filter(|priority| self.0 >> priority & 1 == 1)
	}

	fn intersection(&self, other: &Self) -> Self {
		Self(self.0 & other.0)
	}
}

fn main() {
	let lines: Vec<Vec<u8>> = stdin()
		.lines()
		.flatten()
		.map(|ref line| parse_line(line))
		.collect();

	fn sum_common_items(
		rucksack_groups: impl Iterator<Item = impl Iterator<Item = Rucksack>>,
	) -> u32 {
		rucksack_groups.map(common_item).map(|x| x as u32).sum()
	}

	let part1: u32 = split_compartments(&lines).map_self(sum_common_items);
	println!("Part 1: {part1}");

	let part2: u32 = collect_by_groups(&lines).map_self(sum_common_items);

	println!("Part 2: {part2}");
}

fn parse_line(line: &str) -> Vec<u8> {
	line.bytes().map(priority).collect()
}

fn priority(x: u8) -> u8 {
	match x {
		_ if x.is_ascii_lowercase() => x - b'a' + 1,
		_ if x.is_ascii_uppercase() => x - b'A' + 27,
		_ => panic!(),
	}
}

/// Extracts the single item that is common between all the rucksacks.
/// Panics if there isn't exactly one common item.
fn common_item(rucksacks: impl IntoIterator<Item = Rucksack>) -> u8 {
	let intersected = rucksacks
		.into_iter()
		.reduce(|acc, ref x| acc.intersection(x))
		.unwrap();
	let mut common_items = intersected.iter();

	let common_item = common_items.next();

	if common_item.is_none() || common_items.next().is_some() {
		panic!("Expected a single item");
	}

	common_item.unwrap()
}

fn split_compartments(
	lines: &[Vec<u8>],
) -> impl Iterator<Item = impl Iterator<Item = Rucksack> + '_> {
	lines
		.iter()
		.map(|line| split_middle(line))
		.map(|group| group.into_iter().map(|x| x.iter().collect()))
}

fn split_middle<T>(items: &[T]) -> [&[T]; 2] {
	let mid = items.len() / 2;
	[&items[..mid], &items[mid..]]
}

fn collect_by_groups(
	lines: &[Vec<u8>],
) -> impl Iterator<Item = impl Iterator<Item = Rucksack> + '_> {
	lines
		.chunks_exact(3)
		.into_iter()
		.map(|group| group.into_iter().map(|x| x.iter().collect()))
}
