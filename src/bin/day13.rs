use std::{
	io::{stdin, Read},
	str::FromStr,
};

use itertools::Itertools;

fn main() {
	let input = {
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		buf
	};
	let patterns = input
		.split("\n\n")
		.map(|block| Pattern::from_str(block).unwrap())
		.collect::<Vec<_>>();

	let solve = |expected_errors: usize| {
		patterns
			.iter()
			.map(|pattern| {
				find_reflection_lines(pattern, expected_errors)
					.exactly_one()
					.ok()
					.expect("Expected a single reflection line")
			})
			.sum::<usize>()
	};

	println!("Part 1: {}", solve(0));
	println!("Part 2: {}", solve(1));
}

fn find_reflection_lines(
	pattern: &Pattern,
	expected_errors: usize,
) -> impl Iterator<Item = usize> + '_ {
	let vertical_lines = find_reflection_points(&pattern.columns, expected_errors);
	let horizontal_lines = find_reflection_points(&pattern.rows, expected_errors).map(|x| x * 100);
	horizontal_lines.chain(vertical_lines)
}

fn find_reflection_points(
	lines: &[u32],
	expected_errors: usize,
) -> impl Iterator<Item = usize> + '_ {
	let potential_reflection_lines = 1..=(lines.len() - 1);
	potential_reflection_lines.filter(move |&i| {
		let above = lines[..i].iter().rev();
		let below = lines[i..].iter();

		let errors = above
			.zip(below)
			.map(|(x, y)| (*x ^ *y).count_ones() as usize)
			.sum::<usize>();
		errors == expected_errors
	})
}

#[derive(Debug)]
struct Pattern {
	rows: Vec<u32>,
	columns: Vec<u32>,
}

impl FromStr for Pattern {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		fn bit_value(c: u8) -> anyhow::Result<bool> {
			Ok(match c {
				b'#' => true,
				b'.' => false,
				_ => anyhow::bail!("Unexpected symbol '{}'", c),
			})
		}

		let mut rows = vec![0; s.lines().count()];
		let mut columns = vec![0; s.lines().next().map(|l| l.len()).unwrap_or(0)];

		for (y, line) in s.lines().enumerate() {
			for (x, c) in line.trim().bytes().enumerate() {
				let bit = bit_value(c)? as u32;
				rows[y] |= bit << x;
				columns[x] |= bit << y;
			}
		}

		Ok(Self { rows, columns })
	}
}
