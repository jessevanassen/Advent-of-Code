use std::{io::stdin, iter};

use aoc2022::{extensions::Pipe, min_max, Grid2D};

const SAND_START: usize = 500;

type Coord = (usize, usize);
type Cave = Grid2D<bool>;

fn main() {
	let points = stdin()
		.lines()
		.flatten()
		.map_self(|lines| {
			let mut result = Vec::new();
			for line in lines {
				result.extend(parse_path(&line));
			}
			result
		});
	let max_y = points
		.iter()
		.map(|(_, y)| *y)
		.max()
		.unwrap_or(0);

	let height = max_y + 1 + 2;

	/* The maximum possible width, in case the sand encounters no walls and
	 * forms a perfect pyramid. */
	let width = height * 2 + 1;

	/* Because the maximum width of the pyramid is known (and we assume no lines
	 * fall outside of the range of the pyramid), all the x-coordinates can be
	 * safely offset to save a bit of otherwise unused memory. */
	let x_offset = SAND_START - (width / 2);

	let mut cave: Cave = Grid2D::with_size(width, height);
	for (x, y) in points {
		cave[(x - x_offset, y)] = true;
	}

	let sand_start = (SAND_START - x_offset, 0);

	let part1 = drop_sand_until_stable(&mut cave, sand_start);
	println!("Part 1: {part1}");

	// Fill in the floor
	for x in 0..cave.width() {
		let y = cave.height() - 1;
		cave[(x, y)] = true;
	}

	let part2 = part1 + drop_sand_until_stable(&mut cave, sand_start);
	println!("Part 2: {part2}");
}

fn drop_sand_until_stable(cave: &mut Cave, sand_start: Coord) -> usize {
	let mut i = 0;

	while let Some(next_position) = next_sand_position(cave, sand_start) {
		i += 1;
		cave[next_position] = true;

		if next_position == sand_start {
			break;
		}
	}

	i
}

fn next_sand_position(cave: &Cave, mut sand: Coord) -> Option<Coord> {
	const EMPTY: bool = false;

	'outer: loop {
		if sand.1 >= cave.height() {
			return None;
		}

		for coord in [
			(sand.0, sand.1 + 1),     // Directly below
			(sand.0 - 1, sand.1 + 1), // Below-left
			(sand.0 + 1, sand.1 + 1), // Below-right
		] {
			if *cave.get(coord)? == EMPTY {
				sand = coord;
				continue 'outer;
			}
		}

		return Some(sand);
	}
}

fn parse_path(line: &str) -> impl IntoIterator<Item = Coord> + '_ {
	line.split(" -> ")
		.map(|coord| {
			let (x, y) = coord
				.split_once(',')
				.expect("Missing delimiter ','");
			let x = x.parse::<usize>().unwrap();
			let y = y.parse::<usize>().unwrap();
			(x, y)
		})
		.map_self(adjacent_windows)
		.flat_map(|w| -> Box<dyn Iterator<Item = Coord>> {
			if w[0].0 == w[1].0 {
				let (min, max) = min_max(w[0].1, w[1].1);
				let ys = min..=max;
				Box::new(ys.map(move |y| (w[0].0, y)))
			} else {
				let (min, max) = min_max(w[0].0, w[1].0);
				let xs = min..=max;
				Box::new(xs.map(move |x| (x, w[0].1)))
			}
		})
}

fn adjacent_windows<T: Copy>(iter: impl IntoIterator<Item = T>) -> impl Iterator<Item = [T; 2]> {
	let mut iter = iter.into_iter().peekable();

	iter::from_fn(move || {
		let current = iter.next()?;
		let next = *iter.peek()?;
		Some([current, next])
	})
}
