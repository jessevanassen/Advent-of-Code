use std::{io::stdin, str::FromStr};

use anyhow::Context;
use aoc2018::vec::IVec2D;
use regex::Regex;

#[derive(Debug)]
struct Rectangle {
	id: u16,
	position: IVec2D,
	width: u32,
	height: u32,
}

impl Rectangle {
	fn points(&self) -> impl Iterator<Item = IVec2D> + '_ {
		let x_range = self.position.0..(self.position.0 + self.width as i32);
		let y_range = self.position.1..(self.position.1 + self.height as i32);

		y_range.flat_map(move |y| {
			x_range
				.clone()
				.map(move |x| IVec2D(x, y))
		})
	}
}

impl FromStr for Rectangle {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		lazy_static::lazy_static! {
			static ref PATTERN: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
		}

		let matches = PATTERN
			.captures(s)
			.context("Expected input to match pattern")?;
		let mut matches = matches
			.iter()
			.skip(1)
			.map(|capture| capture.unwrap().as_str());

		macro_rules! next {
			() => {
				matches.next().unwrap().parse()?
			};
		}

		Ok(Rectangle {
			id: next!(),
			position: IVec2D(next!(), next!()),
			width: next!(),
			height: next!(),
		})
	}
}

fn main() -> anyhow::Result<()> {
	let rectangles = stdin()
		.lines()
		.flatten()
		.map(|line| line.parse())
		.collect::<anyhow::Result<Vec<Rectangle>>>()?;

	let occupation = {
		let max = rectangles
			.iter()
			.map(|r| {
				(
					r.position.0 + r.width as i32,
					r.position.1 + r.height as i32,
				)
			})
			.reduce(|acc, it| (acc.0.max(it.0), acc.1.max(it.1)))
			.unwrap();

		let mut occupation = vec![vec![0u16; max.0 as usize]; max.1 as usize];

		for point in rectangles
			.iter()
			.flat_map(|r| r.points())
		{
			occupation[point.1 as usize][point.0 as usize] += 1;
		}

		occupation
	};

	let part1: usize = occupation
		.iter()
		.map(|xs| xs.iter().filter(|&&c| c > 1).count())
		.sum();
	println!("Part 1: {part1}");

	let part2 = 'block: {
		for rectangle in &rectangles {
			if rectangle.points().all(|p| occupation[p.1 as usize][p.0 as usize] == 1) {
				break 'block rectangle;
			}
		}

		unreachable!();
	};
	println!("Part 2: {}", part2.id);

	Ok(())
}
