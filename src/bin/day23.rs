use std::{collections::HashSet, io::stdin};

use aoc2022::vec::IVec2D;
use directions::MOVE_ORDER;
use itertools::Itertools;

mod directions {
	use aoc2022::vec::IVec2D;

	pub const NORTH: IVec2D = IVec2D(0, 1);
	pub const SOUTH: IVec2D = IVec2D(0, -1);
	pub const WEST: IVec2D = IVec2D(-1, 0);
	pub const EAST: IVec2D = IVec2D(1, 0);
	pub const NORTH_WEST: IVec2D = IVec2D(-1, 1);
	pub const NORTH_EAST: IVec2D = IVec2D(1, 1);
	pub const SOUTH_WEST: IVec2D = IVec2D(-1, -1);
	pub const SOUTH_EAST: IVec2D = IVec2D(1, -1);
	pub const NORTH_DIRECTION: (IVec2D, [IVec2D; 3]) = (NORTH, [NORTH_EAST, NORTH, NORTH_WEST]);
	pub const SOUTH_DIRECTION: (IVec2D, [IVec2D; 3]) = (SOUTH, [SOUTH_EAST, SOUTH, SOUTH_WEST]);
	pub const WEST_DIRECTION: (IVec2D, [IVec2D; 3]) = (WEST, [NORTH_WEST, WEST, SOUTH_WEST]);
	pub const EAST_DIRECTION: (IVec2D, [IVec2D; 3]) = (EAST, [NORTH_EAST, EAST, SOUTH_EAST]);
	pub const MOVE_ORDER: [&(IVec2D, [IVec2D; 3]); 4] = [
		&NORTH_DIRECTION,
		&SOUTH_DIRECTION,
		&WEST_DIRECTION,
		&EAST_DIRECTION,
	];
	pub const AROUND: [IVec2D; 8] = [
		NORTH_WEST, NORTH, NORTH_EAST, WEST, EAST, SOUTH_WEST, SOUTH, SOUTH_EAST,
	];
}

type Positions = HashSet<IVec2D>;

fn main() -> anyhow::Result<()> {
	let generations = {
		let positions = parse(stdin().lines().flatten());
		let mut result = vec![positions];

		loop {
			let previous = result.last().unwrap();
			let current = evolve(previous, result.len() - 1);

			if &current == previous {
				break;
			}

			result.push(current);
		}

		result
	};

	println!("Part 1: {}", {
		let generation = &generations[10];
		size(generation) - generation.len()
	});
	println!("Part 2: {}", generations.len());

	Ok(())
}

fn evolve(original_positions: &Positions, index: usize) -> Positions {
	struct Movement {
		from: IVec2D,
		to: IVec2D,
	}

	let proposed_movements = original_positions
		.iter()
		.map(|original_position| {
			let contains_elf = |directions: &[IVec2D]| {
				directions.iter().any(|direction| {
					let new_position = *original_position + *direction;
					original_positions.contains(&new_position)
				})
			};

			let next_direction = if !contains_elf(&directions::AROUND) {
				None
			} else {
				cycle_possible_moves(index)
					.find(|(_, adjacent)| !contains_elf(adjacent))
					.map(|(direction, _)| Some(*direction))
					.unwrap_or(None)
			};

			let next_position = next_direction
				.map(|direction| *original_position + direction)
				.unwrap_or(*original_position);

			Movement {
				from: *original_position,
				to: next_position,
			}
		})
		.collect::<Vec<_>>();

	let next_position_occurrences = proposed_movements
		.iter()
		.map(|Movement { to, .. }| *to)
		.counts();

	proposed_movements
		.into_iter()
		.map(|Movement { from, to }| {
			if matches!(next_position_occurrences.get(&to), Some(c) if c <= &1) {
				to
			} else {
				from
			}
		})
		.collect()
}

fn cycle_possible_moves(n: usize) -> impl Iterator<Item = &'static (IVec2D, [IVec2D; 3])> {
	let moves = directions::MOVE_ORDER.len();
	MOVE_ORDER
		.iter()
		.copied()
		.cycle()
		.skip(n % moves)
		.take(moves)
}

fn parse(lines: impl IntoIterator<Item = String>) -> Positions {
	let mut result = Positions::new();

	for (y, line) in lines.into_iter().enumerate() {
		for (x, b) in line.bytes().enumerate() {
			if b == b'#' {
				/* Flip the y-axis, so the coordinates are in world coordinates
				 * instead of screen coordinates. */
				let coordinate = IVec2D(x as _, -(y as i32));
				result.insert(coordinate);
			}
		}
	}

	result
}

fn min_max<'a>(positions: impl IntoIterator<Item = &'a IVec2D>) -> Option<(IVec2D, IVec2D)> {
	positions
		.into_iter()
		.fold(None, |acc, &IVec2D(x, y)| match acc {
			None => Some((IVec2D(x, y), IVec2D(x, y))),
			Some((min, max)) => Some((
				IVec2D(min.0.min(x), min.1.min(y)),
				IVec2D(max.0.max(x), max.1.max(y)),
			)),
		})
}

fn size<'a>(positions: impl IntoIterator<Item = &'a IVec2D>) -> usize {
	min_max(positions)
		.map(|(min, max)| ((max.0 - min.0 + 1) * (max.1 - min.1 + 1)) as usize)
		.unwrap_or(0)
}
