use std::io::stdin;

use ::aoc2023::Vector2D;

const UP: Vector2D = Vector2D(0, -1);
const RIGHT: Vector2D = Vector2D(1, 0);
const DOWN: Vector2D = Vector2D(0, 1);
const LEFT: Vector2D = Vector2D(-1, 0);

const DIRECTIONS: [Vector2D; 4] = [UP, RIGHT, DOWN, LEFT];

fn main() {
	let mut input = stdin()
		.lines()
		.flatten()
		.map(|x| x.bytes().collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let start = enumerate_positions(&input)
		.find_map(|(position, c)| (c == b'S').then_some(position))
		.expect("Expected start position in input");

	*input.index_mut_by_vector2d(start) = reconstruct_start(&input, start);

	let main_loop = find_main_loop(&input, start);

	println!(
		"Part 1: {}",
		main_loop.iter().flatten().filter(|x| **x).count() / 2
	);

	let mut enclosed_positions = 0;

	// Walk over all positions that aren't part of the main loop
	for (y, row) in main_loop.iter().enumerate() {
		for (x, _) in row.iter().enumerate().filter(|(_, is_pipe)| !**is_pipe) {
			let mut pipes_hit = 0;

			// Cast a ray to the right from the current position
			let ray = x..row.len();
			let mut pipes = ray
				// Only include pipes on the ray that are part of the main loop
				.filter(|&x| row[x])
				// Transform to the actual pipes
				.map(|x| input[y][x])
				// Ignore the horizontal pipes
				.filter(|c| *c != b'-');

			while let Some(symbol) = pipes.next() {
				if symbol == b'|' {
					pipes_hit += 1;
					continue;
				}

				let next_symbol = pipes.next().expect("Unmatched corner");

				match (symbol, next_symbol) {
					// The pair of these corners effectively acts as a single wall
					(b'F', b'J') => pipes_hit += 1,
					(b'L', b'7') => pipes_hit += 1,

					// These corners cancel each other out
					(b'F', b'7') => {}
					(b'L', b'J') => {}

					_ => panic!("Mismatched corner pair"),
				}
			}

			let is_enclosed = pipes_hit % 2 != 0;
			if is_enclosed {
				enclosed_positions += 1;
			}
		}
	}

	println!("Part 2: {enclosed_positions}");
}

fn reconstruct_start(input: &Vec<Vec<u8>>, start: Vector2D) -> u8 {
	let [top_connected, right_connected, bottom_connected, left_connected] =
		[UP, RIGHT, DOWN, LEFT].map(|direction| {
			let location = start + direction;
			input
				.get_by_vector2d(location)
				.and_then(|&symbol| adjust_course(direction, symbol))
				.is_some()
		});

	match (
		top_connected,
		right_connected,
		bottom_connected,
		left_connected,
	) {
		(true, false, true, false) => b'|',
		(false, true, false, true) => b'-',

		(true, true, false, false) => b'L',
		(false, true, true, false) => b'F',
		(false, false, true, true) => b'7',
		(true, false, false, true) => b'J',

		_ => panic!(),
	}
}

fn find_main_loop(input: &Vec<Vec<u8>>, start: Vector2D) -> Vec<Vec<bool>> {
	let mut main_loop: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
	*main_loop.index_mut_by_vector2d(start) = true;

	let mut position = start;
	let mut direction = DIRECTIONS
		.into_iter()
		.find(|&direction| {
			let destination = start + direction;

			input
				.get_by_vector2d(destination)
				.and_then(|&c| adjust_course(direction, c))
				.is_some()
		})
		.expect("Start should be connected to a pipe");

	loop {
		position = position + direction;

		if *main_loop.index_by_vector2d(position) {
			break;
		}

		*main_loop.index_mut_by_vector2d(position) = true;
		direction = adjust_course(direction, *input.index_by_vector2d(position)).unwrap();
	}

	main_loop
}

fn adjust_course(direction: Vector2D, pipe: u8) -> Option<Vector2D> {
	Some(match (pipe, direction) {
		(b'|', UP) => UP,
		(b'7', UP) => LEFT,
		(b'F', UP) => RIGHT,

		(b'-', RIGHT) => RIGHT,
		(b'J', RIGHT) => UP,
		(b'7', RIGHT) => DOWN,

		(b'|', DOWN) => DOWN,
		(b'L', DOWN) => RIGHT,
		(b'J', DOWN) => LEFT,

		(b'-', LEFT) => LEFT,
		(b'F', LEFT) => DOWN,
		(b'L', LEFT) => UP,

		_ => None?,
	})
}

fn enumerate_positions<T: Copy>(items: &Vec<Vec<T>>) -> impl Iterator<Item = (Vector2D, T)> + '_ {
	let height = items.len();
	let width = items.get(0).map(|row| row.len()).unwrap_or(0);

	let xs = 0..width;
	let ys = 0..height;

	ys.flat_map(move |y| {
		xs.clone().map(move |x| {
			let position = Vector2D::from((x, y));
			(position, *items.index_by_vector2d(position))
		})
	})
}

trait Vec2DExt {
	type Output;
	fn get_by_vector2d(&self, index: Vector2D) -> Option<&Self::Output>;
	fn index_by_vector2d(&self, index: Vector2D) -> &Self::Output;
	fn get_mut_by_vector2d(&mut self, index: Vector2D) -> Option<&mut Self::Output>;
	fn index_mut_by_vector2d(&mut self, index: Vector2D) -> &mut Self::Output;
}

impl<T> Vec2DExt for Vec<Vec<T>> {
	type Output = T;

	fn get_by_vector2d(&self, index: Vector2D) -> Option<&Self::Output> {
		if index.0 < 0 || index.1 < 0 {
			return None;
		}

		self.get(index.1 as usize)
			.and_then(|row| row.get(index.0 as usize))
	}

	fn index_by_vector2d(&self, index: Vector2D) -> &Self::Output {
		self.get_by_vector2d(index).unwrap()
	}

	fn get_mut_by_vector2d(&mut self, index: Vector2D) -> Option<&mut Self::Output> {
		if index.0 < 0 || index.1 < 0 {
			return None;
		}

		self.get_mut(index.1 as usize)
			.and_then(|row| row.get_mut(index.0 as usize))
	}

	fn index_mut_by_vector2d(&mut self, index: Vector2D) -> &mut Self::Output {
		self.get_mut_by_vector2d(index).unwrap()
	}
}
