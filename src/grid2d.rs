use std::ops::{Index, IndexMut};

type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid2D<T> {
	items: Vec<T>,
	width: usize,
}

impl<T> Grid2D<T> {
	pub fn from_values(items: Vec<T>, width: usize) -> Self {
		if width == 0 && !items.is_empty() || items.len() % width != 0 {
			panic!("Grid isn't a rectangle");
		}

		Self { items, width }
	}

	pub fn row(&self, y: usize) -> impl DoubleEndedIterator<Item = &T> + '_ {
		(0..self.width).flat_map(move |x| self.get((x, y)))
	}

	pub fn column(&self, x: usize) -> impl DoubleEndedIterator<Item = &T> + '_ {
		(0..self.height()).flat_map(move |y| self.get((x, y)))
	}

	pub fn last_row(&self) -> impl DoubleEndedIterator<Item = &T> + '_ {
		self.row(self.height() - 1)
	}

	pub fn last_column(&self) -> impl DoubleEndedIterator<Item = &T> + '_ {
		self.column(self.height() - 1)
	}
}

impl<T: Default> Grid2D<T> {
	pub fn with_size(width: usize, height: usize) -> Self {
		let mut items = Vec::with_capacity(width * height);
		items.resize_with(items.capacity(), T::default);
		Self { items, width }
	}
}

impl<T, Iter: IntoIterator<Item = T>> FromIterator<Iter> for Grid2D<T> {
	fn from_iter<U: IntoIterator<Item = Iter>>(iter: U) -> Self {
		let mut width = None;
		let mut items = Vec::new();

		for item in iter {
			let old_size = items.len();
			items.extend(item.into_iter());

			let items_added = items.len() - old_size;
			match width {
				Some(width) => {
					if width != items_added {
						panic!("Source iterator yields iterators of different sizes");
					}
				}
				None => {
					width = Some(items_added);
				}
			}
		}

		Self::from_values(items, width.unwrap_or(0))
	}
}

impl<T> Index<Coord> for Grid2D<T> {
	type Output = T;

	fn index(&self, index: Coord) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<T> IndexMut<Coord> for Grid2D<T> {
	fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

impl<T> Grid2D<T> {
	pub fn height(&self) -> usize {
		if self.width > 0 {
			self.items.len() / self.width
		} else {
			0
		}
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn get(&self, (x, y): Coord) -> Option<&T> {
		self.items.get(y * self.width + x)
	}

	pub fn get_mut(&mut self, (x, y): Coord) -> Option<&mut T> {
		self.items.get_mut(y * self.width + x)
	}

	pub fn set(&mut self, index: Coord, value: T) {
		self[index] = value;
	}

	pub fn indices(&self) -> impl Iterator<Item = Coord> + '_ {
		(0..self.height()).flat_map(move |y| (0..self.width()).map(move |x| (x, y)))
	}

	pub fn enumerate(&self) -> impl Iterator<Item = (Coord, &T)> {
		self.indices()
			.map(|index| (index, self.index(index)))
	}

	pub fn values(&self) -> impl Iterator<Item = &T> {
		self.items.iter()
	}

	pub fn contains_coordinate(&self, (x, y): Coord) -> bool {
		y < self.height() && x < self.width()
	}

	pub fn neighbors(&self, (x, y): Coord) -> impl Iterator<Item = Coord> {
		let width = self.width();
		let height = self.height();

		let horizontal = ((x.saturating_sub(1))..=(x + 1))
			.filter(move |&_x| _x != x && _x < width)
			.map(move |_x| (_x, y));
		let vertical = ((y.saturating_sub(1))..=(y + 1))
			.filter(move |&_y| _y != y && _y < height)
			.map(move |_y| (x, _y));

		Iterator::chain(horizontal, vertical)
	}

	pub fn len(&self) -> usize {
		self.width * self.height()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}
