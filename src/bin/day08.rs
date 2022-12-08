use std::io::stdin;

use aoc2022::{extensions::TakeWhileInclusiveExt, Grid2D};

type Forest = Grid2D<u8>;
type Coord = (usize, usize);

fn main() {
	let forest: Forest = stdin()
		.lines()
		.flatten()
		.map(|line| line.bytes().map(|b| b - b'0').collect())
		.collect::<Vec<_>>()
		.into();

	// All trees in the outer ring are visible
	let mut visible_tree_count = (forest.width() + forest.height() - 2) * 2;
	let mut highest_scenic_score = 0;

	for (coord, &value) in forest
		.enumerate()
		.filter(|(coord, _)| !in_outer_ring(&forest, coord))
	{
		if straight_coordinates(&forest, coord)
			.into_iter()
			/* In at least one of the directions, all other values should be
			 * lower than the current value. */
			.any(|mut range| range.all(|coord| forest[coord] < value))
		{
			visible_tree_count += 1;
		}

		let scenic_score = straight_coordinates(&forest, coord)
			.map(|range| {
				range
					.take_while_inclusive(|&coord| forest[coord] < value)
					.count()
			})
			.iter()
			.product::<usize>();

		highest_scenic_score = highest_scenic_score.max(scenic_score);
	}

	println!("Part 1: {}", visible_tree_count);
	println!("Part 2: {}", highest_scenic_score);
}

fn in_outer_ring(forest: &Forest, &(x, y): &Coord) -> bool {
	x == 0 || y == 0 || x == forest.width() - 1 || y == forest.height() - 1
}

fn straight_coordinates(forest: &Forest, (x, y): Coord) -> [Box<dyn Iterator<Item = Coord>>; 4] {
	[
		// North
		Box::new((0..y).rev().map(move |y| (x, y))),
		// South
		Box::new(((y + 1)..forest.width()).map(move |y| (x, y))),
		// East
		Box::new((0..x).rev().map(move |x| (x, y))),
		// West
		Box::new(((x + 1)..forest.height()).map(move |x| (x, y))),
	]
}
