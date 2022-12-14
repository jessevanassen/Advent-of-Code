mod bit_set;
pub use bit_set::BitSet;

mod byte_set;
pub use byte_set::ByteSet;

pub mod extensions;

mod grid2d;
pub use grid2d::Grid2D;

pub mod macros;

pub mod range_utils;

pub mod vec;

pub fn min_max<T: Ord + Copy>(x: T, y: T) -> (T, T) {
	(Ord::min(x, y), Ord::max(x, y))
}
