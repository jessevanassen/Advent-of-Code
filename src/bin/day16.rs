use std::{
	cmp::Reverse,
	collections::{BinaryHeap, HashMap},
	io::stdin,
};

use aoc2022::ByteSet;
use regex::Regex;

type FlowRate = usize;
type TravelCost = usize;
type Room = (FlowRate, Vec<TravelCost>);

fn main() {
	let rooms = parse_rooms(stdin().lines().flatten());
	let (part1, ..) = maximum_pressure(30, ByteSet::default(), &rooms);
	println!("Part 1: {part1}");

	let (first_relieved_pressure, opened_valves) = maximum_pressure(26, ByteSet::default(), &rooms);
	let (second_relieved_pressure, ..) = maximum_pressure(26, opened_valves, &rooms);
	println!("Part 2: {}", first_relieved_pressure + second_relieved_pressure);
}

fn maximum_pressure(time_remaining: usize, opened_valves: ByteSet, rooms: &[Room]) -> (FlowRate, ByteSet) {
	type Cache = HashMap<(usize, usize, ByteSet), usize>;

	fn maximum_pressure(
		time_remaining: usize,
		current_room: usize,
		opened_valves: ByteSet,
		rooms: &[Room],
		cache: &mut Cache,
	) -> (FlowRate, ByteSet) {
		/* The pressure will start to relieve after 1 turn. If there is only
		 * one turn remaining, the pressure won't relieve before the time
		 * is up. */
		if time_remaining <= 1 || opened_valves.len() as usize == rooms.len() {
			return (0, opened_valves);
		}

		let cache_key = (time_remaining, current_room, opened_valves);

		if let Some(v) = cache.get(&cache_key) {
			return (*v, opened_valves);
		}

		let (flow_rate, connections) = &rooms[current_room];

		let current_room_result = (!opened_valves.contains(current_room as _)).then(|| {
			let mut opened_valves = opened_valves;
			opened_valves.insert(current_room as _);

			let room_gain = flow_rate * (time_remaining - 1);

			let (relieved_pressure, opened_valves) = maximum_pressure(time_remaining - 1, current_room, opened_valves, rooms, cache);
			(room_gain + relieved_pressure, opened_valves)
		});

		let max_pressure = connections
			.iter()
			.enumerate()
			.filter(|(room, ..)| *room != current_room)
			.filter(|(_, travel_cost)| {
				matches!(
					time_remaining.checked_sub(**travel_cost),
					/* It takes at least 2 turns in addition to the travel cost
					 * to start to relieve pressure. */
					Some(v) if v > 1,
				)
			})
			.map(|(room, travel_cost)| {
				maximum_pressure(time_remaining - travel_cost, room, opened_valves, rooms, cache)
			})
			.chain(current_room_result)
			.max_by_key(|(relieved_pressure, ..)| *relieved_pressure)
			.unwrap_or((0, opened_valves));

		cache.insert(cache_key, max_pressure.0);

		max_pressure
	}

	maximum_pressure(
		time_remaining,
		0,
		opened_valves,
		rooms,
		&mut Cache::new(),
	)
}

fn parse_rooms(input: impl IntoIterator<Item = String>) -> Vec<Room> {
	const START: RoomKey = parse_room_key("AA");

	let mut lines = input
		.into_iter()
		.map(|line| parse_line(&line).unwrap())
		.collect::<Vec<_>>();
	lines.sort_by_key(|x| x.0);

	let rooms_with_valves = lines
		.iter()
		.filter(|(key, flow_rate, ..)| *flow_rate > 0 || key == &START)
		.map(|(key, flow_rate, ..)| (*key, *flow_rate))
		.collect::<Vec<_>>();

	rooms_with_valves
		.iter()
		.map(|&(room_key, flow_rate)| {
			let paths = shortest_paths(room_key, &lines);
			let connections = rooms_with_valves
				.iter()
				.map(|(room_key, ..)| *paths.get(room_key).unwrap())
				.collect();
			(flow_rate, connections)
		})
		.collect()
}

fn shortest_paths(from: RoomKey, lines: &[Line]) -> HashMap<RoomKey, usize> {
	let mut distances = HashMap::new();

	let mut queue: BinaryHeap<Reverse<(usize, RoomKey)>> = BinaryHeap::new();
	queue.push(Reverse((0, from)));

	while let Some(Reverse((distance, room_key))) = queue.pop() {
		distances.insert(room_key, distance);

		let connection_index = lines
			.binary_search_by_key(&&room_key, |(room_key, ..)| room_key)
			.unwrap();
		let connections = lines[connection_index]
			.2
			.iter()
			.filter(|room_key| !distances.contains_key(room_key));
		for connection in connections {
			queue.push(Reverse((distance + 1, *connection)));
		}
	}

	distances
}

type RoomKey = u16;
type Line = (RoomKey, FlowRate, Vec<RoomKey>);

fn parse_line(line: &str) -> Option<Line> {
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
	))
}

const fn parse_room_key(room: &str) -> RoomKey {
	let bytes = room.as_bytes();
	(bytes[0] - b'A') as RoomKey * 26 + (bytes[1] - b'A') as RoomKey
}
