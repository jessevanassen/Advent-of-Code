use std::{io::stdin, iter};

use aoc2022::extensions::Pipe;

const INITIAL_STATE: i32 = 1;

fn main() {
	let values = stdin()
		.lines()
		.flatten()
		.flat_map(|line| process_line(&line))
		.scan(INITIAL_STATE, |acc, to_add| {
			*acc += to_add;
			Some(*acc)
		})
		.map_self(|iter|
			/* Currently, the iterator contains the values _during_ the last
			 * cycle they are modified, not _after_. To fix this, insert an
			 * additional INITIAL_STATE at the beginning to offset the values by
			 * one. */
			iter::once(INITIAL_STATE).chain(iter))
		.collect::<Vec<_>>();

	let part1: i32 = values
		.iter()
		.map_self(|iter|
			/* For part 1, the cycles are considered to be one-based, whereas
			 * indices are 0-based. By inserting an INITIAL_STATE at the
			 * beginning of the iterator, the indices shift one over and become
			 * correct cycles. */
			iter::once(&INITIAL_STATE).chain(iter))
		.enumerate()
		.skip(20)
		.step_by(40)
		.map(|(i, v)| i as i32 * v)
		.sum();
	println!("Part 1: {part1}");

	let characters = values
		.iter()
		.enumerate()
		.map(|(cycle, value)| {
			let sprite = (value - 1)..=(value + 1);
			let screen_pixel = (cycle as i32) % 40;
			sprite.contains(&screen_pixel)
		})
		.map(|bit| if bit { '#' } else { '.' });

	print!("Part 2:");
	for (i, c) in characters.enumerate() {
		if i % 40 == 0 {
			println!()
		}
		print!("{c}");
	}
}

fn process_line(line: &str) -> Box<dyn Iterator<Item = i32>> {
	if line == "noop" {
		return Box::new(iter::once(0));
	}

	if line.starts_with("addx") {
		let amount: i32 = line[("addx ".len())..].parse().unwrap();
		return Box::new(iter::once(0).chain(iter::once(amount)));
	}

	panic!("Wrong input format");
}
