use std::ops::{Add, AddAssign, Neg, Sub};

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
