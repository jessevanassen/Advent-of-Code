use std::{collections::HashSet, io::stdin};

use aoc2024::Grid;

type Coord = (usize, usize);

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
			.map(|&c| reachable_tops(c, &map))
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

fn reachable_tops(coord: Coord, map: &Grid<u8>) -> usize {
	fn reachable_tops(coord: Coord, map: &Grid<u8>, acc: &mut HashSet<Coord>) {
		if map[coord] == 9 {
			acc.insert(coord);
			return;
		}

		for next in uphill_neighbors(coord, map) {
			reachable_tops(next, map, acc);
		}
	}

	let mut acc = HashSet::with_capacity(9);
	reachable_tops(coord, map, &mut acc);
	acc.len()
}

fn unique_trails(coord: Coord, map: &Grid<u8>) -> usize {
	if map[coord] == 9 {
		return 1;
	}

	uphill_neighbors(coord, map)
		.map(|c| unique_trails(c, map))
		.sum()
}

fn neighbors((x, y): Coord, map: &Grid<u8>) -> impl Iterator<Item = Coord> + '_ {
	const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

	DIRECTIONS.into_iter().flat_map(move |direction| {
		let x = x as isize + direction.0;
		let y = y as isize + direction.1;

		((0..map.width() as isize).contains(&x) && (0..map.height() as isize).contains(&y))
			.then_some((x as usize, y as usize))
	})
}

fn uphill_neighbors(coord: Coord, map: &Grid<u8>) -> impl Iterator<Item = Coord> + '_ {
	neighbors(coord, map).filter(move |&neighbor| map[neighbor] == map[coord] + 1)
}

fn parse_input() -> Grid<u8> {
	stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| line.into_bytes().into_iter().map(|b| b - b'0'))
		.collect()
}
