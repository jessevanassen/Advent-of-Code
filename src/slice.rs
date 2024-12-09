pub trait SliceExt {
	/// Swaps two ranges in the slice.
	fn swap_chunks(self, a: usize, b: usize, len: usize);
}

impl<T> SliceExt for &mut [T] {
	fn swap_chunks(self, a: usize, b: usize, len: usize) {
		for (i, j) in (a..a + len).zip(b..b + len) {
			self.swap(i, j);
		}
	}
}
