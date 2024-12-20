use std::{cmp::Reverse, collections::BinaryHeap, io::stdin};

use aoc2024::{binary_search, Grid, Vector2D};

type Map = Grid<Option<usize>>;

fn main() {
	let map = parse_input();

	let part1 = find_shortest_path(&map, 1024).expect("Expect answer for part 1");
	println!("Part 1: {part1}");

	let part2 = find_first_blocking_corrupted_byte(&map).expect("Expect an answer for part 2");
	println!("Part 2: {},{}", part2.x, part2.y);
}

fn find_shortest_path(map: &Map, corrupted_bytes_len: usize) -> Option<usize> {
	let target = Vector2D {
		x: map.width() as i64 - 1,
		y: map.height() as i64 - 1,
	};

	let mut todo = BinaryHeap::<(Reverse<usize>, Vector2D)>::new();
	todo.push((Reverse(0), Vector2D::new(0, 0)));
	let mut seen = Grid::new(map.width(), map.height(), false);

	while let Some((Reverse(cost), index)) = todo.pop() {
		if index == target {
			return Some(cost);
		}

		if seen[index] {
			continue;
		}

		seen[index] = true;

		let next = map.neighbors(index).filter_map(|index| {
			let cost = cost + 1;
			let is_accessible = map[index].is_none_or(|v| v >= corrupted_bytes_len);
			is_accessible.then_some((Reverse(cost), index))
		});
		todo.extend(next);
	}

	None
}

fn find_first_blocking_corrupted_byte(map: &Map) -> Option<Vector2D> {
	let max_corrupted_bytes = map.iter().flatten().count();

	let path_is_blocked =
		|corrupted_bytes_len| find_shortest_path(map, corrupted_bytes_len).is_none();
	binary_search::find_leftmost(1..max_corrupted_bytes, path_is_blocked).map(
		|corrupted_bytes_len| {
			map.position(|v| *v == Some(corrupted_bytes_len - 1))
				.expect("Matching index should exist")
		},
	)
}

fn parse_input() -> Map {
	let mut map = Map::new(71, 71, None);
	// let mut result = Map::new(7, 7, None);

	let indices = stdin().lines().map(Result::unwrap).map(|line| {
		let (x, y) = line.split_once(',').expect("Pair per line");
		Vector2D::new(
			x.parse().expect("Expect x-coord to be an int"),
			y.parse().expect("Expect y-coord to be an int"),
		)
	});

	for (i, index) in indices.enumerate() {
		map[index] = Some(i);
	}

	map
}
