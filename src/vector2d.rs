use std::ops::{Mul, MulAssign};

#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Hash,
	derive_more::Add,
	derive_more::AddAssign,
	derive_more::Sub,
	derive_more::SubAssign,
	derive_more::From,
	derive_more::Into,
)]
#[from((i64, i64))]
#[into((i64, i64))]
pub struct Vector2D {
	pub x: i64,
	pub y: i64,
}

impl Vector2D {
	pub const HORIZONTAL_DIRECTIONS: [Vector2D; 2] =
		[Vector2D { x: -1, y: 0 }, Vector2D { x: 1, y: 0 }];
	pub const VERTICAL_DIRECTIONS: [Vector2D; 2] =
		[Vector2D { x: 0, y: -1 }, Vector2D { x: 0, y: 1 }];
	pub const DIRECTIONS: [Vector2D; 4] = [
		Self::HORIZONTAL_DIRECTIONS[0],
		Self::HORIZONTAL_DIRECTIONS[1],
		Self::VERTICAL_DIRECTIONS[0],
		Self::VERTICAL_DIRECTIONS[1],
	];

	pub fn rotate_cw(self) -> Self {
		Self {
			x: self.y,
			y: -self.x,
		}
	}

	pub fn rotate_ccw(self) -> Self {
		Self {
			x: -self.y,
			y: self.x,
		}
	}

	pub fn manhattan_distance(self, other: Self) -> u64 {
		self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
	}
}

impl MulAssign<i64> for Vector2D {
	fn mul_assign(&mut self, rhs: i64) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl Mul<i64> for Vector2D {
	type Output = Vector2D;

	fn mul(mut self, rhs: i64) -> Self::Output {
		self *= rhs;
		self
	}
}

macro_rules! impl_try_into_tuple {
	($($ty:ty),+ $(,)?) => {
		$(
			impl TryFrom<Vector2D> for ($ty, $ty) {
				type Error = <$ty as TryFrom<i64>>::Error;

				fn try_from(Vector2D{ x, y }: Vector2D) -> Result<Self, Self::Error> {
					Ok((x.try_into()?, y.try_into()?))
				}
			}
		)*
	};
}

macro_rules! impl_try_from_tuple {
	($($ty:ty),+ $(,)?) => {
		$(
			impl TryFrom<($ty, $ty)> for Vector2D {
				type Error = <$ty as TryInto<i64>>::Error;

				fn try_from((x, y): ($ty, $ty)) -> Result<Self, Self::Error> {
					Ok(Vector2D {
						x: x.try_into()?,
						y: y.try_into()?,
					})
				}
			}
		)*
	};
}

impl_try_from_tuple! { u8, u16, u32, u64, usize, isize }
impl_try_into_tuple! { u8, i8, u16, i16, u32, i32, u64, isize, usize }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rotate_cw() {
		let mut vector = Vector2D { x: 1, y: 0 };

		vector = vector.rotate_cw();
		assert_eq!(vector, Vector2D { x: 0, y: -1 });

		vector = vector.rotate_cw();
		assert_eq!(vector, Vector2D { x: -1, y: 0 });

		vector = vector.rotate_cw();
		assert_eq!(vector, Vector2D { x: 0, y: 1 });

		vector = vector.rotate_cw();
		assert_eq!(vector, Vector2D { x: 1, y: 0 });
	}

	#[test]
	fn test_rotate_ccw() {
		let mut vector = Vector2D { x: 1, y: 0 };

		vector = vector.rotate_ccw();
		assert_eq!(vector, Vector2D { x: 0, y: 1 });

		vector = vector.rotate_ccw();
		assert_eq!(vector, Vector2D { x: -1, y: 0 });

		vector = vector.rotate_ccw();
		assert_eq!(vector, Vector2D { x: 0, y: -1 });

		vector = vector.rotate_ccw();
		assert_eq!(vector, Vector2D { x: 1, y: 0 });
	}
}
