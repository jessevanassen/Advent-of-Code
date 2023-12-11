use std::io::stdin;

use itertools::Itertools;

type Coordinate = [usize; 2];

fn main() {
	let coordinates: Vec<Coordinate> = stdin()
		.lines()
		.flatten()
		.enumerate()
		.flat_map(|(y, line)| {
			line.into_bytes()
				.into_iter()
				.enumerate()
				.filter(|(_, s)| *s == b'#')
				.map(move |(x, _)| [x, y])
		})
		.collect();

	let solve = |replace_empty_by: usize| {
		expand_universe(&coordinates, replace_empty_by)
			.into_iter()
			.tuple_combinations()
			.map(|(c1, c2)| manhattan_distance(c1, c2))
			.sum::<usize>()
	};

	println!("Part 1: {}", solve(2));
	println!("Part 2: {}", solve(1_000_000));
}

fn expand_universe(coordinates: &[Coordinate], replace_empty_by: usize) -> Vec<Coordinate> {
	fn find_replacements<Iter>(numbers: Iter, replace_empty_by: usize) -> Vec<usize>
	where
		Iter: Iterator<Item = usize> + Clone,
	{
		let max = numbers.clone().max().unwrap();

		// Effectively a Set
		let mut contains = vec![false; max + 1];
		for it in numbers {
			contains[it] = true;
		}

		let replacements = (0..=max).map(|value| {
			if !contains[value] {
				replace_empty_by
			} else {
				1
			}
		});

		let cumulative_sum = replacements.scan(0, |acc, it| {
			*acc += it;
			Some(*acc)
		});

		cumulative_sum.collect()
	}

	let x_replacements = find_replacements(coordinates.iter().map(|c| c[0]), replace_empty_by);
	let y_replacements = find_replacements(coordinates.iter().map(|c| c[1]), replace_empty_by);

	coordinates
		.iter()
		.map(|&[x, y]| [x_replacements[x], y_replacements[y]])
		.collect()
}

fn manhattan_distance([x0, y0]: Coordinate, [x1, y1]: Coordinate) -> usize {
	x0.abs_diff(x1) + y0.abs_diff(y1)
}
