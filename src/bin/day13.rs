use anyhow::anyhow;
use aoc2022::propagate;
use std::{cmp::Ordering, io::stdin, iter, str::FromStr};

type PacketValue = u8;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
	Integer(PacketValue),
	List(Vec<Packet>),
}

impl FromStr for Packet {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use nom::{
			branch::alt,
			character::complete::{char, u8},
			combinator::map,
			error::VerboseError,
			multi::separated_list0,
			sequence::delimited,
			Finish, IResult,
		};

		fn packet(s: &str) -> IResult<&str, Packet, VerboseError<&str>> {
			alt((
				map(u8, Packet::Integer),
				map(
					delimited(char('['), separated_list0(char(','), packet), char(']')),
					Packet::List,
				),
			))(s)
		}

		packet(s)
			.finish()
			.map(|x| x.1)
			.map_err(|err| anyhow!(err.to_string()))
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> Ordering {
		if let (Packet::Integer(l), Packet::Integer(r)) = (self, other) {
			return l.cmp(r);
		}

		fn iter(packet: &Packet) -> Box<dyn Iterator<Item = &Packet> + '_> {
			match packet {
				Packet::Integer(_) => Box::new(iter::once(packet)),
				Packet::List(packets) => Box::new(packets.iter()),
			}
		}

		iter(self).cmp(iter(other))
	}
}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn main() -> anyhow::Result<()> {
	let packet_pairs =
		parse_packet_pairs(stdin().lines().flatten()).collect::<anyhow::Result<Vec<_>>>()?;

	let part1 = packet_pairs
		.iter()
		.enumerate()
		.filter(|(_, p)| p[0] < p[1])
		.map(|(i, _)| i + 1)
		.sum::<usize>();
	println!("Part 1: {part1}");

	let dividers = [2, 6].map(|v| Packet::List(vec![Packet::List(vec![Packet::Integer(v)])]));

	let part2 = packet_pairs
		.iter()
		.flatten()
		// We don't have to sort the entries to find the final position of a
		// divider, we only have to count how many items are smaller than a
		// divider to find its final position.
		// With a fold, we can do that for both dividers in a single pass.
		.fold([1, 2], |acc, entry| {
			let divider0_larger = entry < &dividers[0];
			let divider1_larger =
				// If divider 0 is larger than the entry, divider 1 is certainly
				// larger than the entry as well, as divider 1 is larger than
				// divider 0
				divider0_larger ||
				entry < &dividers[1];

			[
				acc[0] + divider0_larger as usize,
				acc[1] + divider1_larger as usize,
			]
		})
		.into_iter()
		.product::<usize>();
	println!("Part 2: {part2:?}");

	Ok(())
}

fn parse_packet_pairs(
	iter: impl IntoIterator<Item = String>,
) -> impl Iterator<Item = anyhow::Result<[Packet; 2]>> {
	let mut iter = iter.into_iter();

	iter::from_fn(move || {
		let fst = propagate!(iter.next().map(|x| x.parse()));
		let snd = propagate!(iter.next().map(|x| x.parse()));

		let result: [Packet; 2] = [fst, snd];
		iter.next(); // Optionally consume trailing blank line
		Some(Ok(result))
	})
}
