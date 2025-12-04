#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
	items: Vec<T>,
	width: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Index {
	pub row: usize,
	pub column: usize,
}

impl From<(usize, usize)> for Index {
	fn from((row, column): (usize, usize)) -> Self {
		Self { row, column }
	}
}

impl<T> Grid<T> {
	pub const fn empty() -> Self {
		Grid {
			items: Vec::new(),
			width: 0,
		}
	}

	pub const fn width(&self) -> usize {
		self.width
	}

	pub const fn height(&self) -> usize {
		self.items.len() / self.width()
	}

	pub const fn len(&self) -> usize {
		self.items.len()
	}

	pub const fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn indices(&self) -> impl Iterator<Item = Index> {
		(0..self.height())
			.flat_map(|row| (0..self.width()).map(move |column| Index { row, column }))
	}

	const fn to_vec_index(&self, Index { row, column }: Index) -> usize {
		row * self.width() + column
	}

	pub fn get(&self, index: Index) -> Option<&T> {
		let index = self.to_vec_index(index);
		self.items.get(index)
	}

	pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
		let index = self.to_vec_index(index);
		self.items.get_mut(index)
	}

	pub const fn has_index(&self, Index { row, column }: Index) -> bool {
		row < self.height() && column < self.width()
	}

	pub fn enumerate(&self) -> impl Iterator<Item = (Index, &T)> {
		self.indices().map(|index| (index, &self[index]))
	}
}

impl<T> std::ops::Index<Index> for Grid<T> {
	type Output = T;

	fn index(&self, index: Index) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<T> std::ops::IndexMut<Index> for Grid<T> {
	fn index_mut(&mut self, index: Index) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
#[error("Unbalanced grid: not all rows have the same length")]
pub struct UnbalancedGridError;

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
	type Error = UnbalancedGridError;

	fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
		if value.is_empty() {
			return Ok(Self::empty());
		}

		let mut grid = Grid {
			items: Vec::with_capacity(value.len() * value[0].len()),
			width: value[0].len(),
		};

		for mut row in value {
			if row.len() != grid.width {
				return Err(UnbalancedGridError);
			}

			grid.items.append(&mut row);
		}

		Ok(grid)
	}
}
