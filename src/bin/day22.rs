use std::{collections::HashMap, io::stdin, iter};

use aoc2022::{vec::IVec2D, Grid2D};

type Map = Vec<Vec<Cell>>;

const FACE_SIZE: usize = 50;

/* The vectors are in "space coordinates", but the puzzle is in screen
 * coordinates, which is why the vertical axis is flipped. */
const UP: IVec2D = IVec2D::DOWN;
const DOWN: IVec2D = IVec2D::UP;
const LEFT: IVec2D = IVec2D::LEFT;
const RIGHT: IVec2D = IVec2D::RIGHT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Placement {
	position: IVec2D,
	direction: IVec2D,
}

fn main() -> anyhow::Result<()> {
	let lines: Vec<_> = stdin().lines().flatten().collect();
	let empty = lines
		.iter()
		.position(|line| line.is_empty())
		.unwrap();
	let map = parse_map(
		lines[..empty]
			.iter()
			.map(String::as_str),
	);
	let instructions = parse_instructions(&lines[empty + 1]);

	let start = IVec2D(
		map[0]
			.iter()
			.position(|s| s == &OPEN)
			.unwrap() as _,
		0,
	);

	let part1 = solve(
		start,
		&instructions,
		|original @ Placement {
		     position,
		     direction,
		 }| {
			let mut next_position = position + direction;
			if index(&map, next_position) == WRAP {
				/* Walk backwards until we reach the beginning on
				 * this axis. */
				loop {
					let x = next_position + direction.flip();
					if index(&map, x) == WRAP {
						break;
					}
					next_position = x;
				}
			}
			if index(&map, next_position) == OPEN {
				Placement {
					position: next_position,
					direction,
				}
			} else {
				original
			}
		},
	);
	println!("Part 1: {}", password(part1));

	let stitched = stitch_cube(&map, FACE_SIZE);
	let part2 = solve(
		start,
		&instructions,
		|original @ Placement {
		     mut position,
		     mut direction,
		 }| {
			/* Check if we're about to enter a portal, and if so, store where we
			 * would end up. Otherwise, just move regularly. */
			if let Some(&target) = stitched.get(&original) {
				position = target.position;
				direction = target.direction;
			} else {
				position += direction;
			}

			if index(&map, position) == OPEN {
				Placement {
					position,
					direction,
				}
			} else {
				original
			}
		},
	);
	println!("Part 2: {}", password(part2));

	Ok(())
}

fn solve<'a>(
	start: IVec2D,
	instructions: impl IntoIterator<Item = &'a Instruction>,
	mover: impl Fn(Placement) -> Placement,
) -> Placement {
	let mut position = start;
	let mut direction = RIGHT;

	for instruction in instructions.into_iter() {
		match instruction {
			Instruction::Move(n) => {
				for _ in 0..*n {
					Placement {
						position,
						direction,
					} = mover(Placement {
						position,
						direction,
					});
				}
			}
			Instruction::RotateCw => direction = direction.rotate_ccw(),
			Instruction::RotateCcw => direction = direction.rotate_cw(),
		}
	}

	Placement {
		position,
		direction,
	}
}

