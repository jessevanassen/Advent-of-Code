use std::{
	io::stdin,
	ops::Deref, collections::HashSet, cmp::Reverse,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
	x: usize,
	y: usize,
}

struct HeightMap(Vec<Vec<u8>>);

impl HeightMap {
	fn width(&self) -> usize {
		self.0.get(0).map_or(0, Vec::len)
	}

	fn height(&self) -> usize {
		self.0.len()
	}

	fn coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
		(0..self.height()).flat_map(|y| (0..self.width()).map(move |x| Coordinate { x, y }))
	}

	fn neighbors(&self, &Coordinate { x, y }: &Coordinate) -> Vec<Coordinate> {
		let mut neighbors = Vec::with_capacity(4);
		if y > 0 {
			neighbors.push(Coordinate { x, y: y - 1 });
		}
		if x > 0 {
			neighbors.push(Coordinate { x: x - 1, y });
		}
		if x < self.width() - 1 {
			neighbors.push(Coordinate { x: x + 1, y });
		}
		if y < self.height() - 1 {
			neighbors.push(Coordinate { x, y: y + 1 });
		}
		neighbors
	}

	fn get(&self, &Coordinate { x, y }: &Coordinate) -> Option<u8> {
		self.0.get(y).and_then(|row| row.get(x)).copied()
	}

	fn index(&self, coordinate: &Coordinate) -> u8 {
		self.get(coordinate).unwrap()
	}

	fn low_points(&self) -> Vec<Coordinate> {
		self
			.coordinates()
			.filter(|c| {
				let value = self.index(c);
				self
					.neighbors(&c)
					.iter()
					.all(|n| self.index(n) > value)
			})
			.collect()
	}

	fn basins(&self) -> Vec<HashSet<Coordinate>> {
		let low_points = self.low_points();
		let mut basins = Vec::with_capacity(low_points.len());

		for low_point in low_points {
			let mut basin = HashSet::new();
			let mut queue = vec![low_point];

			while let Some(coordinate) = queue.pop() {
				queue.extend(
					self.neighbors(&coordinate)
						.into_iter()
						.filter(|c| {
							let value = self.index(c);
							value < 9 && !basin.contains(c)
						})
				);
				basin.insert(coordinate);
			}

			basins.push(basin);
		}

		basins
	}
}

impl<'a, T, I> From<T> for HeightMap
where
	I: Deref<Target = str>,
	T: Iterator<Item = I>,
{
	fn from(input: T) -> Self {
		let values: Vec<Vec<u8>> = input
			.map(|line| line.as_bytes().iter().map(|c| c - b'0').collect())
			.collect();
		Self(values)
	}
}

fn main() {
	let height_map: HeightMap = stdin().lines().flatten().into();

	let risk: usize = height_map
		.low_points()
		.iter()
		.map(|c| height_map.index(c) as usize + 1)
		.sum()
		;
	println!("Part 1: {risk:?}");

	let mut basins = height_map.basins()
		.iter()
		.map(HashSet::len)
		.collect::<Vec<_>>()
		;
	basins.sort();
	let sizes = basins.iter()
		.rev()
		.take(3)
		.fold(1, |x, y| x * y)
		;
	println!("Part 2: {x}");
}
