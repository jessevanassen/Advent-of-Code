use std::{collections::HashMap, io::stdin};

use aoc2022::ByteSet;
use regex::Regex;

type Room = (
	u32,     // Flow rate when valve is opened
	Vec<u8>, // Connected rooms
);

fn main() {
	let rooms = parse_input(stdin().lines().flatten());

	const TIME_LIMIT: u32 = 26;

	let result = do_it_daddy(TIME_LIMIT, &rooms);
	dbg!(result);
}

fn do_it_daddy(time_remaining: u32, rooms: &[Room]) -> u32 {
	type Cache = HashMap<(u32, [u8; 2], ByteSet), u32>;

	fn recurse(
		time_remaining: u32,
		current_rooms: [u8; 2],
		visited: ByteSet,
		rooms: &[Room],
		cache: &mut Cache,
	) -> u32 {
		if time_remaining == 0 {
			return 0;
		}

		let cache_key = (time_remaining, current_rooms, visited);

		if let Some(&cached) = cache.get(&cache_key) {
			return cached;
		}

		let time_remaining = time_remaining - 1;

		let to_visit = current_rooms.map(|room| {
			let (flow, connected) = &rooms[room as usize];
			let can_open_valve = *flow > 0 && !visited.contains(room);
			connected
				.iter()
				.map(|room| (*room, 0, visited))
				.chain(can_open_valve.then(|| {
					let mut visited = visited;
					visited.insert(room);
					let additional = time_remaining * flow;
					(room, additional, visited)
				}))
				.collect::<Vec<_>>()
		});

		let result = combinations(&to_visit[0], &to_visit[1])
			.filter(|(x, y)| !(x.0 == y.0 && x.1 > 0 && y.1 > 0))
			.map(|(&x, &y)| ([x.0, y.0], x.1 + y.1, x.2.union(&y.2)))
			.map(|(mut current_rooms, to_add, visited)| {
				current_rooms.sort();
				to_add + recurse(time_remaining, current_rooms, visited, rooms, cache)
			})
			.max()
			.unwrap();

		cache.insert(cache_key, result);

		result
	}

	recurse(
		time_remaining,
		[0; 2],
		ByteSet::new(),
		rooms,
		&mut HashMap::new(),
	)
}

fn combinations<'a, T>(xs: &'a [T], ys: &'a [T]) -> impl Iterator<Item = (&'a T, &'a T)> + 'a {
	(0..xs.len())
		.flat_map(|i| (0..ys.len()).map(move |j| (i, j)))
		.map(|(i, j)| (&xs[i], &ys[j]))
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_combinations() {
		let combinations = combinations(&[1, 2, 3], &[4, 5]).collect::<Vec<_>>();
		assert_eq!(6, combinations.len());
		assert_eq!((&1, &4), combinations[0]);
		assert_eq!((&1, &5), combinations[1]);
		assert_eq!((&2, &4), combinations[2]);
		assert_eq!((&2, &5), combinations[3]);
		assert_eq!((&3, &4), combinations[4]);
		assert_eq!((&3, &5), combinations[5]);
	}
}
