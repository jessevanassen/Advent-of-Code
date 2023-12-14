use solution::*;
use std::{
	collections::HashMap,
	io::{stdin, Read},
	str::FromStr,
};

fn main() {
	let input = {
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		buf
	};

	let platform = Platform::from_str(&input).unwrap();

	{
		let mut platform = platform.clone();
		platform.tilt_north();

		println!("Part 1: {}", platform.total_load());
	}

	{
		let mut platform = platform.clone();

		// Is there a better key for this map? Will take ~10kb per entry this way...
		let mut cycles = HashMap::<Platform, usize>::new();

		const LIMIT: usize = 1000000000;

		let mut i = 0;
		while i < LIMIT {
			platform.cycle();

			if let Some(cycle_start) = cycles.get(&platform) {
				let cycle_size = i - cycle_start;
				let iterations_remaining = LIMIT - i;
				let cycles_remaining = iterations_remaining / cycle_size;
				let iterations_to_skip = cycles_remaining * cycle_size;
				i += iterations_to_skip;
			} else {
				cycles.insert(platform.clone(), i);
			}

			i += 1;
		}

		println!("Part 2: {}", platform.total_load());
	}
}

mod solution {
	use std::{
		fmt::{Display, Write},
		ops::{Index, IndexMut},
		str::FromStr,
	};

	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	pub struct Platform {
		values: Vec<Tile>,
		width: usize,
	}

	type Idx = (usize, usize);

	impl Platform {
		pub fn total_load(&self) -> usize {
			self.enumerate()
				.filter(|(_, tile)| **tile == Tile::Round)
				.map(|((_, y), _)| self.height() - y)
				.sum()
		}

		pub fn swap(&mut self, a: Idx, b: Idx) {
			(self[a], self[b]) = (self[b], self[a]);
		}

		pub fn width(&self) -> usize {
			self.width
		}

		pub fn height(&self) -> usize {
			self.values.len() / self.width()
		}

		pub fn get(&self, index: Idx) -> Option<&Tile> {
			self.values.get(index.0 + index.1 * self.width())
		}

		pub fn get_mut(&mut self, index: Idx) -> Option<&mut Tile> {
			let width = self.width();
			self.values.get_mut(index.0 + index.1 * width)
		}

		pub fn keys(&self) -> impl Iterator<Item = Idx> {
			let xs = 0..self.width();
			let ys = 0..self.height();

			ys.flat_map(move |y| xs.clone().map(move |x| (x, y)))
		}

		pub fn enumerate(&self) -> impl Iterator<Item = (Idx, &Tile)> + '_ {
			self.keys().zip(self.values.iter())
		}

		pub fn cycle(&mut self) {
			self.tilt_north();
			self.tilt_west();
			self.tilt_south();
			self.tilt_east();
		}

		pub fn tilt_north(&mut self) {
			for y in 1..(self.height()) {
				for x in 0..self.width() {
					if self[(x, y)] != Tile::Round {
						continue;
					}

					let dest_y = (0..y)
						.rev()
						.find(|&y| self[(x, y)] != Tile::Empty)
						.map(|y| y + 1)
						.unwrap_or(0);
					self.swap((x, dest_y), (x, y));
				}
			}
		}

		fn tilt_west(&mut self) {
			for y in 0..self.height() {
				for x in 1..self.width() {
					if self[(x, y)] != Tile::Round {
						continue;
					}

					let dest_x = (0..x)
						.rev()
						.find(|&x| self[(x, y)] != Tile::Empty)
						.map(|x| x + 1)
						.unwrap_or(0);
					self.swap((dest_x, y), (x, y));
				}
			}
		}

		fn tilt_south(&mut self) {
			for y in (0..(self.height() - 1)).rev() {
				for x in 0..self.width() {
					if self[(x, y)] != Tile::Round {
						continue;
					}

					let dest_y = ((y + 1)..self.height())
						.find(|&y| self[(x, y)] != Tile::Empty)
						.map(|y| y - 1)
						.unwrap_or(self.height() - 1);
					self.swap((x, dest_y), (x, y));
				}
			}
		}

		fn tilt_east(&mut self) {
			for y in 0..self.height() {
				for x in (0..(self.width() - 1)).rev() {
					if self[(x, y)] != Tile::Round {
						continue;
					}

					let dest_x = ((x + 1)..self.width())
						.find(|&x| self[(x, y)] != Tile::Empty)
						.map(|x| x - 1)
						.unwrap_or(self.width() - 1);
					self.swap((dest_x, y), (x, y));
				}
			}
		}
	}

	impl Index<Idx> for Platform {
		type Output = Tile;

		fn index(&self, index: Idx) -> &Self::Output {
			self.get(index).unwrap()
		}
	}

	impl IndexMut<Idx> for Platform {
		fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
			self.get_mut(index).unwrap()
		}
	}

	impl FromStr for Platform {
		type Err = anyhow::Error;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			let mut lines = s.lines().peekable();

			let width = lines
				.peek()
				.map(|line| line.len())
				.ok_or_else(|| anyhow::anyhow!("No input"))?;

			let mut values = Vec::with_capacity(width * width);

			for line in lines {
				for c in line.bytes() {
					values.push(Tile::try_from(c)?);
				}
			}

			Ok(Platform { width, values })
		}
	}

	impl Display for Platform {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			for y in 0..self.height() {
				for x in 0..self.width() {
					self[(x, y)].fmt(f)?;
				}
				f.write_char('\n')?;
			}
			Ok(())
		}
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub enum Tile {
		Cube,
		Round,
		Empty,
	}

	impl TryFrom<u8> for Tile {
		type Error = anyhow::Error;

		fn try_from(value: u8) -> Result<Self, Self::Error> {
			Ok(match value {
				b'#' => Self::Cube,
				b'O' => Self::Round,
				b'.' => Self::Empty,
				_ => anyhow::bail!("Unexpected symbol '{}'", value as char),
			})
		}
	}

	impl Display for Tile {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.write_char(match self {
				Self::Cube => '#',
				Self::Round => 'O',
				Self::Empty => '.',
			})?;
			Ok(())
		}
	}
}
