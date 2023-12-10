#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2D(pub isize, pub isize);

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

impl From<(usize, usize)> for Vector2D {
	fn from((x, y): (usize, usize)) -> Self {
		Self(x as isize, y as isize)
	}
}
