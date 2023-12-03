use std::{io::stdin, ops::Range, collections::HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
	static ref ONE_OR_MORE_NUMBERS: Regex = Regex::new(r"\d+").unwrap();
}

type Position = (usize, usize);

struct Number {
	number: usize,
	xs: Range<usize>,
	y: usize,
}

impl Number {
	fn positions_around(&self) -> impl Iterator<Item = Position> {
		around(self.xs.clone(), self.y)
	}

	fn positions(&self) -> impl Iterator<Item = Position> {
		let y = self.y;
		self.xs.clone().map(move |x| (x, y))
	}
}

struct Symbol {
	symbol: u8,
	x: usize,
	y: usize,
}

impl Symbol {
	fn position(&self) -> Position {
		(self.x, self.y)
	}

	fn positions_around(&self) -> impl Iterator<Item = Position> {
		let x = self.x..(self.x + 1);
		around(x, self.y)
	}

	fn is_gear(&self) -> bool {
		self.symbol == b'*'
	}

	fn is_symbol(c: u8) -> bool {
		!c.is_ascii_digit() && c != b'.'
	}
}

fn main() {
	let lines = stdin().lines().flatten();
	let input = lines.collect::<Vec<_>>();

	let numbers = input
		.iter()
		.enumerate()
		.flat_map(|(y, row)| {
			ONE_OR_MORE_NUMBERS.captures_iter(row).map(move |capture| {
				let capture = capture.get(0).unwrap();

				let number = capture.as_str().parse().unwrap();
				let xs = capture.start()..capture.end();

				Number { number, xs, y }
			})
		})
		.collect::<Box<_>>();

	let symbols = character_positions(&input)
		.filter(|(_, c)| Symbol::is_symbol(*c))
		.map(move |((x, y), symbol)| Symbol { symbol, x, y })
		.collect::<Box<_>>();

	println!(
		"Part 1: {}",
		numbers
			.iter()
			.filter({
				let symbol_positions = symbols.iter().map(|s| s.position()).collect::<HashSet<_>>();
				move |number| {
					number
						.positions_around()
						.any(|ref p| symbol_positions.contains(p))
				}
			})
			.map(|number| number.number)
			.sum::<usize>()
	);

	println!(
		"Part 2: {}",
		symbols
			.iter()
			.filter(|s| s.is_gear())
			.map(|gear| {
				let positions_around_gear = gear.positions_around().collect::<Box<_>>();
				let is_touching_gear = |number: &Number| {
					number
						.positions()
						.any(|p| positions_around_gear.contains(&p))
				};

				let numbers_surrounding_gear = numbers
					.iter()
					.filter(|n| is_touching_gear(n))
					.map(|n| n.number)
					.collect::<Box<_>>();

				match &numbers_surrounding_gear[..] {
					[n0, n1] => n0 * n1,
					_ => 0,
				}
			})
			.sum::<usize>()
	);
}

fn character_positions(input: &[impl AsRef<str>]) -> impl Iterator<Item = (Position, u8)> + '_ {
	input.iter().enumerate().flat_map(|(y, row)| {
		row.as_ref()
			.bytes()
			.enumerate()
			.map(move |(x, c)| ((x, y), c))
	})
}

fn around(x: Range<usize>, y: usize) -> impl Iterator<Item = (usize, usize)> {
	let xs = x.start.saturating_sub(1)..(x.end + 1);
	let ys = y.saturating_sub(1)..=(y + 1);
	let input_range_contains = move |(x_, y_): &(usize, usize)| *y_ == y && x.contains(x_);

	xs.cartesian_product(ys)
		.filter(move |it| !input_range_contains(it))
}
