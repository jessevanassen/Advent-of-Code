use std::{collections::HashMap, io::stdin};

fn main() {
	let mut lines = stdin().lines().flatten();

	let instructions = lines
		.next()
		.unwrap()
		.bytes()
		.map(|lr| match lr {
			b'L' => 0,
			b'R' => 1,
			_ => panic!("Invalid input"),
		})
		.collect::<Vec<_>>();

	let map_lines = lines.skip(1).collect::<Vec<_>>();

	let positions = map_lines.iter().map(|line| &line[0..3]).collect::<Vec<_>>();

	let position_indices = positions
		.iter()
		.copied()
		.zip(0..)
		.collect::<HashMap<_, _>>();

	let map = map_lines
		.iter()
		.map(|line| {
			let left = &line[7..10];
			let right = &line[12..15];

			[position_indices[left], position_indices[right]]
		})
		.collect::<Vec<_>>();

	let find_travel_time = |start_position: usize, is_end_position: fn(&str) -> bool| {
		instructions
			.iter()
			.cycle()
			.scan(start_position, |position, instruction| {
				*position = map[*position][*instruction];
				Some(*position)
			})
			.take_while(|position| !is_end_position(positions[*position]))
			.count() + 1 // Count doesn't include the end position
	};

	println!("Part 1: {}", {
		let predicate = |position: &str| position == "ZZZ";
		find_travel_time(position_indices["AAA"], predicate)
	});

	let start_positions = position_indices
		.iter()
		.filter_map(|(key, value)| key.ends_with('A').then_some(*value));
	let travel_times = start_positions.map(|position| {
		let predicate = |position: &str| position.ends_with('Z');
		find_travel_time(position, predicate)
	});

	println!(
		"Part 2: {}",
		travel_times.reduce(lcm).unwrap()
	);
}

fn lcm(x: usize, y: usize) -> usize {
	let min = x.min(y);
	let max = x.max(y);

	(max..).step_by(max).find(|x| x % min == 0).unwrap()
}
