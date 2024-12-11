use std::io::stdin;
use std::ops::{Add as _, Mul as _};

use aoc2024::digits::count_digits;

fn main() {
	let input = parse_input().collect::<Vec<_>>();

	let solved_sum = |operators: &[Operator]| {
		input
			.iter()
			.filter_map(|eq| eq.is_solvable(operators).then_some(eq.target))
			.sum::<u64>()
	};

	println!("Part 1: {}", solved_sum(&[u64::add, u64::mul]));
	println!("Part 2: {}", solved_sum(&[u64::add, u64::mul, concat]));
}

fn concat(x: u64, y: u64) -> u64 {
	x * 10u64.pow(count_digits(y)) + y
}

#[derive(derive_more::From)]
struct Equation {
	target: u64,
	operants: Vec<u64>,
}

type Operator = fn(u64, u64) -> u64;

impl Equation {
	pub fn is_solvable(&self, operators: &[Operator]) -> bool {
		fn is_solvable(operators: &[Operator], target: u64, acc: u64, rest: &[u64]) -> bool {
			match rest {
				[] => acc == target,
				&[head, ref rest @ ..] => operators.iter().any(|op| {
					let acc = op(acc, head);

					if acc > target {
						/* Accumulated value can never become smaller, so if it
						 * overshoots the target, we can terminate early. */
						return false;
					}

					is_solvable(operators, target, acc, rest)
				}),
			}
		}

		is_solvable(
			operators,
			self.target,
			self.operants[0],
			&self.operants[1..],
		)
	}
}

fn parse_input() -> impl Iterator<Item = Equation> {
	fn parse_equation(input: &str) -> Equation {
		use nom::{
			bytes::complete::tag,
			character::complete::{self, space1},
			combinator::map,
			multi::separated_list1,
			sequence::separated_pair,
			IResult,
		};

		let result: IResult<&str, _> = map(
			separated_pair(
				complete::u64,
				tag(": "),
				separated_list1(space1, complete::u64),
			),
			Equation::from,
		)(input);
		result.unwrap().1
	}

	stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| parse_equation(&line))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[rstest::rstest]
	#[case((0, 1), 1)]
	#[case((1, 0), 10)]
	#[case((1, 2), 12)]
	#[case((123, 4), 1234)]
	#[case((1, 234), 1234)]
	#[case((123, 456), 123456)]
	fn test_concat(#[case] (x, y): (u64, u64), #[case] expected: u64) {
		assert_eq!(concat(x, y), expected);
	}
}
