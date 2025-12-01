use std::{io::stdin, iter};

const DIAL_START: i32 = 50;

fn main() {
	let input = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| {
			let direction = if line.as_bytes()[0] == b'L' { -1 } else { 1 };
			let value: i32 = line[1..].parse().unwrap();
			direction * value
		})
		.collect::<Vec<_>>();

	println!("Part 1: {}", count_points_at_zero(input.iter().copied()),);
	println!(
		"Part 2: {}",
		count_points_at_zero(input.iter().flat_map(|&rotations| {
			let element = rotations.signum();
			let count = rotations.unsigned_abs() as usize;
			iter::repeat_n(element, count)
		})),
	);
}

fn count_points_at_zero(rotations: impl IntoIterator<Item = i32>) -> usize {
	let mut acc = DIAL_START;
	let mut points_at_zero_count = 0;

	for rotation in rotations {
		acc = (acc + rotation).rem_euclid(100);

		if acc == 0 {
			points_at_zero_count += 1;
		}
	}

	points_at_zero_count
}
