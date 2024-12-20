use std::{
	fmt::Display,
	ops::{Mul, MulAssign},
};

#[derive(
	Debug,
	Default,
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

	pub const fn new(x: i64, y: i64) -> Self {
		Self { x, y }
	}

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

	pub fn manhattan_distances(self, distance: u64) -> impl Iterator<Item = Vector2D> {
		let Vector2D { x, y } = self;
		let distance = distance as i64;

		let top_right = (x..(x + distance)).zip((y - distance)..y);
		let bottom_right = ((x + 1)..=(x + distance)).rev().zip((y)..(y + distance));
		let bottom_left = ((x - distance + 1)..=x)
			.rev()
			.zip(((y + 1)..=(y + distance)).rev());
		let top_left = ((x - distance)..x).zip(((y - distance + 1)..=y).rev());

		let combined = top_right
			.chain(bottom_right)
			.chain(bottom_left)
			.chain(top_left);

		combined.map(Vector2D::from)
	}
}

impl MulAssign for Vector2D {
	fn mul_assign(&mut self, rhs: Self) {
		self.x *= rhs.x;
		self.y *= rhs.y;
	}
}

impl Mul for Vector2D {
	type Output = Self;

	fn mul(mut self, rhs: Self) -> Self::Output {
		self *= rhs;
		self
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

impl Display for Vector2D {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

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

	#[test]
	fn test_manhattan_distances() {
		assert_eq!(
			Vector2D::new(10, 10)
				.manhattan_distances(3)
				.collect::<Vec<_>>(),
			vec![
				Vector2D { x: 10, y: 7 },
				Vector2D { x: 11, y: 8 },
				Vector2D { x: 12, y: 9 },
				Vector2D { x: 13, y: 10 },
				Vector2D { x: 12, y: 11 },
				Vector2D { x: 11, y: 12 },
				Vector2D { x: 10, y: 13 },
				Vector2D { x: 9, y: 12 },
				Vector2D { x: 8, y: 11 },
				Vector2D { x: 7, y: 10 },
				Vector2D { x: 8, y: 9 },
				Vector2D { x: 9, y: 8 }
			]
		);
	}
}
