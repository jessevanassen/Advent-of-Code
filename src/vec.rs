use std::{
	fmt::Display,
	ops::{Add, AddAssign, Neg, Sub},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct IVec2D(pub i32, pub i32);

impl IVec2D {
	pub const UP: IVec2D = IVec2D(0, 1);
	pub const RIGHT: IVec2D = IVec2D(1, 0);
	pub const DOWN: IVec2D = IVec2D(0, -1);
	pub const LEFT: IVec2D = IVec2D(-1, 0);

	/// # Examples
	/// ```
	/// # use aoc2022::vec::IVec2D;
	/// assert_eq!(IVec2D(0, 0).normalize(), IVec2D(0, 0));
	/// assert_eq!(IVec2D(5, 0).normalize(), IVec2D(1, 0));
	/// assert_eq!(IVec2D(0, -5).normalize(), IVec2D(0, -1));
	/// assert_eq!(IVec2D(-5, 5).normalize(), IVec2D(-1, 1));
	/// ```
	pub fn normalize(self) -> Self {
		IVec2D(self.0.signum(), self.1.signum())
	}

	pub fn direction(self, other: Self) -> Self {
		(other - self).normalize()
	}

	pub fn manhattan_distance(self, other: Self) -> u32 {
		(other.0 - self.0).unsigned_abs() + (other.1 - self.1).unsigned_abs()
	}

	/// Returns the minimum distance needed by a king to go from one square to
	/// another in chess.
	pub fn chessboard_distance(self, other: Self) -> u32 {
		let diff = other - self;
		Ord::max(diff.0.unsigned_abs(), diff.1.unsigned_abs())
	}

	/// # Examples
	/// ```
	/// # use aoc2022::vec::IVec2D;
	/// assert_eq!(IVec2D::UP.rotate_cw(),    IVec2D::RIGHT);
	/// assert_eq!(IVec2D::RIGHT.rotate_cw(), IVec2D::DOWN);
	/// assert_eq!(IVec2D::DOWN.rotate_cw(),  IVec2D::LEFT);
	/// assert_eq!(IVec2D::LEFT.rotate_cw(),  IVec2D::UP);
	/// ```
	pub fn rotate_cw(self) -> Self {
		Self(self.1, -self.0)
	}

	/// # Examples
	/// ```
	/// # use aoc2022::vec::IVec2D;
	/// assert_eq!(IVec2D::UP.rotate_ccw(),    IVec2D::LEFT);
	/// assert_eq!(IVec2D::RIGHT.rotate_ccw(), IVec2D::UP);
	/// assert_eq!(IVec2D::DOWN.rotate_ccw(),  IVec2D::RIGHT);
	/// assert_eq!(IVec2D::LEFT.rotate_ccw(),  IVec2D::DOWN);
	/// ```
	pub fn rotate_ccw(self) -> Self {
		Self(-self.1, self.0)
	}

	pub fn flip(self) -> Self {
		Self(-self.0, -self.1)
	}
}

impl Add for IVec2D {
	type Output = IVec2D;

	fn add(self, rhs: Self) -> Self::Output {
		IVec2D(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl AddAssign for IVec2D {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

impl AddAssign<IVec2D> for &mut IVec2D {
	fn add_assign(&mut self, rhs: IVec2D) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

impl Neg for IVec2D {
	type Output = IVec2D;

	fn neg(self) -> Self::Output {
		IVec2D(-self.0, -self.1)
	}
}

impl Sub for IVec2D {
	type Output = IVec2D;

	fn sub(self, rhs: Self) -> Self::Output {
		self + -rhs
	}
}

impl Display for IVec2D {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.0, self.1)
	}
}

impl From<(i32, i32)> for IVec2D {
	fn from((x, y): (i32, i32)) -> Self {
		Self(x, y)
	}
}

impl From<IVec2D> for (i32, i32) {
	fn from(value: IVec2D) -> Self {
		(value.0, value.1)
	}
}

impl TryFrom<(usize, usize)> for IVec2D {
	type Error = std::num::TryFromIntError;

	fn try_from((x, y): (usize, usize)) -> Result<Self, Self::Error> {
		Ok(Self(x.try_into()?, y.try_into()?))
	}
}

impl TryFrom<IVec2D> for (usize, usize) {
	type Error = std::num::TryFromIntError;

	fn try_from(IVec2D(x, y): IVec2D) -> Result<Self, Self::Error> {
		Ok((x.try_into()?, y.try_into()?))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod ivec2d {
		use super::*;

		mod chessboard_distance {
			use super::*;

			#[test]
			fn same_coordinates() {
				assert_eq!(0, IVec2D(10, 10).chessboard_distance(IVec2D(10, 10)),)
			}

			#[test]
			fn with_distance() {
				const MIDPOINT: IVec2D = IVec2D(0, 0);

				for distance in 1..=10 {
					for point in points_at_distance(distance) {
						assert_eq!(
							distance,
							point.chessboard_distance(MIDPOINT),
							"Distance between ({}, {}) and ({}, {}) should be {}",
							point.0,
							point.1,
							MIDPOINT.0,
							MIDPOINT.1,
							distance,
						);
					}
				}
			}

			fn points_at_distance(distance: u32) -> impl Iterator<Item = IVec2D> {
				let distance = distance as i32;
				let horizontal = (-distance..=distance)
					.flat_map(move |x| [IVec2D(x, -distance), IVec2D(x, distance)]);
				let vertical = (-(distance - 1)..=(distance - 1))
					.flat_map(move |y| [IVec2D(-distance, y), IVec2D(distance, y)]);
				horizontal.chain(vertical)
			}
		}
	}
}
