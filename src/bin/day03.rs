use std::{
	io::{stdin, Read},
	sync::LazyLock,
};

use itertools::Itertools as _;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
	Mul(u32),
	Enable(bool),
}

fn main() {
	let input = {
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		buf
	};

	let (part1, part2, _) = parse_instructions(&input).fold(
		(0u64, 0u64, true),
		|(part1, part2, enabled), instruction| match instruction {
			Instruction::Enable(enabled) => (part1, part2, enabled),
			Instruction::Mul(n) => {
				let n = n as u64;
				let part1 = part1 + n;
				let part2 = part2 + if enabled { n } else { 0 };
				(part1, part2, enabled)
			}
		},
	);

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
	static RE: LazyLock<Regex> = LazyLock::new(|| {
		const PATTERN: &str = concat!(
			r#"(?:(mul)\((\d+),(\d+)\))"#, // mul
			"|",
			r#"(?:(do(?:n't)?)\(\))"# // do / don't
		);
		Regex::new(PATTERN).unwrap()
	});

	RE.captures_iter(input).map(|capture| {
		let matches = capture.iter().skip(1).flatten();
		let mut matches = matches.map(|m| m.as_str());
		match matches.next().unwrap() {
			"do" => Instruction::Enable(true),
			"don't" => Instruction::Enable(false),
			"mul" => {
				let (left, right) = matches
					.map(|x| x.parse::<u32>().unwrap())
					.next_tuple()
					.unwrap();
				Instruction::Mul(left * right)
			}
			_ => {
				unreachable!();
			}
		}
	})
}
