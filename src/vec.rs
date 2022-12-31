use std::{
	fmt::Display,
	ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IVec2D(pub i32, pub i32);

impl IVec2D {
	pub fn min_max(iter: impl IntoIterator<Item = IVec2D>) -> Option<(IVec2D, IVec2D)> {
		iter.into_iter()
			.fold(None, |acc, it| match acc {
				None => Some((it, it)),
				Some((min, max)) => Some((
					IVec2D(min.0.min(it.0), min.1.min(it.1)),
					IVec2D(max.0.max(it.0), max.1.max(it.1)),
				)),
			})
	}
}

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

impl Mul<i32> for IVec2D {
	type Output = Self;

	fn mul(self, rhs: i32) -> Self::Output {
		IVec2D(self.0 * rhs, self.1 * rhs)
	}
}

#[cfg(test)]
mod tests {
	use anyhow::Context;

	use super::*;

	#[test]
	fn test_min_max() -> anyhow::Result<()> {
		let (min, max) = IVec2D::min_max([IVec2D(10, 10), IVec2D(5, 20), IVec2D(15, 0)])
			.context("Expect a result")?;

		assert_eq!(IVec2D(5, 0), min);
		assert_eq!(IVec2D(15, 20), max);
		Ok(())
	}
}
