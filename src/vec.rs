use std::{
	fmt::Display,
	ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IVec2D(pub i32, pub i32);

impl Display for IVec2D {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.0, self.1)
	}
}

impl Neg for IVec2D {
	type Output = Self;

	/// # Examples
	///
	/// ```rust
	/// # use aoc2018::vec::IVec2D;
	/// assert_eq!(IVec2D(-10, -10), -IVec2D(10, 10));
	/// assert_eq!(IVec2D(10, 10), -IVec2D(-10, -10));
	/// assert_eq!(IVec2D(10, -10), -IVec2D(-10, 10));
	/// ```
	fn neg(self) -> Self::Output {
		Self(-self.0, -self.1)
	}
}

impl Add for IVec2D {
	type Output = Self;

	/// # Examples
	///
	/// ```rust
	/// # use aoc2018::vec::IVec2D;
	/// let lhs = IVec2D(10, 20);
	/// let rhs = IVec2D(30, 40);
	/// assert_eq!(IVec2D(40, 60), lhs + rhs);
	/// ```
	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl AddAssign for IVec2D {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Sub for IVec2D {
	type Output = Self;

	/// # Examples
	///
	/// ```rust
	/// # use aoc2018::vec::IVec2D;
	/// let lhs = IVec2D(20, 30);
	/// let rhs = IVec2D(10, 40);
	/// assert_eq!(IVec2D(10, -10), lhs - rhs);
	/// ```
	fn sub(self, rhs: Self) -> Self::Output {
		self + -rhs
	}
}

impl SubAssign for IVec2D {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}
