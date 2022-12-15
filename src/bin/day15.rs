use std::{io::stdin, ops::RangeInclusive, str::FromStr};

use aoc2022::{
	extensions::{MinMaxExt, SingleExt},
	vec::IVec2D,
};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sensor {
	position: IVec2D,
	closest_beacon: IVec2D,
}

impl Sensor {
	fn covers(&self, position: IVec2D) -> bool {
		let distance = self
			.position
			.manhattan_distance(position);
		distance <= self.range()
	}

	fn range(&self) -> u32 {
		self.position
			.manhattan_distance(self.closest_beacon)
	}

	/// Returns the coordinates that form the perimeter around the sensor's
	/// range.
	fn perimeter(&self) -> impl Iterator<Item = IVec2D> {
		let IVec2D(x, y) = self.position;
		let range = self.range() as i32;

		let top_left = ((x - range - 1)..=x).zip(y..=(y + range + 1));
		let bottom_right = (x..=(x + range + 1)).zip(((y + range + 1)..=y).rev());
		let bottom_left = ((x - range)..x).zip(y..(y + range + 1));
		let top_right = ((x + 1)..(x + range + 1)).zip(((y - range)..y).rev());

		top_left
			.chain(top_right)
			.chain(bottom_left)
			.chain(bottom_right)
			.map(|(x, y)| IVec2D(x, y))
	}
}

impl FromStr for Sensor {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		lazy_static! {
			static ref RE: Regex = Regex::new(
				r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
			)
			.unwrap();
		}
		let captures = RE.captures(s).ok_or(())?;
		let mut captures = captures
			.iter()
			.skip(1) // Skip the full match, only take the capture groups
			.flatten()
			.map(|m| m.as_str().parse().unwrap());

		let position = IVec2D(captures.next().unwrap(), captures.next().unwrap());
		let closest_beacon = IVec2D(captures.next().unwrap(), captures.next().unwrap());

		Ok(Sensor {
			position,
			closest_beacon,
		})
	}
}

fn main() {
	let input = stdin()
		.lines()
		.flatten()
		.map(|line| line.parse().unwrap())
		.collect::<Vec<Sensor>>();

	let (min_x, max_x) = input
		.iter()
		.flat_map(|sensor| {
			[
				sensor.position.0 - sensor.range() as i32,
				sensor.position.0 + sensor.range() as i32,
			]
		})
		.min_max()
		.unwrap();

	const Y: i32 = 2000000;

	let part1 = (min_x..=max_x)
		.map(|x| IVec2D(x, Y))
		.filter(|&pos| {
			input
				.iter()
				.any(|sensor| sensor.covers(pos))
		})
		.filter(|&pos| {
			/* Check if the possible position doesn't already have a beacon,
			 * which means that it obviously can contain a beacon. */
			!input
				.iter()
				.map(|sensor| sensor.closest_beacon)
				.any(|x| x == pos)
		})
		.count();

	println!("Part 1: {part1}");

	const RANGE: RangeInclusive<i32> = RangeInclusive::new(0, 4000000);

	let part2 = input
		.iter()
		.flat_map(|sensor| sensor.perimeter())
		.filter(|pos| {
			if !RANGE.contains(&pos.0) || !RANGE.contains(&pos.1) {
				return false;
			}

			input
				.iter()
				.all(|sensor| !sensor.covers(*pos))
		})
		.map(tuning_frequency)
		.single()
		.unwrap();
	println!("Part 2: {part2}");
}

fn tuning_frequency(pos: IVec2D) -> i64 {
	pos.0 as i64 * 4000000 + pos.1 as i64
}
