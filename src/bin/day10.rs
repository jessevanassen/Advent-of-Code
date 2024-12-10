use std::{collections::HashSet, io::stdin};

use aoc2024::{Grid, Vector2D};

fn main() {
	let map = parse_input();
	let starting_points = map
		.enumerate()
		.filter_map(|(i, v)| (*v == 0).then_some(i))
		.collect::<Vec<_>>();

	println!(
		"Part 1: {}",
		starting_points
			.iter()
			.map(|&c| reachable_summits(c, &map))
			.sum::<usize>()
	);
	println!(
		"Part 2: {}",
		starting_points
			.iter()
			.map(|&c| unique_trails(c, &map))
			.sum::<usize>()
	);
}

fn reachable_summits(index: Vector2D, map: &Grid<u8>) -> usize {
	fn reachable_summits(coord: Vector2D, map: &Grid<u8>, acc: &mut HashSet<Vector2D>) {
		if map[coord] == 9 {
			acc.insert(coord);
			return;
		}

		for next in uphill_neighbors(coord, map) {
			reachable_summits(next, map, acc);
		}
	}

	let mut acc = HashSet::with_capacity(9);
	reachable_summits(index, map, &mut acc);
	acc.len()
}

fn unique_trails(index: Vector2D, map: &Grid<u8>) -> usize {
	if map[index] == 9 {
		return 1;
	}

	uphill_neighbors(index, map)
		.map(|c| unique_trails(c, map))
		.sum()
}

fn uphill_neighbors(index: Vector2D, map: &Grid<u8>) -> impl Iterator<Item = Vector2D> + '_ {
	map.neighbors(index)
		.filter(move |&neighbor| map[neighbor] == map[index] + 1)
}

fn parse_input() -> Grid<u8> {
	stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| line.into_bytes().into_iter().map(|b| b - b'0'))
		.collect()
}
