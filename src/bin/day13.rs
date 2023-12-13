use std::{
	io::{stdin, Read},
	str::FromStr,
};

use itertools::Itertools;

fn main() {
	let input = {
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		let buf = buf.trim_end().to_string();
		buf.split("\n\n")
			.map(|block| block.to_string())
			.collect::<Vec<_>>()
	};

	let reflection_lines = input
		.iter()
		.map(|block| Pattern::from_str(block).unwrap())
		.map(|pattern| {
			find_reflection_lines(&pattern)
				.exactly_one()
				.ok()
				.expect("Expected a single reflection line")
		})
		.collect::<Vec<_>>();

	let part1 = reflection_lines.iter().sum::<usize>();
	println!("Part 1: {part1}");

	let part2 = input
		.iter()
		.enumerate()
		.map(|(i, block)| {
			possible_fixes(block)
				.map(|block| Pattern::from_str(&block).unwrap())
				.flat_map(|pattern| find_reflection_lines(&pattern).collect::<Vec<_>>())
				/* The original reflection line might still be there, e.g. if the smudge was fixed
				 * around the top left, but the original reflection line was towards the right. */
				.filter(|line| line != &reflection_lines[i])
				/* There should be only a single additional reflection line left. However, this
				 * line can be found in two ways, by either changing the smudge on one side of the
				 * reflection line, or on the other side, leading in two identical reflection
				 * lines.
				 * After removing the duplicate, only a single unique reflection line should
				 * remain.*/
				.dedup()
				.exactly_one()
				.ok()
				.expect("Expected a single reflection line")
		})
		.sum::<usize>();
	println!("Part 2: {part2}");
}

fn possible_fixes(input: &str) -> impl Iterator<Item = String> + '_ {
	fn replacement(input: u8) -> Option<u8> {
		match input {
			b'.' => Some(b'#'),
			b'#' => Some(b'.'),
			_ => None,
		}
	}

	let input = input.as_bytes();

	input.iter().enumerate().filter_map(|(i, c)| {
		let replacement = replacement(*c)?;

		let mut bytes = input.to_vec();
		bytes[i] = replacement;

		Some(unsafe { String::from_utf8_unchecked(bytes) })
	})
}

fn find_reflection_lines(pattern: &Pattern) -> impl Iterator<Item = usize> + '_ {
	let vertical_lines = find_reflection_points(&pattern.columns);
	let horizontal_lines = find_reflection_points(&pattern.rows).map(|x| x * 100);
	horizontal_lines.chain(vertical_lines)
}

fn find_reflection_points(lines: &[u32]) -> impl Iterator<Item = usize> + '_ {
	(1..lines.len()).filter(|&i| {
		let above = lines[..i].iter().rev();
		let below = lines[i..].iter();

		above.zip(below).all(|(x, y)| x == y)
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
			for (x, c) in line.bytes().enumerate() {
				let bit = bit_value(c)? as u32;
				rows[y] |= bit << x;
				columns[x] |= bit << y;
			}
		}

		Ok(Self { rows, columns })
	}
}
