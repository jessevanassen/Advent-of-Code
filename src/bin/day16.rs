use std::io::stdin;

use aoc2023::{
	grid2d::{Grid2D, Grid2DGet as _},
	vector2d::Vector2D,
};

const RIGHT: Vector2D = Vector2D(1, 0);
const LEFT: Vector2D = Vector2D(-1, 0);

// Inverted vertical axis
const UP: Vector2D = Vector2D(0, -1);
const DOWN: Vector2D = Vector2D(0, 1);

fn main() {
	let input: Grid2D<u8> = stdin()
		.lines()
		.flatten()
		.map(|line| line.bytes().collect::<Vec<_>>())
		.collect();

	let part1 = find_energized_tiles((Vector2D(0, 0), RIGHT), &input);
	println!("Part 1: {part1}");

	let top = (0..input.width()).map(|x| (Vector2D::from((x, 0)), DOWN));
	let left = (0..input.height()).map(|y| (Vector2D::from((0, y)), RIGHT));
	let bottom = (0..input.width()).map(|x| (Vector2D::from((x, input.height() - 1)), UP));
	let right = (0..input.height()).map(|y| (Vector2D::from((input.width() - 1, y)), LEFT));
	let part2 = top
		.chain(left)
		.chain(bottom)
		.chain(right)
		.map(|initial| find_energized_tiles(initial, &input))
		.max()
		.unwrap();
	println!("Part 2: {part2}");
}

fn find_energized_tiles(initial: (Vector2D, Vector2D), input: &Grid2D<u8>) -> usize {
	let mut queue = vec![initial];

	let mut seen: Grid2D<u8> = Grid2D::with_size(input.width(), input.height());

	while let Some((position, direction)) = queue.pop() {
		let direction_bitflag = match direction {
			UP => 1 << 0,
			RIGHT => 1 << 1,
			DOWN => 1 << 2,
			LEFT => 1 << 3,
			_ => panic!(),
		};

		if seen
			.get(position)
			.map(|d| *d & direction_bitflag != 0)
			.unwrap_or(true)
		{
			// Either the position is out of range, or already encountered.
			continue;
		} else {
			seen[position] |= direction_bitflag;
		}

		let mut move_towards = |direction: Vector2D| queue.push((position + direction, direction));

		match (Axis::from(direction), input[position]) {
			(_, b'.') | (Axis::Horizontal, b'-') | (Axis::Vertical, b'|') => {
				move_towards(direction);
			}

			(Axis::Horizontal, b'|') | (Axis::Vertical, b'-') => {
				for direction in split(direction) {
					move_towards(direction);
				}
			}

			(Axis::Horizontal, b'/') | (Axis::Vertical, b'\\') => {
				let direction = rotate_ccw(direction);
				move_towards(direction);
			}

			(Axis::Horizontal, b'\\') | (Axis::Vertical, b'/') => {
				let direction = rotate_cw(direction);
				move_towards(direction)
			}

			(_, symbol) => panic!("Unexpected symbol {}", symbol as char),
		}
	}

	seen.values().filter(|it| **it != 0).count()
}

fn rotate_cw(vector: Vector2D) -> Vector2D {
	// Inverted Y-axis
	vector.rotate_ccw()
}

fn rotate_ccw(vector: Vector2D) -> Vector2D {
	// Inverted Y-axis
	vector.rotate_cw()
}

fn split(vec: Vector2D) -> [Vector2D; 2] {
	[vec.rotate_cw(), vec.rotate_ccw()]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
	Horizontal,
	Vertical,
}

impl From<Vector2D> for Axis {
	fn from(Vector2D(x, y): Vector2D) -> Self {
		match (x, y) {
			(0, 0) => panic!(),
			(_, 0) => Self::Horizontal,
			(0, _) => Self::Vertical,
			_ => panic!(),
		}
	}
}
