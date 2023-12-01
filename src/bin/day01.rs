use lazy_static::lazy_static;
use regex::Regex;
use std::{io::stdin, iter};

lazy_static! {
	static ref NUMBER_PATTERN: Regex = Regex::new(r"\d").unwrap();
	static ref NUMBER_AND_DIGIT_PATTERN: Regex =
		Regex::new(r"zero|one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
}

fn main() {
	let lines = stdin().lines().flatten().collect::<Vec<_>>();

	println!("Part 1: {}", solve(&NUMBER_PATTERN, &lines));
	println!("Part 2: {}", solve(&NUMBER_AND_DIGIT_PATTERN, &lines));
}

fn solve(pattern: &Regex, input: &[String]) -> usize {
	input
		.iter()
		.filter_map(|line| solve_line(pattern, line))
		.fold(0usize, |acc, it| acc + it as usize)
}

fn solve_line(pattern: &Regex, line: &str) -> Option<u8> {
	let mut digits = overlapping_matches(pattern, line).map(|s| parse_digit(s).unwrap());
	let first = digits.next()?;
	let last = digits.last().unwrap_or(first);
	Some(first * 10 + last)
}

/**
 * The standard `Regex::matches` doesn't yield overlapping matches, so "twone" with the
 * `"one|two"` pattern would only yield "two".
 */
fn overlapping_matches<'a>(
	pattern: &'a Regex,
	input: &'a str,
) -> impl Iterator<Item = &'a str> + 'a {
	let mut offset = 0;
	iter::from_fn(move || {
		let capture = pattern.captures_at(input, offset)?.get(0).unwrap();
		offset = capture.start() + 1;
		Some(capture.as_str())
	})
}

fn parse_digit(s: &str) -> Option<u8> {
	let c = s.as_bytes().first()?;

	if c.is_ascii_digit() {
		return Some(c - b'0');
	}

	static DIGITS: [&str; 10] = [
		"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
	];

	DIGITS.iter().position(|&d| d == s).map(|x| x as u8)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solve_line() {
		assert_eq!(Some(21), solve_line(&NUMBER_AND_DIGIT_PATTERN, "two one"));
		assert_eq!(Some(21), solve_line(&NUMBER_AND_DIGIT_PATTERN, "twoone"));
		assert_eq!(Some(21), solve_line(&NUMBER_AND_DIGIT_PATTERN, "twone"));
	}

	#[test]
	fn test_parse_digit() {
		assert_eq!(Some(0), parse_digit("0"));
		assert_eq!(Some(1), parse_digit("1"));
		assert_eq!(Some(2), parse_digit("2"));
		assert_eq!(Some(3), parse_digit("3"));
		assert_eq!(Some(4), parse_digit("4"));
		assert_eq!(Some(5), parse_digit("5"));
		assert_eq!(Some(6), parse_digit("6"));
		assert_eq!(Some(7), parse_digit("7"));
		assert_eq!(Some(8), parse_digit("8"));
		assert_eq!(Some(9), parse_digit("9"));

		assert_eq!(Some(0), parse_digit("zero"));
		assert_eq!(Some(1), parse_digit("one"));
		assert_eq!(Some(2), parse_digit("two"));
		assert_eq!(Some(3), parse_digit("three"));
		assert_eq!(Some(4), parse_digit("four"));
		assert_eq!(Some(5), parse_digit("five"));
		assert_eq!(Some(6), parse_digit("six"));
		assert_eq!(Some(7), parse_digit("seven"));
		assert_eq!(Some(8), parse_digit("eight"));
		assert_eq!(Some(9), parse_digit("nine"));
	}
}
