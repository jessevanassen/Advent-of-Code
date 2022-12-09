use std::{collections::HashSet, io::stdin, iter};

use aoc2022::vec::IVec2D;

fn main() {
	let steps = stdin()
		.lines()
		.flatten()
		.flat_map(parse_input_line)
		.collect::<Vec<_>>();

	println!("Part 1: {}", unique_tail_positions::<2>(&steps));
	println!("Part 2: {}", unique_tail_positions::<10>(&steps));
}

fn unique_tail_positions<'a, const ROPE_LENGTH: usize>(
	steps: impl IntoIterator<Item = &'a IVec2D>,
) -> usize {
	steps
		.into_iter()
		.scan([IVec2D::default(); ROPE_LENGTH], |rope, &direction| {
			move_rope(rope, direction);
			Some(rope[rope.len() - 1])
		})
		.collect::<HashSet<_>>()
		.len()
}

fn move_rope(rope: &mut [IVec2D], direction: IVec2D) {
	fn next_tail_position(previous_tail: IVec2D, head: IVec2D) -> IVec2D {
		if previous_tail.chessboard_distance(head) >= 2 {
			previous_tail + previous_tail.direction(head)
		} else {
			previous_tail
		}
	}

	rope[0] = rope[0] + direction;

	for i in 1..rope.len() {
		rope[i] = next_tail_position(rope[i], rope[i - 1]);
	}
}

fn parse_input_line(line: impl AsRef<str>) -> impl Iterator<Item = IVec2D> {
	let (direction, amount) = line
		.as_ref()
		.split_once(" ")
		.expect("Expected input to be in the '<direction> <amount>' format");
	let direction = match direction {
		"U" => IVec2D::UP,
		"D" => IVec2D::DOWN,
		"L" => IVec2D::LEFT,
		"R" => IVec2D::RIGHT,
		other => panic!("Invalid direction {other}"),
	};
	let amount: usize = amount.parse().unwrap();
	iter::repeat(direction).take(amount)
}
