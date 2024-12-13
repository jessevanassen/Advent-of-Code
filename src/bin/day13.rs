use std::{borrow::Borrow, io::stdin, iter, sync::LazyLock};

use aoc2024::Vector2D;
use regex::Regex;

fn main() {
	let input = parse_input().collect::<Vec<_>>();

	let part1 = input.iter().flat_map(cheapest_prize).sum::<u64>();
	println!("Part 1: {part1}");

	let part2 = input
		.iter()
		.map(|machine| machine.add_correction())
		.flat_map(cheapest_prize)
		.sum::<u64>();
	println!("Part 2: {part2}");
}

fn cheapest_prize(machine: impl Borrow<Machine>) -> Option<u64> {
	let machine = machine.borrow();

	let d = (machine.a.x * machine.b.y - machine.a.y * machine.b.x) as f64;

	let a = (machine.prize.x * machine.b.y - machine.prize.y * machine.b.x) as f64 / d;
	let b = (machine.prize.y * machine.a.x - machine.prize.x * machine.a.y) as f64 / d;

	if a == a.floor() && b == b.floor() {
		Some((3f64 * a + b) as u64)
	} else {
		None
	}
}

#[derive(Debug)]
struct Machine {
	a: Vector2D,
	b: Vector2D,
	prize: Vector2D,
}

impl Machine {
	fn add_correction(&self) -> Self {
		const CORRECTION_VALUE: i64 = 10_000_000_000_000;
		const CORRECTION: Vector2D = Vector2D {
			x: CORRECTION_VALUE,
			y: CORRECTION_VALUE,
		};

		Self {
			prize: self.prize + CORRECTION,
			..*self
		}
	}
}

fn parse_input() -> impl Iterator<Item = Machine> {
	static BUTTON_PATTERN: LazyLock<Regex> =
		LazyLock::new(|| Regex::new(r#"Button [AB]: X\+(\d+), Y\+(\d+)"#).unwrap());

	static PRIZE_PATTERN: LazyLock<Regex> =
		LazyLock::new(|| Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap());

	fn parse_vector(input: &str, pattern: &Regex) -> Vector2D {
		let numbers = pattern.captures(input).expect("Expect correct pattern");
		let mut numbers = numbers
			.iter()
			.skip(1)
			.map(Option::unwrap)
			.map(|m| m.as_str().parse().unwrap());
		Vector2D {
			x: numbers.next().unwrap(),
			y: numbers.next().unwrap(),
		}
	}

	let mut lines = stdin().lines().map(Result::unwrap);
	iter::from_fn(move || {
		let a = parse_vector(&lines.next()?, &BUTTON_PATTERN);
		let b = parse_vector(&lines.next()?, &BUTTON_PATTERN);
		let prize = parse_vector(&lines.next()?, &PRIZE_PATTERN);
		let _ = lines.next(); // Optional separator

		Some(Machine { a, b, prize })
	})
}
