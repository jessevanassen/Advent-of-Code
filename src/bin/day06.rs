use std::io::stdin;

fn main() {
	let mut values = stdin().lines().map(Result::unwrap).collect::<Vec<_>>();

	let operators = values
		.pop()
		.unwrap()
		.chars()
		.filter(|ch| !ch.is_whitespace())
		.map(|ch| try_parse_op(ch).unwrap())
		.collect::<Vec<_>>();

	{
		let values = values
			.iter()
			.map(|line| {
				line.split_whitespace()
					.map(|n| n.parse().unwrap())
					.collect()
			})
			.collect::<Vec<Vec<i64>>>();
		let values = transpose(&values);

		let part1 = operators
			.iter()
			.zip(values.iter())
			.map(|(op, values)| values.iter().copied().reduce(op).unwrap())
			.sum::<i64>();
		println!("Part 1: {part1}");
	}

	{
		let values = values
			.iter()
			.map(|line| line.chars().collect())
			.collect::<Vec<Vec<_>>>();
		let values = transpose(&values)
			.into_iter()
			.map(|chs| {
				chs.into_iter()
					.filter(|ch| ch.is_numeric())
					.collect::<String>()
					.parse()
					.ok()
			})
			.collect::<Vec<Option<i64>>>();

		let mut acc = 0i64;
		let mut subacc: Option<i64> = None;

		let mut op_index = operators.len() - 1;
		for value in values.into_iter().rev() {
			match value {
				None => {
					acc += subacc.unwrap_or_default();
					subacc = None;
					op_index -= 1;
				}
				Some(v) => {
					let op = operators[op_index];
					subacc = Some(match subacc {
						Some(prev) => op(prev, v),
						None => v,
					})
				}
			}
		}
		acc += subacc.unwrap_or_default();

		println!("Part 2: {acc}");
	}
}

fn transpose<T: Copy>(input: &[Vec<T>]) -> Vec<Vec<T>> {
	(0..input[0].len())
		.map(|column| (0..input.len()).map(|row| input[row][column]).collect())
		.collect()
}

fn try_parse_op(op: char) -> Result<fn(i64, i64) -> i64, anyhow::Error> {
	use std::ops::{Add, Div, Mul, Sub};
	match op {
		'+' => Ok(i64::add),
		'-' => Ok(i64::sub),
		'*' => Ok(i64::mul),
		'/' => Ok(i64::div),
		_ => anyhow::bail!("Unexpected operator '{op}'"),
	}
}
