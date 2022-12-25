use std::{
	fmt::Display,
	io::stdin,
	ops::{Add, AddAssign},
	str::FromStr,
};

use thiserror::Error;

fn main() -> anyhow::Result<()> {
	let mut total = Snafu(0);
	for line in stdin().lines().flatten() {
		let snafu: Snafu = line.parse()?;
		total += snafu;
	}
	println!("Part 1: {}", total);

	Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Snafu(i64);

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[error("cannot parse Snafu from string")]
pub struct ParseSnafuError;

impl FromStr for Snafu {
	type Err = ParseSnafuError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		fn parse(c: u8) -> Result<i64, ParseSnafuError> {
			match c {
				b'=' => Ok(-2),
				b'-' => Ok(-1),
				b'0'..=b'2' => Ok((c - b'0') as _),
				_ => Err(ParseSnafuError),
			}
		}

		let mut result = 0;

		for digit in s.bytes() {
			let digit = parse(digit)?;
			result = result * 5 + digit;
		}

		Ok(Snafu(result))
	}
}

impl Display for Snafu {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fn format(number: i64) -> String {
			if number == 0 {
				return String::new();
			}

			match number % 5 {
				0 => format(number / 5) + "0",
				1 => format(number / 5) + "1",
				2 => format(number / 5) + "2",
				3 => format(number / 5 + 1) + "=",
				4 => format(number / 5 + 1) + "-",
				_ => unreachable!(),
			}
		}

		if self.0 == 0 {
			return write!(f, "0");
		}

		write!(f, "{}", format(self.0))
	}
}

impl Add for Snafu {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Snafu(self.0 + rhs.0)
	}
}

impl AddAssign for Snafu {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: [(Snafu, &str); 27] = [
		(Snafu(0), "0"),
		(Snafu(1), "1"),
		(Snafu(2), "2"),
		(Snafu(3), "1="),
		(Snafu(4), "1-"),
		(Snafu(5), "10"),
		(Snafu(6), "11"),
		(Snafu(7), "12"),
		(Snafu(8), "2="),
		(Snafu(9), "2-"),
		(Snafu(10), "20"),
		(Snafu(11), "21"),
		(Snafu(15), "1=0"),
		(Snafu(20), "1-0"),
		(Snafu(31), "111"),
		(Snafu(32), "112"),
		(Snafu(37), "122"),
		(Snafu(107), "1-12"),
		(Snafu(198), "2=0="),
		(Snafu(201), "2=01"),
		(Snafu(353), "1=-1="),
		(Snafu(906), "12111"),
		(Snafu(1257), "20012"),
		(Snafu(1747), "1=-0-2"),
		(Snafu(2022), "1=11-2"),
		(Snafu(12345), "1-0---0"),
		(Snafu(314159265), "1121-1110-1=0"),
	];

	#[test]
	fn test_parse_snafu() {
		for (expected, input) in INPUT {
			let parsed: Result<Snafu, _> = input.parse();
			assert_eq!(Ok(expected), parsed, "Parse {input} to {expected:?}");
		}
	}

	#[test]
	fn test_format_snafu() {
		for (input, expected) in INPUT {
			let actual = input.to_string();
			assert_eq!(expected, actual, "Format {input:?} to {expected}");
		}
	}
}
