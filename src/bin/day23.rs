use std::{collections::BTreeSet, error::Error, io::stdin};

use aoc2024::bitset::BitSet;
use inner::*;
use itertools::Itertools;

fn main() {
	let connections = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| -> Result<_, Box<dyn Error>> {
			let (left, right) = line.split_at(2);
			let left = left.parse()?;
			let right = right[1..].parse()?;
			Ok(Connection::new(left, right))
		})
		.collect::<Result<Vec<_>, _>>()
		.unwrap();

	let computers = connections
		.iter()
		.copied()
		.flat_map(|c| c.computers())
		.unique()
		.collect::<Vec<_>>();

	let mut connection_map = BitSet::with_size(Connection::DIFFERENT_VALUES);
	connection_map.extend(connections.iter().copied());

	let part1 = computers
		.iter()
		.copied()
		.tuple_combinations()
		.map(|(a, b, c)| [a, b, c])
		.filter(|combination| combination.iter().any(|computer| computer.name()[0] == 't'))
		.filter(|&combination| {
			combination
				.iter()
				.copied()
				.tuple_combinations()
				.map(|(a, b)| Connection::new(a, b))
				.all(|connection| connection_map.contains(connection))
		})
		.count();
	println!("Part 1: {part1}");

	let largest_cluster = connections
		.iter()
		.copied()
		.map(|connection| grow(connection, &computers, &connection_map))
		.unique()
		.max_by_key(|cluster| cluster.len())
		.unwrap();
	println!(
		"Part 2: {} (with length {})",
		largest_cluster.iter().format(","),
		largest_cluster.len()
	);
}

fn grow(
	connection: Connection,
	computers: &[Computer],
	connection_map: &BitSet,
) -> BTreeSet<Computer> {
	let mut result = BTreeSet::new();
	result.extend(connection.computers());

	loop {
		let mut grown = false;

		for computer in computers.iter().copied() {
			if !result.contains(&computer)
				&& result
					.iter()
					.all(|&other| connection_map.contains(Connection::new(computer, other)))
			{
				grown |= result.insert(computer);
			}
		}

		if !grown {
			break;
		}
	}

	result
}

mod inner {
	use std::{fmt::Display, str::FromStr};

	use aoc2024::bitset;

	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Into)]
	pub struct Computer(u16);

	impl Computer {
		pub const DIFFERENT_VALUES: usize = 26 * 26;

		pub fn name(self) -> [char; 2] {
			[
				((self.0 / 26) as u8 + b'a') as char,
				((self.0 % 26) as u8 + b'a') as char,
			]
		}
	}

	impl FromStr for Computer {
		type Err = Box<dyn std::error::Error>;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			let bytes = s.as_bytes();
			if bytes.len() != 2 || !bytes[0].is_ascii_lowercase() || !bytes[1].is_ascii_lowercase()
			{
				return Err(format!(r#""{}" is not a valid computer name"#, s).into());
			}

			let v = (bytes[0] - b'a') as u16 * 26 + (bytes[1] - b'a') as u16;
			Ok(Self(v))
		}
	}

	impl Display for Computer {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let name = self.name();
			write!(f, "{}{}", name[0], name[1])
		}
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct Connection(Computer, Computer);

	impl Connection {
		pub const DIFFERENT_VALUES: usize = Computer::DIFFERENT_VALUES * Computer::DIFFERENT_VALUES;

		pub fn new(a: Computer, b: Computer) -> Self {
			Self(a.min(b), a.max(b))
		}

		pub fn computers(self) -> [Computer; 2] {
			[self.0, self.1]
		}
	}

	impl bitset::IntoIndex for Connection {
		fn into_index(self) -> usize {
			self.0 .0 as usize * Computer::DIFFERENT_VALUES + self.1 .0 as usize
		}
	}
}
