use std::iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid2D<T> {
	width: usize,
	items: Vec<T>,
}

pub type Index = (usize, usize);

impl<T> Grid2D<T> {
	pub fn with_size(width: usize, height: usize) -> Self
	where
		T: Default,
	{
		let mut items = Vec::with_capacity(width * height);
		items.resize_with(items.capacity(), || T::default());

		Self { width, items }
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.items.len() / self.width
	}

	fn calculate_index(&self, (x, y): Index) -> Option<usize> {
		if x < self.width() && y < self.height() {
			Some(y * self.width + x)
		} else {
			None
		}
	}

	pub fn keys(&self) -> impl Iterator<Item = Index> {
		let xs = 0..self.width();
		let ys = 0..self.height();
		ys.flat_map(move |y| xs.clone().zip(iter::repeat(y)))
	}

	pub fn values(&self) -> impl Iterator<Item = &T> {
		self.items.iter()
	}

	pub fn enumerate(&self) -> impl Iterator<Item = (Index, &T)> {
		self.keys().zip(self.items.iter())
	}

	pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Index, &mut T)> {
		self.keys().zip(self.items.iter_mut())
	}
}

pub trait Grid2DGet<Idx> {
	type Output;
	fn get(&self, index: Idx) -> Option<&Self::Output>;
}

pub trait Grid2DGetMut<Idx>: Grid2DGet<Idx> {
	fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Output>;
}

macro_rules! grid2d_impl_index {
	($($index: ty),+) => {
		$(
			impl<T> std::ops::Index<$index> for Grid2D<T> {
				type Output = T;

				fn index(&self, index: $index) -> &Self::Output {
					self.get(index).unwrap()
				}
			}

			impl<T> std::ops::IndexMut<$index> for Grid2D<T> {
				fn index_mut(&mut self, index: $index) -> &mut T {
					self.get_mut(index).unwrap()
				}
			}
		)*
	};
}
pub(crate) use grid2d_impl_index;

impl<T> Grid2DGet<Index> for Grid2D<T> {
	type Output = T;

	fn get(&self, index: Index) -> Option<&T> {
		let index = self.calculate_index(index);
		index.and_then(|index| self.items.get(index))
	}
}

impl<T> Grid2DGetMut<Index> for Grid2D<T> {
	fn get_mut(&mut self, index: Index) -> Option<&mut Self::Output> {
		let index = self.calculate_index(index);
		index.and_then(|index| self.items.get_mut(index))
	}
}

grid2d_impl_index!(Index);

impl<T> Grid2DGet<(isize, isize)> for Grid2D<T> {
	type Output = T;

	fn get(&self, index: (isize, isize)) -> Option<&T> {
		if index.0 < 0 || index.1 < 0 {
			return None;
		}

		self.get((index.0 as usize, index.1 as usize))
	}
}

impl<T> Grid2DGetMut<(isize, isize)> for Grid2D<T> {
	fn get_mut(&mut self, index: (isize, isize)) -> Option<&mut T> {
		if index.0 < 0 || index.1 < 0 {
			return None;
		}

		self.get_mut((index.0 as usize, index.1 as usize))
	}
}

grid2d_impl_index!((isize, isize));

impl<T, I: IntoIterator<Item = T>> FromIterator<I> for Grid2D<T> {
	fn from_iter<Iter: IntoIterator<Item = I>>(iter: Iter) -> Self {
		let iter = iter.into_iter();
		let mut items = Vec::new();
		let mut width: Option<usize> = None;

		for inner in iter {
			for item in inner {
				items.push(item);
			}

			if let Some(previous_width) = width {
				assert!(items.len() % previous_width == 0);
			} else {
				width.replace(items.len());
			}
		}

		Self {
			width: width.unwrap_or(0),
			items,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_create_grid_with_size() {
		let grid = Grid2D::<u8>::with_size(2, 3);
		assert_eq!(6, grid.items.len());

		assert_eq!(2, grid.width());
		assert_eq!(3, grid.height());
	}
}
