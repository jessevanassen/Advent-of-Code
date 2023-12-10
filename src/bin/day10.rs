use std::io::stdin;

use aoc2023::{
	grid2d::{Grid2D, Grid2DGet as _},
	vector2d::Vector2D,
};

const UP: Vector2D = Vector2D(0, -1);
const RIGHT: Vector2D = Vector2D(1, 0);
const DOWN: Vector2D = Vector2D(0, 1);
const LEFT: Vector2D = Vector2D(-1, 0);

const DIRECTIONS: [Vector2D; 4] = [UP, RIGHT, DOWN, LEFT];

fn main() {
	let mut input: Grid2D<u8> = stdin()
		.lines()
		.flatten()
		.map(|x| x.bytes().collect::<Vec<_>>())
		.collect();

	let start = input
		.enumerate()
		.find_map(|(index, symbol)| (*symbol == b'S').then_some(index))
		.expect("Start position in input");

	input[start] = reconstruct_start(&input, start.into());

	let main_loop = find_main_loop(&input, start.into());

	println!(
		"Part 1: {}",
		main_loop.enumerate().filter(|(_, v)| **v).count() / 2
	);

	let mut enclosed_positions = 0;

	// Walk over all positions that aren't part of the main loop
	for ((x, y), _) in main_loop.enumerate().filter(|(_, is_pipe)| !**is_pipe) {
		let mut pipes_hit = 0;

		// Cast a ray to the right from the current position
		let ray = x..main_loop.width();
		let mut pipes = ray
			// Only include pipes on the ray that are part of the main loop
			.filter(|&x| main_loop[(x, y)])
			// Transform to the actual pipes
			.map(|x| input[(x, y)])
			// Ignore the horizontal pipes
			.filter(|c| *c != b'-');

		while let Some(symbol) = pipes.next() {
			if symbol == b'|' {
				pipes_hit += 1;
				continue;
			}

			let next_symbol = pipes.next().expect("Matching corner");

			match (symbol, next_symbol) {
				// The pair of these corners effectively acts as a single wall
				(b'F', b'J') | (b'L', b'7') => pipes_hit += 1,

				// These corners cancel each other out
				(b'F', b'7') | (b'L', b'J') => {}

				_ => panic!("Mismatched corner pair"),
			}
		}

		let is_enclosed = pipes_hit % 2 != 0;
		if is_enclosed {
			enclosed_positions += 1;
		}
	}

	println!("Part 2: {enclosed_positions}");
}

fn reconstruct_start(input: &Grid2D<u8>, start: Vector2D) -> u8 {
	let [top_connected, right_connected, bottom_connected, left_connected] =
		[UP, RIGHT, DOWN, LEFT].map(|direction| {
			let location = start + direction;
			/* If we can move to this location, and can then move to another position with the current
			 * direction, the start has to be connected to this pipe. */
			input
				.get(location)
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

fn find_main_loop(input: &Grid2D<u8>, start: Vector2D) -> Grid2D<bool> {
	let mut main_loop: Grid2D<bool> = Grid2D::with_size(input.width(), input.height());
	main_loop[start] = true;

	let mut position = start;

	// Pick the first direction we can move to
	let mut direction = DIRECTIONS
		.into_iter()
		.find(|&direction| {
			let symbol = input[start];
			adjust_course(direction, symbol).is_some()
		})
		.expect("Start to be connected to a pipe");

	loop {
		position = position + direction;

		if main_loop[position] {
			break;
		}

		main_loop[position] = true;
		direction = adjust_course(direction, input[position]).unwrap();
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
