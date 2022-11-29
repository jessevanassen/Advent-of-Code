use std::{
	fmt::Display,
	io::stdin,
	ops::{Add, AddAssign, Range},
};

use aoc2022::{extensions::MinMaxExt, vec::IVec2D};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Shape(
	/* Cheat: in case of shape with 4 squares, add a duplicate position so a
	 * shape can be stack-allocated and cheaply copied. */
	[IVec2D; 5],
);

impl Shape {
	fn max_y(&self) -> i32 {
		self.0
			.iter()
			.map(|p| p.1)
			.max()
			.unwrap()
	}

	fn min_max_x(&self) -> (i32, i32) {
		self.0
			.iter()
			.map(|p| p.0)
			.min_max()
			.unwrap()
	}
}

impl Add<IVec2D> for &Shape {
	type Output = Shape;

	fn add(self, rhs: IVec2D) -> Self::Output {
		Shape(self.0.map(|c| c + rhs))
	}
}

impl AddAssign<IVec2D> for Shape {
	fn add_assign(&mut self, rhs: IVec2D) {
		for mut i in self.0.iter_mut() {
			i += rhs;
		}
	}
}

const SHAPES: [Shape; 5] = [
	// Horizontal line
	Shape([
		IVec2D(2, 0),
		IVec2D(3, 0),
		IVec2D(4, 0),
		IVec2D(5, 0),
		IVec2D(5, 0),
	]),
	// Star
	Shape([
		IVec2D(3, 0),
		IVec2D(2, 1),
		IVec2D(3, 1),
		IVec2D(4, 1),
		IVec2D(3, 2),
	]),
	// J
	Shape([
		IVec2D(2, 0),
		IVec2D(3, 0),
		IVec2D(4, 0),
		IVec2D(4, 1),
		IVec2D(4, 2),
	]),
	// Vertical line
	Shape([
		IVec2D(2, 0),
		IVec2D(2, 1),
		IVec2D(2, 2),
		IVec2D(2, 3),
		IVec2D(2, 3),
	]),
	// Square
	Shape([
		IVec2D(2, 0),
		IVec2D(3, 0),
		IVec2D(2, 1),
		IVec2D(3, 1),
		IVec2D(3, 1),
	]),
];

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Field {
	pieces: Vec<u8>,
	piece_count: usize,
}

impl Field {
	pub fn intersects(&self, shape: &Shape) -> bool {
		shape.0.iter().any(|coord| {
			let x = coord.0 as u8;
			let y = coord.1 as usize;
			self.pieces
				.get(y)
				.map(|row| (1 << x) & row > 0)
				.unwrap_or(false)
		})
	}

	pub fn insert(&mut self, shape: &Shape) {
		self.piece_count += 1;

		let minimum_required_size = Ord::max(self.pieces.len(), shape.max_y() as usize + 1);
		self.pieces
			.resize(minimum_required_size, 0);

		for coord in shape.0 {
			let x = coord.0 as u8;
			let y = coord.1 as usize;
			self.pieces[y] |= 1 << x;
		}
	}

	pub fn height(&self) -> usize {
		self.pieces.len()
	}

	fn has_solid_top_layer(&self) -> bool {
		if let Some(layer) = self.pieces.last() {
			layer == &0b0111_1111
		} else {
			true
		}
	}
}

impl Display for Field {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}", "-".repeat(WIDTH + 2))?;
		for line in self.pieces.iter().rev() {
			write!(f, "|")?;
			for i in 0..WIDTH {
				write!(f, "{}", if (1 << i) & line > 0 { '#' } else { ' ' })?;
			}
			writeln!(f, "|")?;
		}
		write!(f, "{}", "-".repeat(WIDTH + 2))?;
		Ok(())
	}
}

const WIDTH: usize = 7;
const DROP_HEIGHT: usize = 3;

fn main() {
	let directions = parse_input(
		&stdin()
			.lines()
			.flatten()
			.next()
			.unwrap(),
	);

	println!("Part 1: {}", play(2022, &directions));
	println!("Part 2: {}", play(1_000_000_000_000, &directions));
}

fn play(limit: usize, directions: &[IVec2D]) -> usize {
	let mut direction_index = 0;
	let mut shape_index = 0;

	let mut stabilized_pieces: Field = Field::default();

	let mut height_offset = 0;

	#[derive(Debug)]
	struct SolidLayer {
		height: usize,
		piece_count: usize,
		shape_index: usize,
		direction_index: usize,
	}

	let mut last_solid_layer: Option<SolidLayer> = None;

	while shape_index < limit {
		let shape = &SHAPES[shape_index % SHAPES.len()];
		shape_index += 1;

		let start_y = stabilized_pieces.height() + DROP_HEIGHT;

		let mut shape = shape + IVec2D(0, start_y as _);

		for y in 0..=start_y {
			let direction = &directions[direction_index % directions.len()];
			direction_index += 1;

			/* Move shape left or right, if possible. */
			let moved_shape = &shape + *direction;
			if is_in_field_range(&moved_shape) && !stabilized_pieces.intersects(&moved_shape) {
				shape = moved_shape;
			}

			/* Move shape down, but only if it is not the last turn, because in
			 * the last turn the piece can still move sideways while already
			 * touching the shape below, but it can't move further down. */
			if y < start_y {
				let moved_shape = &shape + IVec2D::DOWN;
				if !stabilized_pieces.intersects(&moved_shape) {
					shape = moved_shape;
				} else {
					/* If the moved piece intersected with something on the way,
					 * it "has landed" and can't move further. */
					break;
				}
			}
		}

		stabilized_pieces.insert(&shape);

		if stabilized_pieces.has_solid_top_layer() {
			let current = SolidLayer {
				/* The indices have been proactively incremented so they are
				 * already pointing to the next indices, but for the layer
				 * calculation, the old values are needed. */
				shape_index: shape_index - 1,
				direction_index: direction_index - 1,
				height: stabilized_pieces.height(),
				piece_count: stabilized_pieces.piece_count,
			};

			if let Some(previous) = last_solid_layer {
				let same_shape_index =
					previous.shape_index % SHAPES.len() == current.shape_index % SHAPES.len();
				let same_direction_index = previous.direction_index % directions.len()
					== current.direction_index % directions.len();

				if same_shape_index && same_direction_index {
					let directions_per_cycle = current.direction_index - previous.direction_index;
					let shapes_per_cycle = current.piece_count - previous.piece_count;
					let height_per_cycle = current.height - previous.height;
					let cycles_to_skip = (limit - shape_index - 1) / shapes_per_cycle;

					shape_index += shapes_per_cycle * cycles_to_skip;
					direction_index += directions_per_cycle * cycles_to_skip;
					height_offset += height_per_cycle * cycles_to_skip;
				}
			}

			last_solid_layer = Some(current);
		}
	}

	height_offset + stabilized_pieces.height()
}

fn is_in_field_range(shape: &Shape) -> bool {
	const X_RANGE: Range<i32> = 0..(WIDTH as i32);
	let (min_x, max_x) = &shape.min_max_x();
	X_RANGE.contains(min_x) && X_RANGE.contains(max_x)
}

fn parse_input(line: &str) -> Vec<IVec2D> {
	line.as_bytes()
		.iter()
		.map(move |b| match b {
			b'<' => IVec2D::LEFT,
			b'>' => IVec2D::RIGHT,
			&other => panic!("Unrecognized direction {}", other as char),
		})
		.collect::<Vec<_>>()
}
