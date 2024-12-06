#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
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
