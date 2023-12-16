use std::num::TryFromIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2D(pub isize, pub isize);

impl Vector2D {
	pub fn rotate_cw(self) -> Vector2D {
		assert!(self.0 == 0 || self.1 == 0);

		Vector2D(self.1, -self.0)
	}

	pub fn rotate_ccw(self) -> Vector2D {
		assert!(self.0 == 0 || self.1 == 0);

		Vector2D(-self.1, self.0)
	}
}

impl std::ops::Add for Vector2D {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl std::ops::Neg for Vector2D {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self(-self.0, -self.1)
	}
}

impl From<(isize, isize)> for Vector2D {
	fn from((x, y): (isize, isize)) -> Self {
		Self(x, y)
	}
}

impl From<(usize, usize)> for Vector2D {
	fn from((x, y): (usize, usize)) -> Self {
		Self(x as isize, y as isize)
	}
}

impl From<Vector2D> for (isize, isize) {
	fn from(Vector2D(x, y): Vector2D) -> Self {
		(x, y)
	}
}

impl TryFrom<Vector2D> for (usize, usize) {
	type Error = TryFromIntError;

	fn try_from(Vector2D(x, y): Vector2D) -> Result<Self, Self::Error> {
		let x = x.try_into()?;
		let y = y.try_into()?;
		Ok((x, y))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const UP: Vector2D = Vector2D(0, 1);
	const RIGHT: Vector2D = Vector2D(1, 0);
	const DOWN: Vector2D = Vector2D(0, -1);
	const LEFT: Vector2D = Vector2D(-1, 0);

	#[test]
	fn test_rotate_cw() {
		assert_eq!(RIGHT, UP.rotate_cw());
		assert_eq!(DOWN, RIGHT.rotate_cw());
		assert_eq!(LEFT, DOWN.rotate_cw());
		assert_eq!(UP, LEFT.rotate_cw());
	}

	#[test]
	fn test_rotate_ccw() {
		assert_eq!(LEFT, UP.rotate_ccw());
		assert_eq!(UP, RIGHT.rotate_ccw());
		assert_eq!(RIGHT, DOWN.rotate_ccw());
		assert_eq!(DOWN, LEFT.rotate_ccw());
	}
}
