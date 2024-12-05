use std::{cmp::Ordering, collections::HashSet, io::stdin};

use tap::Pipe as _;

fn main() {
	let (ordering_rules, updates) = parse_input::<_, Vec<_>>();

	let comparator = create_comparator(ordering_rules);

	let (correct_order, incorrect_order) = updates
		.into_iter()
		.partition::<Vec<_>, _>(|pages| pages.is_sorted_by(|x, y| comparator(x, y).is_le()));

	let part1 = sum_middles(correct_order);
	println!("Part 1: {part1}");

	let part2 = incorrect_order
		.into_iter()
		.map(|mut pages| {
			pages.sort_by(|x, y| comparator(x, y));
			pages
		})
		.pipe(sum_middles);
	println!("Part 2: {part2}");
}

fn create_comparator(
	ordering_rules: HashSet<(usize, usize)>,
) -> impl (Fn(&usize, &usize) -> Ordering) {
	move |lhs, rhs| {
		if ordering_rules.contains(&(*lhs, *rhs)) {
			Ordering::Less
		} else if ordering_rules.contains(&(*rhs, *lhs)) {
			Ordering::Greater
		} else {
			Ordering::Equal
		}
	}
}

fn sum_middles<IntoIter>(items: IntoIter) -> usize
where
	IntoIter: IntoIterator,
	IntoIter::Item: AsRef<[usize]>,
{
	items
		.into_iter()
		.map(|xs| {
			let xs = xs.as_ref();
			xs[xs.len() / 2]
		})
		.sum()
}

fn parse_input<T, U>() -> (T, U)
where
	T: FromIterator<(usize, usize)>,
	U: FromIterator<Vec<usize>>,
{
	let mut lines = stdin().lines().map(Result::unwrap);

	let ordering_rules = (&mut lines)
		.take_while(|line| !line.is_empty())
		.map(|line| {
			let (first, second) = line.split_once("|").unwrap();
			(first.parse().unwrap(), second.parse().unwrap())
		})
		.collect();

	let updates = lines
		.map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
		.collect();

	(ordering_rules, updates)
}
