use std::ops::{Index, IndexMut};

use crate::Vector2D;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
	items: Box<[T]>,
	width: u64,
}

impl<T> Grid<T> {
	pub fn new(width: u64, height: u64, items: T) -> Self
	where
		T: Clone,
	{
		Self {
			items: vec![items; (width * height) as usize].into_boxed_slice(),
			width,
		}
	}

	pub fn get(&self, index: Vector2D) -> Option<&T> {
		self.raw_index(index)
			.and_then(|index| self.items.get(index))
	}

	pub fn get_mut(&mut self, index: Vector2D) -> Option<&mut T> {
		self.raw_index(index)
			.and_then(|index| self.items.get_mut(index))
	}

	fn raw_index(&self, index: Vector2D) -> Option<usize> {
		self.contains_index(index)
			.then(|| (index.y * self.width() as i64 + index.x) as usize)
	}

	pub fn width(&self) -> u64 {
		self.width
	}

	pub fn height(&self) -> u64 {
		self.items.len() as u64 / self.width
	}

	pub fn indices(&self) -> impl Iterator<Item = Vector2D> + '_ {
		(0..self.height() as i64)
			.flat_map(|y| (0..self.width() as i64).map(move |x| Vector2D { x, y }))
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
		self.items.iter()
	}

	pub fn enumerate(&self) -> impl Iterator<Item = (Vector2D, &T)> + '_ {
		self.indices().zip(self.iter())
	}

	pub fn contains_index(&self, Vector2D { x, y }: Vector2D) -> bool {
		let x_range = 0..self.width() as i64;
		let y_range = 0..self.height() as i64;
		x_range.contains(&x) && y_range.contains(&y)
	}

	pub fn neighbors(&self, index: Vector2D) -> impl Iterator<Item = Vector2D> + '_ {
		const DIRECTIONS: [Vector2D; 4] = [
			Vector2D { x: 1, y: 0 },
			Vector2D { x: 0, y: -1 },
			Vector2D { x: -1, y: 0 },
			Vector2D { x: 0, y: 1 },
		];

		DIRECTIONS
			.into_iter()
			.map(move |direction| index + direction)
			.filter(|index| self.contains_index(*index))
	}
}

impl<T> Index<Vector2D> for Grid<T> {
	type Output = T;

	fn index(&self, index: Vector2D) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<T> IndexMut<Vector2D> for Grid<T> {
	fn index_mut(&mut self, index: Vector2D) -> &mut Self::Output {
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
			width: width as _,
		}
	}
}
