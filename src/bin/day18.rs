use std::io::stdin;

use aoc2023::vector2d::Vector2D;
use itertools::Itertools;

const RIGHT: Vector2D = Vector2D(1, 0);
const LEFT: Vector2D = Vector2D(-1, 0);
// Inverted Y-axis
const UP: Vector2D = Vector2D(0, -1);
const DOWN: Vector2D = Vector2D(0, 1);

type Instruction = (Vector2D, usize);

fn main() {
	let input: Vec<(Instruction, Instruction)> = stdin()
		.lines()
		.flatten()
		.map(|line| {
			let mut line = line.split_whitespace();

			let first_instruction = {
				let direction = match line.next().unwrap() {
					"R" => RIGHT,
					"D" => DOWN,
					"L" => LEFT,
					"U" => UP,
					_ => panic!(),
				};
				let amount = line.next().unwrap().parse::<usize>().unwrap();
				(direction, amount)
			};

			let second_instruction = {
				let hex = &line.next().unwrap()[2..8];

				let amount = usize::from_str_radix(&hex[..5], 16).unwrap();
				let direction = match hex.as_bytes()[5] {
					b'0' => RIGHT,
					b'1' => DOWN,
					b'2' => LEFT,
					b'3' => UP,
					_ => panic!(),
				};
				(direction, amount)
			};

			(first_instruction, second_instruction)
		})
		.collect();

	println!(
		"Part 1: {}",
		calculate_area(input.iter().map(|(instruction, _)| instruction))
	);
	println!(
		"Part 2: {}",
		calculate_area(input.iter().map(|(_, instruction)| instruction))
	);
}

fn calculate_area<'a>(instructions: impl Iterator<Item = &'a Instruction> + 'a) -> usize {
	let positions = instructions
		.scan(Vector2D::default(), |pos, (direction, amount)| {
			*pos = *pos + *direction * *amount as isize;
			Some(*pos)
		})
		.collect::<Vec<_>>();

	let edges = || {
		positions
			.iter()
			.cycle()
			.take(positions.len() + 1)
			.tuple_windows()
	};

	let max_y = positions.iter().map(|Vector2D(_, y)| *y).max().unwrap_or(0);
	let area = edges()
		.map(|(Vector2D(x0, y0), Vector2D(x1, _))| {
			let x_diff = x1 - x0;
			let y_diff = max_y - y0;
			x_diff * y_diff
		})
		.sum::<isize>() as usize;

	let border_length = edges()
		.map(|(&a, &b)| a.manhattan_distance_to(b))
		.sum::<usize>();

	area + (border_length / 2) + 1
}
