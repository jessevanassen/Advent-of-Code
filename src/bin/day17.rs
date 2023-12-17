use std::{
	cmp::Reverse,
	collections::{BinaryHeap, HashSet},
	io::stdin,
	u8,
};

use aoc2023::{
	grid2d::{Grid2D, Grid2DGet as _},
	vector2d::Vector2D,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Step(u32, Position);

impl Ord for Step {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.cmp(&other.0)
	}
}
impl PartialOrd for Step {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

type Position = (
	// Location
	Vector2D,
	Direction,
);

type Direction = (
	// Direction
	Vector2D,
	// Steps
	u8,
);

fn main() {
	let input: Grid2D<u8> = stdin()
		.lines()
		.flatten()
		.map(|line| {
			line.bytes()
				.inspect(|b| {
					if !b.is_ascii_digit() {
						panic!("Not a digit");
					}
				})
				.map(|b| b - b'0')
				.collect::<Vec<_>>()
		})
		.collect();

	println!("Part 1: {}", find_path(&input, false));
	println!("Part 2: {}", find_path(&input, true));
}

fn find_path(input: &Grid2D<u8>, ultra: bool) -> u32 {
	let start: Step = Step(0, (Vector2D(0, 0), (Vector2D(1, 0), 0)));
	let target = Vector2D::from((input.width() - 1, input.height() - 1));

	let mut seen: HashSet<Position> = HashSet::new();

	let mut queue: BinaryHeap<Reverse<Step>> = Default::default();
	queue.push(Reverse(start));

	while let Some(Reverse(step @ Step(cost, position @ (location, (_, steps))))) = queue.pop() {
		if location == target && (!ultra || steps >= 4) {
			return cost;
		}

		if seen.contains(&position) {
			continue;
		}
		seen.insert(position);

		for neighbor in neighbors(step, input, ultra) {
			queue.push(Reverse(neighbor));
		}
	}

	panic!("No path found");
}

fn neighbors(
	Step(cost, (location, (direction, steps))): Step,
	input: &Grid2D<u8>,
	ultra: bool,
) -> impl Iterator<Item = Step> {
	let mut options: [Option<Step>; 3] = Default::default();

	if !ultra || steps >= 4 {
		for (i, direction) in [(direction.rotate_cw(), 1), (direction.rotate_ccw(), 1)]
			.into_iter()
			.enumerate()
		{
			let location = location + direction.0;
			options[i] = input
				.get(location)
				.map(|&c| Step(cost + c as u32, (location, direction)));
		}
	}

	if (ultra && steps < 10) || (!ultra && steps < 3) {
		let direction = (direction, steps + 1);
		let location = location + direction.0;
		options[2] = input
			.get(location)
			.map(|&c| Step(cost + c as u32, (location, direction)));
	}

	options.into_iter().flatten()
}
