use std::io::stdin;

use regex::Regex;

type Room = (
	u32,     // Flow rate when valve is opened
	Vec<u8>, // Connected rooms
);

fn main() {
	let rooms = parse_input(stdin().lines().flatten());
	dbg!(&rooms);
}

fn parse_input(input: impl IntoIterator<Item = String>) -> Vec<Room> {
	let mut lines = input
		.into_iter()
		.map(|line| parse_line(&line).unwrap())
		.collect::<Vec<_>>();
	lines.sort_by_key(|x| x.0);

	lines
		.iter()
		.map(|(_, (flow_rate, connected))| {
			let names = connected
				.iter()
				.map(|name| {
					lines
						.binary_search_by_key(&name, |(name, _)| name)
						.map(|x| x as u8)
						.unwrap()
				})
				.collect();
			(*flow_rate, names)
		})
		.collect()
}

fn parse_line(line: &str) -> Option<(u16, (u32, Vec<u16>))> {
	lazy_static::lazy_static! {
		static ref RE: Regex = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
	}

	let captures = RE.captures(line)?;
	let mut captures = captures
		.iter()
		.skip(1)
		.map(|capture| capture.unwrap().as_str());

	Some((
		parse_room_key(captures.next().unwrap()),
		(
			captures
				.next()
				.unwrap()
				.parse()
				.unwrap(),
			captures
				.next()
				.unwrap()
				.split(", ")
				.map(parse_room_key)
				.collect(),
		),
	))
}

fn parse_room_key(room: &str) -> u16 {
	room.bytes()
		.into_iter()
		.map(|c| c - b'A')
		.fold(0, |acc, c| acc * 26 + c as u16)
}
