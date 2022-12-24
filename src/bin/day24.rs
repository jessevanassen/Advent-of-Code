use std::{
	cmp::Reverse,
	collections::{BinaryHeap, HashSet},
	io::{stdin, Read},
	str::FromStr,
};

use aoc2022::{vec::IVec2D, Grid2D};

const START: IVec2D = IVec2D(0, -1);

fn main() -> anyhow::Result<()> {
	let map: Map = {
		let mut buffer = String::new();
		stdin().read_to_string(&mut buffer)?;
		buffer.parse()?
	};

	let points_to_travel = [map.start(), map.end(), map.start(), map.end()];
	let distances = map.shortest_journey(&points_to_travel);
	println!("Part 1: {}", distances[0]);
	println!("Part 2: {}", distances[2]);

	Ok(())
}

#[derive(Debug)]
pub struct Placement {
	position: IVec2D,
	direction: IVec2D,
}

pub struct Map {
	blizzards: Grid2D<Option<Placement>>,
}

impl Map {
	pub fn new(blizzards: Grid2D<Option<Placement>>) -> Self {
		Self { blizzards }
	}

	fn blizzards_at(&self, time: usize) -> Grid2D<bool> {
		let time = time as i32;
		let width = self.width() as i32;
		let height = self.height() as i32;

		let mut blizzards = Grid2D::with_size(self.width(), self.height());
		for Placement {
			position,
			direction,
		} in self.blizzards.values().flatten()
		{
			let x = (position.0 + direction.0 * time)
				.rem_euclid(width)
				.try_into()
				.unwrap();
			let y = (position.1 + direction.1 * time)
				.rem_euclid(height)
				.try_into()
				.unwrap();
			blizzards[(x, y)] = true;
		}
		blizzards
	}

	fn reachable_from(&self, position: IVec2D) -> Vec<IVec2D> {
		let width = self.width() as i32;
		let height = self.height() as i32;
		let x_range = 0..width;
		let y_range = 0..height;

		let mut reachable = Vec::with_capacity(5);

		for direction in [IVec2D::UP, IVec2D::RIGHT, IVec2D::DOWN, IVec2D::LEFT] {
			let position = position + direction;
			if x_range.contains(&position.0) && y_range.contains(&position.1) {
				reachable.push(position);
			}
		}

		if position.0 == 0 && position.1 == 0 {
			reachable.push(START);
		} else if position.0 == width - 1 && position.1 == height - 1 {
			reachable.push(self.end());
		} else {
			reachable.push(position);
		}

		reachable
	}

	pub fn start(&self) -> IVec2D {
		IVec2D(0, -1)
	}

	pub fn end(&self) -> IVec2D {
		IVec2D(self.width() as i32 - 1, self.height() as i32)
	}

	pub fn width(&self) -> usize {
		self.blizzards.width()
	}

	pub fn height(&self) -> usize {
		self.blizzards.height()
	}

	pub fn shortest_path(&self, from: IVec2D, to: IVec2D, time: usize) -> usize {
		let mut to_do = BinaryHeap::<(Reverse<usize>, (i32, i32))>::new();
		to_do.push((Reverse(time), from.into()));

		let mut seen = HashSet::new();
		let mut blizzard_cache: Vec<Option<Grid2D<bool>>> = Vec::new();

		while let Some((Reverse(distance), position)) = to_do.pop() {
			let position = position.into();

			let blizzards = {
				if blizzard_cache
					.get(distance)
					.and_then(|v| v.as_ref())
					.is_none()
				{
					blizzard_cache.resize(blizzard_cache.len().max(distance + 1), None);
					blizzard_cache[distance] = Some(self.blizzards_at(distance));
				}

				blizzard_cache[distance]
					.as_ref()
					.unwrap()
			};

			for reachable in self.reachable_from(position) {
				if seen.contains(&(distance, reachable)) {
					continue;
				}
				seen.insert((distance, reachable));

				if reachable
					.try_into()
					.ok()
					.and_then(|i| blizzards.get(i).copied())
					.unwrap_or(false)
				{
					continue;
				}

				if reachable == to {
					return distance;
				}

				to_do.push((Reverse(distance + 1), reachable.into()));
			}
		}

		unreachable!()
	}

	pub fn shortest_journey<'a>(&self, points: impl IntoIterator<Item = &'a IVec2D>) -> Vec<usize> {
		let mut points = points.into_iter();

		let mut distances = Vec::new();

		let mut from = match points.next() {
			Some(v) => *v,
			None => return distances,
		};

		for to in points.copied() {
			let distance = self.shortest_path(from, to, *distances.last().unwrap_or(&0));
			distances.push(distance);
			from = to;
		}

		distances
	}
}

impl FromStr for Map {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();
		let mut blizzards = Vec::new();

		let width = lines.next().unwrap().len() - 2;

		for (y, line) in lines
			.take_while(|line| line.as_bytes()[1] != b'#')
			.enumerate()
		{
			for (x, cell) in line
				.bytes()
				.skip(1)
				.take_while(|cell| cell != &b'#')
				.enumerate()
			{
				blizzards.push(if cell == b'.' {
					None
				} else {
					let direction = match cell {
						// Y-axis is reversed because of screen coordinates
						b'^' => IVec2D::DOWN,
						b'v' => IVec2D::UP,

						b'<' => IVec2D::LEFT,
						b'>' => IVec2D::RIGHT,
						other => anyhow::bail!("Unexpected cell value {other}"),
					};
					let position = (x, y).try_into()?;
					Some(Placement {
						position,
						direction,
					})
				});
			}
		}

		Ok(Map::new(Grid2D::from_values(blizzards, width)))
	}
}
