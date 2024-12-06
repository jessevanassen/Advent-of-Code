use std::{collections::HashSet, io::stdin};

use aoc2024::{Grid, Vector2D};
use itertools::Itertools;

fn main() {
	let grid = parse_input();
	let start_position: Vector2D = grid
		.enumerate()
		.find_map(|(position, square)| (square == &Square::Start).then_some(position))
		.expect("Expect start position in input")
		.try_into()
		.unwrap();

	let route_positions = visited(&grid, start_position)
		.unwrap()
		.into_iter()
		.map(|(position, _)| position)
		.collect::<HashSet<_>>();

	println!("Part 1: {}", route_positions.len());

	let part2 = route_positions
		.iter()
		.flat_map(|&position| {
			[
				position,
				position + Vector2D { x: -1, y: 0 },
				position + Vector2D { x: 1, y: 0 },
				position + Vector2D { x: 0, y: -1 },
				position + Vector2D { x: 0, y: 1 },
			]
			.into_iter()
			.filter_map(|position| <(usize, usize)>::try_from(position).ok())
		})
		.unique()
		.filter(|&position| {
			let mut grid = grid.clone();
			grid[position] = Square::Obstructed;

			visited(&grid, start_position).is_err()
		})
		.count();
	println!("Part 2: {part2}");
}

type Visited = HashSet<(Vector2D, Vector2D)>;

fn visited(grid: &Grid<Square>, mut position: Vector2D) -> Result<Visited, Visited> {
	let mut direction = Vector2D { x: 0, y: -1 };

	let mut visited = Visited::new();
	visited.insert((position, direction));

	loop {
		match <(usize, usize)>::try_from(position + direction)
			.ok()
			.and_then(|index| grid.get(index))
		{
			None => {
				/* Out of bounds */
				return Ok(visited);
			}
			Some(Square::Open) | Some(Square::Start) => {
				position += direction;
			}
			Some(Square::Obstructed) => {
				direction = direction.rotate_ccw();
			}
		}

		if !visited.insert((position, direction)) {
			/* This combination of position and direction has already been
			 * encountered, so the guard is in a loop. */
			return Err(visited);
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
	Open,
	Obstructed,
	Start,
}

fn parse_input() -> Grid<Square> {
	stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| {
			line.into_bytes().into_iter().map(|b| match b {
				b'#' => Square::Obstructed,
				b'.' => Square::Open,
				b'^' => Square::Start,
				_ => unreachable!("Input error"),
			})
		})
		.collect()
}
