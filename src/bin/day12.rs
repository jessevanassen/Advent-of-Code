use std::io::stdin;

use aoc2024::{Grid, Vector2D};
use itertools::Itertools;

fn main() {
	let map = parse_input();
	let regions = divide_into_regions(&map);

	let (part1, part2) = regions.iter().fold((0, 0), |(part1, part2), region| {
		let perimeter = perimeter(&map, region).collect::<Vec<_>>();

		let line_segments = perimeter
			.iter()
			.tuple_combinations()
			.filter(|&(a, &b)| a.is_line_segment(b));
		let corners = perimeter.len() - line_segments.count();

		let area = region.len();
		(part1 + area * perimeter.len(), part2 + area * corners)
	});
	println!("Part 1: {part1}\nPart 2: {part2}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Segment {
	position: Vector2D,
	direction: Vector2D,
}

impl Segment {
	fn is_line_segment(self, other: Self) -> bool {
		self.direction == other.direction && self.position.manhattan_distance(other.position) == 1
	}
}

fn perimeter<'a>(map: &'a Grid<u8>, region: &'a [Vector2D]) -> impl Iterator<Item = Segment> + 'a {
	region.iter().flat_map(|&position| {
		different_neighbors(map, position).map(move |direction| Segment {
			position,
			direction,
		})
	})
}

fn different_neighbors(map: &Grid<u8>, index: Vector2D) -> impl Iterator<Item = Vector2D> + '_ {
	Vector2D::DIRECTIONS
		.into_iter()
		.filter(move |&neighbor| map.get(neighbor + index).is_none_or(|&v| v != map[index]))
}

fn divide_into_regions(map: &Grid<u8>) -> Vec<Vec<Vector2D>> {
	let mut regions: Vec<Vec<Vector2D>> = Vec::new();
	let mut visited = Grid::new(map.width(), map.height(), false);

	let mut todo = Vec::new();

	for index in map.indices() {
		if visited[index] {
			continue;
		}

		let mut region = Vec::new();

		todo.clear();
		todo.push(index);

		while let Some(index) = todo.pop() {
			if visited[index] {
				continue;
			}

			region.push(index);
			visited[index] = true;

			let connected = map
				.neighbors(index)
				.filter(|&neighbor| map[neighbor] == map[index]);
			todo.extend(connected);
		}

		regions.push(region);
	}

	regions
}

fn parse_input() -> Grid<u8> {
	stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| line.into_bytes())
		.collect()
}