#[rustfmt::skip]
/// The cube is stitched into "portals": a mapping from a position and a
/// direction, and the position and direction you will end up at if you move
/// from the position into the direction.
/// For instance, if you are at a position at the edge of a face, and are facing
/// towards another face, the mapping will return the position and direction
/// you would get if you moved from this face to the other.
fn stitch_cube(map: &Map, face_size: usize) -> HashMap<Placement, Placement> {
	let mut result = HashMap::new();

	/* Based on this input structure:
	 *   0011
	 *   0011
	 *   22
	 *   22
	 * 3344
	 * 3344
	 * 55
	 * 55
	 */
	let faces = [
		face(1, 0, map, face_size),
		face(2, 0, map, face_size),
		face(1, 1, map, face_size),
		face(0, 2, map, face_size),
		face(1, 2, map, face_size),
		face(0, 3, map, face_size),
	].map(Option::unwrap);

	/* Implemented with a macro instead of a closure, because closures can't have
	 * generic or impl parameters, which means I can't pass iterators to it. */
	macro_rules! connect {
		($ps1: expr, $d1: expr, $ps2: expr, $d2: expr) => {
			for (p1, p2) in $ps1.zip($ps2) {
				result.insert(
					Placement { position: *p1, direction: $d1 },
					Placement { position: *p2, direction: $d2 },
				);
				result.insert(
					Placement { position: *p2, direction: $d2.flip() },
					Placement { position: *p1, direction: $d1.flip() },
				);
			}
		};
	}

	// Horizontal alignment
	connect!(faces[0].last_row(), DOWN, faces[2].row(0), DOWN);
	connect!(faces[2].last_row(), DOWN, faces[4].row(0), DOWN);
	connect!(faces[3].last_row(), DOWN, faces[5].row(0), DOWN);

	// Vertical adjacent
	connect!(faces[0].last_column(), RIGHT, faces[1].column(0), RIGHT);
	connect!(faces[3].last_column(), RIGHT, faces[4].column(0), RIGHT);

	// Inner corners
	connect!(faces[1].last_row(), DOWN, faces[2].last_column(), LEFT);
	connect!(faces[2].column(0), LEFT, faces[3].row(0), DOWN);
	connect!(faces[4].last_row(), DOWN, faces[5].last_column(), LEFT);

	// Vertical but not adjacent
	connect!(faces[0].column(0), LEFT, faces[3].column(0).rev(), RIGHT);
	connect!(faces[1].last_column(), RIGHT, faces[4].last_column().rev(), LEFT);

	// Other
	connect!(faces[0].row(0), UP, faces[5].column(0), RIGHT);
	connect!(faces[1].row(0), UP, faces[5].last_row(), UP);

	result
}

fn face(x: usize, y: usize, map: &Map, face_size: usize) -> Option<Grid2D<IVec2D>> {
	let has_face = map
		.get(y * face_size)
		.and_then(|row| row.get(x * face_size).copied())
		.flatten();
	has_face?;

	let x = x as i32;
	let y = y as i32;
	let face_size = face_size as i32;

	let x_range = (x * face_size)..((x + 1) * face_size);
	let y_range = (y * face_size)..((y + 1) * face_size);
	let coordinates = y_range.flat_map(move |y| {
		x_range
			.clone()
			.map(move |x| IVec2D(x, y))
	});
	Some(Grid2D::from_values(
		coordinates.collect(),
		face_size as usize,
	))
}

fn password(
	Placement {
		position,
		direction,
	}: Placement,
) -> i32 {
	1000 * (position.1 + 1)
		+ (position.0 + 1) * 4
		+ match direction {
			RIGHT => 0,
			DOWN => 1,
			LEFT => 2,
			UP => 3,
			_ => panic!(),
		}
}

fn index(map: &Map, coord: IVec2D) -> Cell {
	if coord.0 < 0 || coord.1 < 0 {
		return None;
	}

	map.get(coord.1 as usize)
		.and_then(|line| line.get(coord.0 as usize))
		.copied()
		.flatten()
}

fn parse_map<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<Vec<Cell>> {
	lines
		.into_iter()
		.map(|line| {
			line.as_bytes()
				.iter()
				.map(|b| match b {
					b' ' => WRAP,
					b'.' => OPEN,
					b'#' => WALL,
					_ => panic!("Unexpected character {}", *b as char),
				})
				.collect()
		})
		.collect()
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
	let mut iter = line.bytes().peekable();
	iter::from_fn(move || {
		let b = iter.next()?;
		Some(match b {
			b'R' => Instruction::RotateCw,
			b'L' => Instruction::RotateCcw,
			_ if b.is_ascii_digit() => {
				let mut v = (b - b'0') as usize;
				while let Some(b) = iter.next_if(|b| b.is_ascii_digit()) {
					v = v * 10 + (b - b'0') as usize;
				}
				Instruction::Move(v)
			}
			_ => panic!(),
		})
	})
	.collect()
}

