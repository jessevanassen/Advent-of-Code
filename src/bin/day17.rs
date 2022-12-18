use std::{
	collections::HashMap,
	fmt::Display,
	io::stdin,
	ops::{Add, AddAssign, Range, Sub},
};

use aoc2022::{extensions::MinMaxExt, vec::IVec2D, last_n};

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

	fn is_in_field_range(&self) -> bool {
		const X_RANGE: Range<i32> = 0..(Field::WIDTH as i32);
		let (min_x, max_x) = &self.min_max_x();
		X_RANGE.contains(min_x) && X_RANGE.contains(max_x)
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
	const WIDTH: usize = 7;

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

	pub fn last_signature_key(&self) -> Option<RepetitionSignatureKey> {
		last_n(RepetitionSignature::SIGNATURE_LENGTH, &self.pieces).map(|x| x.try_into().unwrap())
	}
}

impl Display for Field {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}", "-".repeat(Field::WIDTH + 2))?;
		for line in self.pieces.iter().rev() {
			write!(f, "|")?;
			for i in 0..Field::WIDTH {
				write!(f, "{}", if (1 << i) & line > 0 { '#' } else { ' ' })?;
			}
			writeln!(f, "|")?;
		}
		write!(f, "{}", "-".repeat(Field::WIDTH + 2))?;
		Ok(())
	}
}

const DROP_HEIGHT: usize = 3;

struct RepetitionSignature {
	height: usize,
	shape_count: usize,
	direction_index: usize,
}

impl RepetitionSignature {
	const SIGNATURE_LENGTH: usize = 16;
	const REQUIRED_REPETITIONS: usize = 4;

	fn equal_indices(&self, other: &Self) -> bool {
		self.direction_index == other.direction_index
			&& self.shape_count % SHAPES.len() == other.shape_count % SHAPES.len()
	}
}

impl Sub for &RepetitionSignature {
	type Output = RepetitionSignature;

	fn sub(self, rhs: Self) -> Self::Output {
		RepetitionSignature {
			height: self.height - rhs.height,
			shape_count: self.shape_count - rhs.shape_count,
			direction_index: self.direction_index - rhs.direction_index,
		}
	}
}

type RepetitionSignatureKey = [u8; RepetitionSignature::SIGNATURE_LENGTH];

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
	let mut directions = directions
		.iter()
		.enumerate()
		.cycle()
		.peekable();

	let mut shape_count = 0;
	let mut stabilized_pieces: Field = Field::default();
	let mut height_offset = 0;
	let mut repetitions: HashMap<RepetitionSignatureKey, Vec<RepetitionSignature>> = HashMap::new();

	while shape_count < limit {
		let shape = &SHAPES[shape_count % SHAPES.len()];
		shape_count += 1;

		let start_y = stabilized_pieces.height() + DROP_HEIGHT;

		let mut shape = shape + IVec2D(0, start_y as _);

		for y in 0..=start_y {
			let direction = directions.next().unwrap().1;

			/* Move shape left or right, if possible. */
			let moved_shape = &shape + *direction;
			if moved_shape.is_in_field_range() && !stabilized_pieces.intersects(&moved_shape) {
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

		if let Some(signature_key) = stabilized_pieces.last_signature_key() {
			let entry = repetitions
				.entry(signature_key)
				.or_default();
			let signature = RepetitionSignature {
				height: stabilized_pieces.height(),
				shape_count,
				direction_index: directions.peek().unwrap().0,
			};
			entry.push(signature);

			if matches!(
				last_n(RepetitionSignature::REQUIRED_REPETITIONS, entry),
				Some(signatures) if signatures.windows(2).all(|xs| xs[0].equal_indices(&xs[1]))
			) {
				let last_two = last_n(2, entry).unwrap();
				let diff = &last_two[1] - &last_two[0];

				let cycles_to_skip = (limit - shape_count) / diff.shape_count;

				shape_count += diff.shape_count * cycles_to_skip;
				height_offset += diff.height * cycles_to_skip;
			}
		}
	}

	height_offset + stabilized_pieces.height()
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
