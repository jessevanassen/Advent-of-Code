use std::{collections::HashMap, io::stdin};

use aoc2024::Vector2D;
use itertools::Itertools;

fn main() {
	let map = parse_input();

	let part1 = count_unique_antinodes(&map, |a, b| {
		let distance = b - a;
		[a - distance, b + distance]
	});
	println!("Part 1: {part1}");

	let part2 = count_unique_antinodes(&map, |a, b| {
		let distance = b - a;

		let xs = (0..)
			.map(move |x| a - distance * x)
			.take_while(|pos| map.contains(*pos));
		let ys = (0..)
			.map(move |x| b + distance * x)
			.take_while(|pos| map.contains(*pos));

		xs.chain(ys)
	});
	println!("Part 2: {part2}");
}

fn count_unique_antinodes<II, F>(map: &Map, create_antinodes: F) -> usize
where
	F: Fn(Vector2D, Vector2D) -> II,
	II: IntoIterator<Item = Vector2D>,
{
	map.antennas
		.values()
		.flat_map(|antennas| {
			antennas
				.iter()
				.tuple_combinations()
				.flat_map(|(&a, &b)| create_antinodes(a, b))
		})
		.filter(|&pos| map.contains(pos))
		.unique()
		.count()
}

struct Map {
	antennas: HashMap<u8, Vec<Vector2D>>,
	width: i64,
	height: i64,
}

impl Map {
	pub fn contains(&self, Vector2D { x, y }: Vector2D) -> bool {
		(0..self.width).contains(&x) && (0..self.height).contains(&y)
	}
}

fn parse_input() -> Map {
	let mut antennas: HashMap<_, Vec<_>> = HashMap::new();
	let mut max_x = 0;
	let mut max_y = 0;

	for (y, row) in stdin().lines().map(Result::unwrap).enumerate() {
		for (x, b) in row.bytes().enumerate() {
			let x = x as i64;
			let y = y as i64;

			max_x = max_x.max(x);
			max_y = max_y.max(y);

			if b != b'.' {
				let position = Vector2D { x, y };
				antennas.entry(b).or_default().push(position);
			}
		}
	}

	Map {
		antennas,
		width: max_x + 1,
		height: max_y + 1,
	}
}
