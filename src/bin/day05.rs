use std::io::stdin;

fn main() {
	let (ordering_rules, updates) = parse_input();

	let (correct_order, incorrect_order) = updates
		.iter()
		.partition::<Vec<_>, _>(|pages| is_in_correct_order(&ordering_rules, pages));

	let part1 = correct_order.into_iter().map(middle).sum::<usize>();
	println!("Part 1: {part1}");

	let part2 = incorrect_order
		.into_iter()
		.cloned()
		.map(|mut pages| {
			fix_order(&ordering_rules, &mut pages);
			pages
		})
		.map(middle)
		.sum::<usize>();
	println!("Part 2: {part2}");
}

fn fix_order(ordering_rules: &[(usize, usize)], pages: &mut [usize]) {
	for i in 0..(pages.len() - 1) {
		while !page_is_in_correct_order(ordering_rules, pages[i], &pages[(i + 1)..]) {
			rotate(&mut pages[i..]);
		}
	}
}

/// Moves the first item to the end, second item to the front, etc.
fn rotate<T>(xs: &mut [T]) {
	for i in 0..(xs.len() - 1) {
		xs.swap(i, i + 1);
	}
}

fn is_in_correct_order(ordering_rules: &[(usize, usize)], pages: &[usize]) -> bool {
	(0..(pages.len() - 1))
		.all(|i| page_is_in_correct_order(ordering_rules, pages[i], &pages[(i + 1)..]))
}

fn page_is_in_correct_order(
	ordering_rules: &[(usize, usize)],
	page: usize,
	after: &[usize],
) -> bool {
	after.iter()
		.all(|other| ordering_rules.contains(&(page, *other)))
}

fn middle<T: Copy>(xs: impl AsRef<[T]>) -> T {
	let xs = xs.as_ref();
	xs[xs.len() / 2]
}

fn parse_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
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
