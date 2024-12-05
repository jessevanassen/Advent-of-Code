use std::{cmp::Ordering, io::stdin};

use aoc2024::bitset::{self, BitSet};
use tap::Pipe as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OrderingRule(u8, u8);

impl bitset::IntoIndex for OrderingRule {
	fn into_index(self) -> usize {
		self.1 as usize * 100 + self.0 as usize
	}
}

fn main() {
	let (ordering_rules, updates) = parse_input::<_, Vec<_>>();

	let comparator = create_comparator(ordering_rules);

	let (correct_order, incorrect_order) = updates
		.into_iter()
		.partition::<Vec<_>, _>(|pages| pages.is_sorted_by(|&x, &y| comparator(x, y).is_le()));

	let part1 = sum_middles(correct_order);
	println!("Part 1: {part1}");

	let part2 = incorrect_order
		.into_iter()
		.map(|mut pages| {
			pages.sort_by(|&x, &y| comparator(x, y));
			pages
		})
		.pipe(sum_middles);
	println!("Part 2: {part2}");
}

fn create_comparator(ordering_rules: BitSet) -> impl (Fn(u8, u8) -> Ordering) {
	move |lhs, rhs| {
		if ordering_rules.contains(OrderingRule(lhs, rhs)) {
			Ordering::Less
		} else if ordering_rules.contains(OrderingRule(rhs, lhs)) {
			Ordering::Greater
		} else {
			Ordering::Equal
		}
	}
}

fn sum_middles<IntoIter>(items: IntoIter) -> u64
where
	IntoIter: IntoIterator,
	IntoIter::Item: AsRef<[u8]>,
{
	items
		.into_iter()
		.map(|xs| {
			let xs = xs.as_ref();
			xs[xs.len() / 2]
		})
		.fold(0, |acc, x| acc + x as u64)
}

fn parse_input<T, U>() -> (T, U)
where
	T: FromIterator<OrderingRule>,
	U: FromIterator<Vec<u8>>,
{
	let mut lines = stdin().lines().map(Result::unwrap);

	let ordering_rules = (&mut lines)
		.take_while(|line| !line.is_empty())
		.map(|line| {
			let (first, second) = line.split_once("|").unwrap();
			OrderingRule(first.parse().unwrap(), second.parse().unwrap())
		})
		.collect();

	let updates = lines
		.map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
		.collect();

	(ordering_rules, updates)
}
