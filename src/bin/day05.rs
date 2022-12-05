use lazy_static::lazy_static;
use regex::Regex;
use std::{io::stdin, ops::Deref, str::FromStr};

type Stack = Vec<u8>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move {
	from: usize,
	to: usize,
	count: usize,
}

impl FromStr for Move {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		lazy_static! {
			static ref RE: Regex = Regex::new("^move (\\d+) from (\\d+) to (\\d+)$").unwrap();
		}

		let captures = RE.captures(s).ok_or(())?;
		let mut captures = captures
			.iter()
			.skip(1) // Skip the full match, only take the capture groups
			.flatten()
			.map(|m| m.as_str().parse().unwrap());

		Ok(Move {
			count: captures.next().unwrap(),
			from: captures.next().unwrap() - 1,
			to: captures.next().unwrap() - 1,
		})
	}
}

fn main() {
	let (stacks, moves) = parse_input(&mut stdin().lines().flatten());

	let mut part1_stacks = stacks.clone();
	for &Move { from, to, count } in &moves {
		for _ in 0..count {
			let item = part1_stacks[from].pop().expect("Expected the stack to be non-empty");
			part1_stacks[to].push(item);
		}
	}
	println!("Part 1: {}", format_stacks(&part1_stacks));

	let mut part2_stacks = stacks.clone();
	for &Move { from, to, count } in &moves {
		let from = &mut part2_stacks[from];
		let mut popped = from.split_off(from.len() - count);
		part2_stacks[to].append(&mut popped);
	}
	println!("Part 2: {}", format_stacks(&part2_stacks));
}

fn format_stacks<'a>(stacks: impl IntoIterator<Item = &'a Stack>) -> String {
	stacks
		.into_iter()
		.map(|s| s.last().copied().unwrap_or(b' '))
		.map(|c| c as char)
		.collect::<String>()
}

fn parse_input(
	input: &mut impl Iterator<Item = impl Deref<Target = str>>,
) -> (Vec<Stack>, Vec<Move>) {
	let mut crate_lines = input
		.take_while(|line| !line.is_empty())
		.map(|line| {
			line.bytes()
				.skip(1)
				.step_by(4)
				.map(|c| c.is_ascii_alphanumeric().then(|| c))
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	crate_lines.pop(); // Number line can be ignored

	let mut crates: Vec<Vec<u8>> = vec![Vec::new(); crate_lines.last().map_or(0, Vec::len)];

	for cs in crate_lines.iter().rev() {
		for (i, c) in cs.iter().enumerate() {
			if let Some(v) = c {
				crates[i].push(*v);
			}
		}
	}

	let moves = input
		.map(|line| line.parse().expect("Malformed line"))
		.collect();

	(crates, moves)
}
