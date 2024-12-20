use std::{cell::Cell, cmp::Reverse, collections::BinaryHeap, io::stdin, iter};

use aoc2024::{Grid, Vector2D};

type Map = Grid<bool>;
type PathMap = Grid<Option<(usize, Vector2D)>>;

fn main() {
	let (map, start, end) = parse_input();
	let distances_to_end = distances_to_end(&map, end);

	let [part1, part2] = [2, 20].map(|max_cheat_length| {
		cheats(&distances_to_end, start, max_cheat_length)
			.filter(|(_, _, saved)| *saved >= 100)
			.count()
	});
	println!("Part 1: {part1}\nPart 2: {part2}");
}

fn cheats(
	distances_to_end: &PathMap,
	from: Vector2D,
	max_cheat_length: u64,
) -> impl Iterator<Item = (Vector2D, Vector2D, usize)> + '_ {
	path_to_end(distances_to_end, from).flat_map(move |from| {
		let ends = (2..=max_cheat_length)
			.flat_map(move |distance| from.manhattan_distances(distance))
			.filter(|index| distances_to_end.contains_index(*index));
		ends.filter_map(move |to| {
			let (normal_distance_to_end, _) = distances_to_end[from]?;
			let with_shortcut_distance_to_end =
				distances_to_end[to]?.0 + from.manhattan_distance(to) as usize;

			(with_shortcut_distance_to_end < normal_distance_to_end).then(|| {
				let saved = normal_distance_to_end - with_shortcut_distance_to_end;
				(from, to, saved)
			})
		})
	})
}

fn path_to_end(
	distances_to_end: &PathMap,
	mut from: Vector2D,
) -> impl Iterator<Item = Vector2D> + '_ {
	iter::from_fn(move || {
		let (_, to) = distances_to_end[from].expect("Reachable");

		if from == to {
			return None;
		}

		let result = Some(from);
		from = to;
		result
	})
}

fn distances_to_end(map: &Map, end: Vector2D) -> PathMap {
	let mut seen = Grid::new(map.width(), map.height(), None);

	let mut todo: BinaryHeap<(Reverse<usize>, Vector2D, Vector2D)> = BinaryHeap::new();
	todo.push((Reverse(0), end, end));

	while let Some((Reverse(cost), from, to)) = todo.pop() {
		if seen[to].is_some() {
			continue;
		}

		seen[to] = Some((cost, from));

		let next = map
			.neighbors(to)
			.filter_map(|next_pos| (!map[next_pos]).then_some((Reverse(cost + 1), to, next_pos)));
		todo.extend(next);
	}

	seen
}

fn parse_input() -> (Map, Vector2D, Vector2D) {
	let mut stdin = stdin().lines().map(Result::unwrap);

	/* Misuse a Cell to let the start and end escape the closure */
	let start = &Cell::new(None);
	let end = &Cell::new(None);

	let map: Map = (&mut stdin)
		.take_while(|line| !line.is_empty())
		.enumerate()
		.map(|(y, line)| {
			line.into_bytes()
				.into_iter()
				.enumerate()
				.map(move |(x, b)| match b {
					b'S' => {
						start.set(Some(Vector2D::new(x as _, y as _)));
						false
					}
					b'E' => {
						end.set(Some(Vector2D::new(x as _, y as _)));
						false
					}
					b'.' => false,
					b'#' => true,
					other => {
						panic!("Input error: unexpected {}", other as char);
					}
				})
		})
		.collect();

	(
		map,
		start.get().expect("Start in input"),
		end.get().expect("End in input"),
	)
}
