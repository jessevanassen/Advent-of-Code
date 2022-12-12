use std::{cmp::Reverse, collections::BinaryHeap, io::stdin, time::Instant};

use aoc2022::{BitSet, Grid2D};

type HeightMap = Grid2D<u8>;
type Coord = (usize, usize);

fn main() {
	let time = Instant::now();

	let Input {
		height_map,
		start,
		end,
	} = parse_input();

	let distances_to = distances_to(end, &height_map);

	let part1 = distances_to[start].unwrap();
	let part2 = distances_to
		.enumerate()
		.filter(|(coord, _)| height_map[*coord] == 0)
		.filter_map(|(_, distance)| *distance)
		.min()
		.unwrap();

	let time = Instant::now() - time;

	println!("Time: {}ms", time.as_nanos() as f64 * 1.0E-6);
	println!("Part 1: {part1}");
	println!("Part 2: {part2}",);
}

fn distances_to(to: Coord, height_map: &HeightMap) -> Grid2D<Option<usize>> {
	let mut distances = Grid2D::<Option<usize>>::with_size(height_map.width(), height_map.height());
	distances[to] = Some(0);

	let mut seen = BitSet::with_capacity(height_map.len());
	seen.insert(to.1 * height_map.width() + to.0);

	let mut todo: BinaryHeap<Reverse<(usize, Coord)>> = BinaryHeap::new();
	todo.push(Reverse((0, to)));

	while let Some(Reverse((distance, coord))) = todo.pop() {
		for n in height_map.neighbors(coord) {
			let distance = distance + 1;
			let index = n.1 * height_map.width() + n.0;
			if !seen.contains(index) && height_map[n] + 1 >= height_map[coord] {
				todo.push(Reverse((distance, n)));
				seen.insert(index);
				distances[n] = Some(distance);
			}
		}
	}

	distances
}

struct Input {
	height_map: HeightMap,
	start: Coord,
	end: Coord,
}
fn parse_input() -> Input {
	let mut start: Coord = Default::default();
	let mut end: Coord = Default::default();

	let height_map = {
		stdin()
			.lines()
			.flatten()
			.enumerate()
			.map(|(y, line)| {
				line.into_bytes()
					.into_iter()
					.enumerate()
					.map(|(x, b)| match b {
						b'S' => {
							start = (x, y);
							0
						}
						b'E' => {
							end = (x, y);
							b'z' - b'a'
						}
						b'a'..=b'z' => b - b'a',
						other => panic!("Invalid character {}", other as char),
					})
					.collect::<Vec<_>>()
			})
			.collect::<Grid2D<_>>()
	};

	Input {
		height_map,
		start,
		end,
	}
}
