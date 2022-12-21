use std::{
	collections::HashMap,
	io::{prelude::*, stdin},
	str::FromStr,
};

use anyhow::Context;
use aoc2022::extensions::AvgExt;

fn main() -> anyhow::Result<()> {
	let input = {
		let mut buffer = String::new();
		stdin().read_to_string(&mut buffer)?;
		buffer
	};

	let tree = input.parse::<Expression>()?;

	let part1 = tree.evaluate(None);
	println!("Part 1: {part1}");

	/* By turning the root of the tree into a "Sub" node, and taking the .abs()
	 * after evaluating, we can find the distance between the two branches of
	 * the root node.
	 * The smaller the distance, the closer we are to the answer. If the
	 * distance is 0, the two branches evaluate to the same value. */
	let tree = turn_root_into_sub(tree);
	let distance_between_root_branches = |n| tree.evaluate(Some(n as _)).abs();

	/* Find the correct argument value through binary search.
	 * If the distance from the middle of the bounds to the correct argument
	 * value is 0, the correct value has been found.
	 * Otherwise, recurse into the split of which the midpoint is the closest to
	 * the correct argument value. */
	let mut bounds = (0u64, u64::MAX);

	fn find_middle(bounds: (u64, u64)) -> u64 {
		bounds.0.avg(bounds.1)
	}

	let part2 = loop {
		let middle = find_middle(bounds);

		if distance_between_root_branches(middle) == 0.0 {
			break middle;
		}

		let splits = [(bounds.0, middle), (middle, bounds.1)];
		let distances = splits
			.map(find_middle)
			.map(distance_between_root_branches);
		bounds = if distances[0] <= distances[1] {
			splits[0]
		} else {
			splits[1]
		};
	};
	println!("Part 2: {part2}");

	Ok(())
}

fn turn_root_into_sub(expression: Expression) -> Expression {
	if let Expression::Operation { left, right, .. } = expression {
		Expression::Operation {
			operator: Operator::Sub,
			left,
			right,
		}
	} else {
		expression
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
	Add,
	Sub,
	Mul,
	Div,
}

impl Operator {
	fn evaluate(self, left: f64, right: f64) -> f64 {
		match self {
			Operator::Add => left + right,
			Operator::Sub => left - right,
			Operator::Mul => left * right,
			Operator::Div => left / right,
		}
	}
}

impl FromStr for Operator {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Operator::*;
		match s {
			"+" => Ok(Add),
			"-" => Ok(Sub),
			"*" => Ok(Mul),
			"/" => Ok(Div),
			other => Err(anyhow::format_err!("Unknown operator {other}")),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
enum Expression {
	Value(f64),
	Variable {
		default: f64,
	},
	Operation {
		left: Box<Expression>,
		operator: Operator,
		right: Box<Expression>,
	},
}

impl Expression {
	fn evaluate(&self, argument: Option<f64>) -> f64 {
		match self {
			Expression::Value(v) => *v,
			Expression::Variable { default } => argument.unwrap_or(*default),
			Expression::Operation {
				left,
				operator: operation,
				right,
			} => {
				let left = left.evaluate(argument);
				let right = right.evaluate(argument);
				operation.evaluate(left, right)
			}
		}
	}
}

impl FromStr for Expression {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		fn resolve(key: &str, indexed: &HashMap<&str, &str>) -> anyhow::Result<Expression> {
			let value = indexed[key];

			if let Ok(value) = value.parse() {
				return if key == "humn" {
					Ok(Expression::Variable { default: value })
				} else {
					Ok(Expression::Value(value))
				};
			}

			let mut parts = value.split(' ');

			let left = parts
				.next()
				.context("Missing lhs of operation")?;
			let operation: Operator = parts
				.next()
				.context("Missing operator of operation")?
				.parse()?;
			let right = parts
				.next()
				.context("Missing rhs of operation")?;

			let left = resolve(left, indexed)?;
			let right = resolve(right, indexed)?;

			if let (Expression::Value(left), Expression::Value(right)) = (&left, &right) {
				/* If this is a constant expression, eagerly evaluate it to create a
					* more condensed tree. */
				Ok(Expression::Value(operation.evaluate(*left, *right)))
			} else {
				Ok(Expression::Operation {
					left: Box::new(left),
					operator: operation,
					right: Box::new(right),
				})
			}
		}

		let indexed = input
			.split('\n')
			.filter(|line| !line.is_empty())
			.map(|line| {
				line.split_once(": ")
					.context("Missing ': ' separator between key and expression")
			})
			.collect::<anyhow::Result<HashMap<_, _>>>()?;

		resolve("root", &indexed)
	}
}
