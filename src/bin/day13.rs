use std::{
	cmp::Ordering,
	io::stdin,
	iter::{self, Peekable},
	str::FromStr,
};

type PacketValue = u8;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
	Integer(PacketValue),
	List(Vec<Packet>),
}

impl FromStr for Packet {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		fn parse(iter: &mut Peekable<impl Iterator<Item = u8>>) -> Result<Packet, ()> {
			if matches!(iter.peek(), Some(c) if c.is_ascii_digit()) {
				let mut acc = 0;

				while let Some(c) = iter.next_if(u8::is_ascii_digit) {
					acc = acc * 10 + (c - b'0');
				}

				return Ok(Packet::Integer(acc));
			}

			if iter.next_if_eq(&b'[').is_some() {
				let mut children = Vec::new();

				while !matches!(iter.peek(), Some(b']')) {
					children.push(parse(iter)?);
					iter.next_if_eq(&b',');
				}

				if !matches!(iter.next(), Some(b']')) {
					return Err(());
				}

				return Ok(Packet::List(children));
			}

			Err(())
		}

		parse(&mut s.bytes().peekable())
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

fn main() {
	let packet_pairs = parse_packet_pairs(stdin().lines().flatten()).collect::<Vec<_>>();

	let part1 = packet_pairs
		.iter()
		.enumerate()
		.filter(|(_, p)| p[0] < p[1])
		.map(|(i, _)| i + 1)
		.sum::<usize>();
	println!("Part 1: {part1}");

	let dividers = [2, 6].map(|v| Packet::List(vec![Packet::List(vec![Packet::Integer(v)])]));

	let mut packets = packet_pairs
		.iter()
		.flatten()
		.chain(dividers.iter())
		.collect::<Vec<_>>();
	packets.sort();
	let part2 = packets
		.iter()
		.enumerate()
		.filter(|(_, packet)| dividers.contains(packet))
		.map(|(i, _)| i + 1)
		.product::<usize>();
	println!("Part 2: {part2}");
}

fn parse_packet_pairs(iter: impl IntoIterator<Item = String>) -> impl Iterator<Item = [Packet; 2]> {
	let mut iter = iter.into_iter();

	iter::from_fn(move || {
		let result = [iter.next()?.parse().unwrap(), iter.next()?.parse().unwrap()];
		iter.next(); // Optionally consume trailing blank line
		Some(result)
	})
}
