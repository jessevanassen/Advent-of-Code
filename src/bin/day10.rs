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

	let height = input.len();
	let width = input[0].len();

	let start = enumerate_positions(&input)
		.find_map(|(position, c)| (c == b'S').then_some(position))
		.expect("Expected start position in input");

	*input.index_mut_by_vector2d(start) = reconstruct_start(&input, start);

	let main_loop = find_main_loop(&input, start);

	println!(
		"Part 1: {}",
		main_loop.iter().flatten().filter(|x| **x).count() / 2
	);


	// Enlarge the map, so two touching pipes get a gap. This way, we can flood-fill the map and it
	// will penetrate the gaps.
	let mut big_input = vec![vec![false; width * 2 + 1]; height * 2 + 1];
	for (y, row) in input.iter().enumerate() {
		for (x, v) in row.iter().enumerate() {
			if !*main_loop.index_by_vector2d(Vector2D::from((x, y))) {
				continue;
			}

			big_input[1 + y * 2][1 + x * 2] = true;

			let right_connected = matches!(*v, b'S' | b'-' | b'F' | b'L');
			big_input[1 + y * 2][1 + x * 2 + 1] = right_connected;

			let bottom_connected = matches!(*v, b'S' | b'|' | b'7' | b'F');
			big_input[1 + y * 2 + 1][1 + x * 2] = bottom_connected;
		}
	}

	let flooded = flood_fill(&big_input, Vector2D(0, 0));

	let open_positions =
		enumerate_positions(&main_loop).filter_map(|(position, v)| (!v).then_some(position));
	let part2 = open_positions
		.filter(|Vector2D(x, y)| {
			let position = Vector2D(1 + x * 2, 1 + y * 2);
			!*flooded.index_by_vector2d(position)
		})
		.count();
	println!("Part 2: {part2}");
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

fn flood_fill(map: &Vec<Vec<bool>>, start: Vector2D) -> Vec<Vec<bool>> {
	let mut flooded: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
	let mut visited: Vec<Vec<bool>> = flooded.clone();
	let mut queue = vec![start];

	while let Some(next) = queue.pop() {
		let around = DIRECTIONS
			.map(|it| next + it)
			.into_iter()
			.filter(|it| map.get_by_vector2d(*it).is_some());

		for pos in around {
			if *visited.index_by_vector2d(pos) {
				continue;
			}
			*visited.index_mut_by_vector2d(pos) = true;

			let open = !*map.index_by_vector2d(pos);

			if open {
				*flooded.index_mut_by_vector2d(pos) = true;
				queue.push(pos);
			}
		}
	}

	flooded
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
