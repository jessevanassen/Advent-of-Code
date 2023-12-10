pub mod bitset;
pub mod grid2d;
pub mod range;
pub mod vector2d;

use grid2d::{Grid2D, Grid2DGet, Grid2DGetMut};
use vector2d::Vector2D;

impl<T> Grid2DGet<Vector2D> for Grid2D<T> {
	type Output = T;

	fn get(&self, index: Vector2D) -> Option<&Self::Output> {
		let index: (isize, isize) = index.into();
		self.get(index)
	}
}

impl<T> Grid2DGetMut<Vector2D> for Grid2D<T> {
	fn get_mut(&mut self, index: Vector2D) -> Option<&mut Self::Output> {
		let index: (isize, isize) = index.into();
		self.get_mut(index)
	}
}

grid2d::grid2d_impl_index!(Vector2D);

