use std::{io::stdin, str::FromStr};

use anyhow::Context;
use aoc2018::vec::IVec2D;
use regex::Regex;

fn main() -> anyhow::Result<()> {
	let points: Vec<Point> = stdin()
		.lines()
		.flatten()
		.map(|line| line.parse())
		.collect::<Result<_, _>>()?;

	let time = find_time_of_minimum_y_distance(&points);
	println!("Time: {time}");
	println!(
		"{}",
		display(
			&points
				.iter()
				.map(|p| p.position_at(time as _))
				.collect::<Vec<_>>()
		)
	);

	Ok(())
}

fn find_time_of_minimum_y_distance(points: &[Point]) -> usize {
	let mut previous_distance = i32::MAX;
	let mut time = 0;

	loop {
		let (min, max) = IVec2D::min_max(
			points
				.iter()
				.map(|p| p.position_at(time)),
		)
		.unwrap();

		let distance = max.1 - min.1;
		if distance >= previous_distance {
			return time as usize - 1;
		} else {
			previous_distance = distance;
		}

		time += 1;
	}
}

fn display(positions: &[IVec2D]) -> String {
	let (min, max) = IVec2D::min_max(positions.iter().copied()).unwrap();
	let mut output = vec![vec![' '; (max.0 - min.0 + 1) as usize]; (max.1 - min.1 + 1) as usize];
	for position in positions {
		output[(position.1 - min.1) as usize][(position.0 - min.0) as usize] = '#';
	}
	output
		.into_iter()
		.map(|line| line.into_iter().collect::<String>())
		.collect::<Vec<_>>()
		.join("\n")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Point {
	position: IVec2D,
	velocity: IVec2D,
}

impl Point {
	fn position_at(&self, time: i32) -> IVec2D {
		self.position + self.velocity * time
	}
}

impl FromStr for Point {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		lazy_static::lazy_static! {
			static ref PATTERN: Regex = Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$").unwrap();
		}

		let captures = PATTERN
			.captures(s)
			.context("Invalid pattern")?;
		let mut captures = captures
			.iter()
			.skip(1)
			.map(Option::unwrap)
			.map(|c| c.as_str())
			.map(|s| s.parse().unwrap());

		let mut next = || captures.next().unwrap();

		Ok(Point {
			position: IVec2D(next(), next()),
			velocity: IVec2D(next(), next()),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse() -> anyhow::Result<()> {
		const INPUT: &str = "position=< -9932,  40548> velocity=< 1, -4>";
		const EXPECTED: Point = Point {
			position: IVec2D(-9932, 40548),
			velocity: IVec2D(1, -4),
		};
		let actual: Point = INPUT.parse()?;
		assert_eq!(EXPECTED, actual);
		Ok(())
	}
}
