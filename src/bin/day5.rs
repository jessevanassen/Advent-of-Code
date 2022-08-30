use std::{collections::HashMap, fmt::Display, str::FromStr};

use advent_of_code_2021::read_lines_from_stdin;
#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
	x: i32,
	y: i32,
}
impl From<(i32, i32)> for Point {
	fn from((x, y): (i32, i32)) -> Self {
		Self { x, y }
	}
}
impl FromStr for Point {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (left, right) = s.split_once(',').ok_or(())?;
		let x = left.parse().map_err(|_| ())?;
		let y = right.parse().map_err(|_| ())?;
		Ok(Self { x, y })
	}
}
impl Display for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{},{}", self.x, self.y)?;
		Ok(())
	}
}

#[derive(PartialEq, Debug)]
struct Line(Point, Point);
impl Line {
	fn points(&self) -> Vec<Point> {
		if self.is_horizontal() {
			range(self.0.x, self.1.x)
				.map(|x| Point { x, y: self.0.y })
				.collect()
		} else if self.is_vertical() {
			range(self.0.y, self.1.y)
				.map(|y| Point { y, x: self.0.x })
				.collect()
		} else {
			range(self.0.x, self.1.x)
				.zip(range(self.0.y, self.1.y))
				.map(Point::from)
				.collect()
		}
	}

	fn is_horizontal(&self) -> bool {
		self.0.y == self.1.y
	}
	fn is_vertical(&self) -> bool {
		self.0.x == self.1.x
	}
	fn is_straight(&self) -> bool {
		self.is_horizontal() || self.is_vertical()
	}
}
impl FromStr for Line {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (left, right) = s.split_once(" -> ").ok_or(())?;
		Ok(Line(left.parse()?, right.parse()?))
	}
}
impl Display for Line {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} -> {}", self.0, self.1)?;
		Ok(())
	}
}
fn main() {
	let lines = read_lines_from_stdin::<Line>();

	let points = lines.iter()
		.filter(|l| l.is_straight())
		.flat_map(Line::points)
		.into_iter();


	println!("Part 1: {}", count_overlapping(points));

	let points = lines.iter()
		.flat_map(Line::points)
		.into_iter();
	println!("Part 2: {}", count_overlapping(points));
}

fn count_overlapping(points: impl Iterator<Item = Point>) -> usize {
	let mut counts = HashMap::<Point, u32>::new();

	for point in points {
		counts
			.entry(point)
			.and_modify(|count| *count += 1)
			.or_insert(1);
	}

	counts.values().filter(|count| **count >= 2).count()
}

fn range(x: i32, y: i32) -> Box<dyn Iterator<Item = i32>> {
	if x < y {
		Box::new(x..=y)
	} else {
		Box::new((y..=x).rev())
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_points_horizontal() {
		let line = Line(Point { x: 1, y: 1 }, Point { x: 1, y: 3 });
		let points = line.points();
		let expected = (1..=3).map(|y| Point { x: 1, y }).collect::<Vec<_>>();
		assert_eq!(expected, points);
	}
}
