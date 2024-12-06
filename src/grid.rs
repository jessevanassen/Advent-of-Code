use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
	items: Box<[T]>,
	width: usize,
}

impl<T> Grid<T> {
	pub fn new(width: usize, height: usize, items: T) -> Self
	where
		T: Clone,
	{
		Self {
			items: vec![items; width * height].into_boxed_slice(),
			width,
		}
	}

	pub fn get(&self, index: impl Into<(usize, usize)>) -> Option<&T> {
		let (x, y) = index.into();

		if x >= self.width() {
			return None;
		}

		self.items.get(y * self.width + x)
	}

	pub fn get_mut(&mut self, index: impl Into<(usize, usize)>) -> Option<&mut T> {
		let (x, y) = index.into();

		if x >= self.width() {
			return None;
		}

		self.items.get_mut(y * self.width + x)
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.items.len() / self.width
	}

	pub fn indices(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
		(0..self.height()).flat_map(|y| (0..self.width()).map(move |x| (x, y)))
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
		self.items.iter()
	}

	pub fn enumerate(&self) -> impl Iterator<Item = ((usize, usize), &T)> + '_ {
		self.indices().zip(self.iter())
	}
}

impl<T, Idx: Into<(usize, usize)>> Index<Idx> for Grid<T> {
	type Output = T;

	fn index(&self, index: Idx) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<T, Idx: Into<(usize, usize)>> IndexMut<Idx> for Grid<T> {
	fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

impl<T> Default for Grid<T> {
	fn default() -> Self {
		Self {
			items: Default::default(),
			width: Default::default(),
		}
	}
}

impl<T, Row: IntoIterator<Item = T>> FromIterator<Row> for Grid<T> {
	fn from_iter<U: IntoIterator<Item = Row>>(iter: U) -> Self {
		let mut iter = iter.into_iter();

		let mut items = Vec::<T>::new();

		let Some(first_row) = iter.next() else {
			return Self::default();
		};

		items.extend(first_row);
		let width = items.len();

		for row in iter {
			let previous_len = items.len();
			items.extend(row);

			if items.len() - previous_len != width {
				panic!("Input is not rectangular");
			}
		}

		Self {
			items: items.into_boxed_slice(),
			width,
		}
	}
}
