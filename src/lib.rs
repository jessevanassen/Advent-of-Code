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

/// Returns a slice of the last n elements, or `None` if the slice doesn't have
/// enough elements.
///
/// # Examples
/// ```rust
/// # use aoc2022::last_n;
/// let values = [0, 1, 2, 3].as_slice();
/// assert_eq!(last_n(2, values), Some([2, 3].as_slice()));
/// assert_eq!(last_n(5, values), None);
/// ```
pub fn last_n<T>(n: usize, values: &[T]) -> Option<&[T]> {
	(n <= values.len()).then(|| &values[(values.len() - n)..])
}


/// # Examples
/// ```rust
/// # use aoc2022::triangle_number;
/// assert_eq!(triangle_number(0), 0);
/// assert_eq!(triangle_number(1), 1);
/// assert_eq!(triangle_number(2), 1 + 2);
/// assert_eq!(triangle_number(3), 1 + 2 + 3);
/// ```
pub fn triangle_number(n: usize) -> usize {
	n * (n + 1) / 2
}
