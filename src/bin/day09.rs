use std::{io::stdin, iter};

use aoc2025::{minmax, vector::Vector2D};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rect {
	left: i64,
	top: i64,
	right: i64,
	bottom: i64,
}

impl Rect {
	pub const fn width(&self) -> u64 {
		self.left.abs_diff(self.right) + 1
	}

	pub const fn height(&self) -> u64 {
		self.top.abs_diff(self.bottom) + 1
	}

	pub const fn area(&self) -> u64 {
		self.width() * self.height()
	}
}

impl From<(Vector2D, Vector2D)> for Rect {
	fn from(value: (Vector2D, Vector2D)) -> Self {
		let [left, right] = minmax(value.0.x, value.1.x);
		let [top, bottom] = minmax(value.0.y, value.1.y);
		Self {
			top,
			right,
			bottom,
			left,
		}
	}
}

fn main() {
	let positions = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| {
			let (x, y) = line.split_once(',').expect("Expect ',' delimiter");
			let x = x.parse().unwrap();
			let y = y.parse().unwrap();
			Vector2D { x, y }
		})
		.collect::<Vec<_>>();

	let lines = positions
		.iter()
		.chain(iter::once(&positions[0]))
		.copied()
		.tuple_windows()
		.map(|rect: (Vector2D, Vector2D)| Rect::from(rect))
		.collect::<Vec<Rect>>();

	debug_assert!(
		lines
			.iter()
			.all(|line| line.width() == 1 || line.height() == 1),
		"Expected lines to actually be lines",
	);

	let rectangles = positions
		.iter()
		.copied()
		.tuple_combinations()
		.map(|rect: (Vector2D, Vector2D)| Rect::from(rect))
		.sorted_by_key(|rect| std::cmp::Reverse(rect.area()))
		.collect::<Vec<_>>();

	println!("Part 1: {}", rectangles.first().unwrap().area());

	println!(
		"Part 2: {}",
		rectangles
			.iter()
			.find(|rect| !lines.iter().any(move |line| intersects_with(**rect, *line)))
			.unwrap()
			.area()
	)
}

fn intersects_with(rect: Rect, line: Rect) -> bool {
	!(line.right <= rect.left
		|| line.left >= rect.right
		|| line.bottom <= rect.top
		|| line.top >= rect.bottom)
}