type Cell = Option<bool>;
const WRAP: Cell = None;
const OPEN: Cell = Some(true);
const WALL: Cell = Some(false);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
	Move(usize),
	RotateCw,
	RotateCcw,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[rustfmt::skip]
	fn test_stitching() {
		const MAP: &str = "
  ....
  ....
  ..
  ..
....
....
..
..
";
		let map = parse_map(MAP.lines().skip(1).take(8));
		let stitched = stitch_cube(&map, 2);

		// Horizontal adjacent edges
		assert_eq!(stitched.get(&Placement { position: IVec2D(2, 1), direction: DOWN }),
		                   Some(&Placement { position: IVec2D(2, 2), direction: DOWN }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(2, 2), direction: UP }),
		                   Some(&Placement { position: IVec2D(2, 1), direction: UP }));

		assert_eq!(stitched.get(&Placement { position: IVec2D(2, 3), direction: DOWN }),
		                   Some(&Placement { position: IVec2D(2, 4), direction: DOWN }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(2, 4), direction: UP }),
		                   Some(&Placement { position: IVec2D(2, 3), direction: UP }));

		assert_eq!(stitched.get(&Placement { position: IVec2D(0, 5), direction: DOWN }),
		                   Some(&Placement { position: IVec2D(0, 6), direction: DOWN }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(0, 6), direction: UP }),
		                   Some(&Placement { position: IVec2D(0, 5), direction: UP }));

		// Vertical adjacent edges
		assert_eq!(stitched.get(&Placement { position: IVec2D(3, 1), direction: RIGHT }),
		                   Some(&Placement { position: IVec2D(4, 1), direction: RIGHT }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(4, 1), direction: LEFT }),
		                   Some(&Placement { position: IVec2D(3, 1), direction: LEFT }));

		assert_eq!(stitched.get(&Placement { position: IVec2D(1, 5), direction: RIGHT }),
		                   Some(&Placement { position: IVec2D(2, 5), direction: RIGHT }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(2, 5), direction: LEFT }),
		                   Some(&Placement { position: IVec2D(1, 5), direction: LEFT }));

		// Inner corners
		assert_eq!(stitched.get(&Placement { position: IVec2D(4, 1), direction: DOWN }),
		                   Some(&Placement { position: IVec2D(3, 2), direction: LEFT }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(3, 3), direction: RIGHT }),
		                   Some(&Placement { position: IVec2D(5, 1), direction: UP }));

		assert_eq!(stitched.get(&Placement { position: IVec2D(2, 2), direction: LEFT }),
		                   Some(&Placement { position: IVec2D(0, 4), direction: DOWN }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(1, 4), direction: UP }),
		                   Some(&Placement { position: IVec2D(2, 3), direction: RIGHT }));

		assert_eq!(stitched.get(&Placement { position: IVec2D(2, 5), direction: DOWN }),
		                   Some(&Placement { position: IVec2D(1, 6), direction: LEFT }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(1, 7), direction: RIGHT }),
		                   Some(&Placement { position: IVec2D(3, 5), direction: UP }));

		// Both vertical but not touching
		assert_eq!(stitched.get(&Placement { position: IVec2D(2, 0), direction: LEFT }),
		                   Some(&Placement { position: IVec2D(0, 5), direction: RIGHT }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(0, 4), direction: LEFT }),
		                   Some(&Placement { position: IVec2D(2, 1), direction: RIGHT }));

		assert_eq!(stitched.get(&Placement { position: IVec2D(5, 0), direction: RIGHT }),
		                   Some(&Placement { position: IVec2D(3, 5), direction: LEFT }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(3, 4), direction: RIGHT }),
		                   Some(&Placement { position: IVec2D(5, 1), direction: LEFT }));

		// Other
		assert_eq!(stitched.get(&Placement { position: IVec2D(2, 0), direction: UP }),
		                   Some(&Placement { position: IVec2D(0, 6), direction: RIGHT }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(0, 7), direction: LEFT }),
		                   Some(&Placement { position: IVec2D(3, 0), direction: DOWN }));

		assert_eq!(stitched.get(&Placement { position: IVec2D(4, 0), direction: UP }),
		                   Some(&Placement { position: IVec2D(0, 7), direction: UP }));
		assert_eq!(stitched.get(&Placement { position: IVec2D(1, 7), direction: DOWN }),
		                   Some(&Placement { position: IVec2D(5, 0), direction: DOWN }));

		assert_eq!(12 * 2 * 2, stitched.len());
	}
}
