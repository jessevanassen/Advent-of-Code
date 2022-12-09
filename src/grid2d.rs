use std::ops::Index;

type Coord = (usize, usize);

pub struct Grid2D<T>(Vec<Vec<T>>);

impl<T> From<Vec<Vec<T>>> for Grid2D<T> {
	fn from(value: Vec<Vec<T>>) -> Self {
		Self(value)
	}
}

impl<T> Index<Coord> for Grid2D<T> {
	type Output = T;

	fn index(&self, index: Coord) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<T> Grid2D<T> {
	pub fn height(&self) -> usize {
		self.0.len()
	}

	pub fn width(&self) -> usize {
		self.0.get(0).map(Vec::len).unwrap_or(0)
	}

	pub fn get(&self, (x, y): Coord) -> Option<&T> {
		self.0.get(y).and_then(|row| row.get(x))
	}

	pub fn indices(&self) -> impl Iterator<Item = Coord> + '_ {
		(0..self.height()).flat_map(move |y| (0..self.width()).map(move |x| (x, y)))
	}

	pub fn enumerate(&self) -> impl Iterator<Item = (Coord, &T)> {
		self.indices()
			.map(|index| (index, self.index(index)))
	}
}
