use std::{collections::HashMap, io::stdin};

use aoc2022::ByteSet;
use regex::Regex;

type Room = (
	u32,     // Flow rate when valve is opened
	Vec<u8>, // Connected rooms
);

fn main() {
	let rooms = parse_input(stdin().lines().flatten());

	const TIME_LIMIT: u32 = 30;

	let result = do_it_daddy(TIME_LIMIT, &rooms);
	dbg!(result);
}

fn do_it_daddy(time_remaining: u32, rooms: &[Room]) -> u32 {
	type Cache = HashMap<(u32, u8, ByteSet), u32>;

	fn recurse(
		time_remaining: u32,
		current_room: u8,
		visited: ByteSet,
		rooms: &[Room],
		cache: &mut Cache,
	) -> u32 {
		if time_remaining == 0 {
			return 0;
		}

		let cache_key = (time_remaining, current_room, visited);

		if let Some(&cached) = cache.get(&cache_key) {
			return cached;
		}

		let time_remaining = time_remaining - 1;

		let (flow, connected) = &rooms[current_room as usize];

		let can_open_valve = *flow > 0 && !visited.contains(current_room);

		let room_result = can_open_valve.then(|| {
			let mut visited = visited;
			visited.insert(current_room);
			let room_produces = flow * time_remaining;
			room_produces + recurse(time_remaining, current_room, visited, rooms, cache)
		});

		let result = connected
			.iter()
			.map(|&room| recurse(time_remaining, room, visited, rooms, cache))
			.chain(room_result)
			.max()
			.unwrap_or(0);

		cache.insert(cache_key, result);

		result
	}

	recurse(
		time_remaining,
		0,
		ByteSet::new(),
		rooms,
		&mut HashMap::new(),
	)
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
