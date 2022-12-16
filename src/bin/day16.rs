use std::{io::stdin, collections::HashMap};

use regex::Regex;

type Room = (String, u32, Vec<String>);
fn main() {
	let rooms = stdin()
		.lines()
		.flatten()
		.map(|line| parse_line(&line).unwrap())
		.map(|room| (room.0, (room.1, room.2)))
		.collect::<HashMap<_, _>>();
	dbg!(rooms);
}

fn parse_line(line: &str) -> Option<Room> {
	lazy_static::lazy_static! {
		static ref RE: Regex = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
	}

	let captures = RE.captures(line)?;
	let mut captures = captures
		.iter()
		.skip(1)
		.map(|capture| capture.unwrap().as_str());

	Some((
		captures.next().unwrap().to_string(),
		captures
			.next()
			.unwrap()
			.parse()
			.unwrap(),
		captures
			.next()
			.unwrap()
			.split(", ")
			.map(|x| x.to_string())
			.collect(),
	))
}
