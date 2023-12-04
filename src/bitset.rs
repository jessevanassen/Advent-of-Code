/// BUCKET_COUNT can be determined with ceil(max_value / 8).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BitSet<const BUCKET_COUNT: usize>([u8; BUCKET_COUNT]);

impl<const BUCKET_COUNT: usize> BitSet<BUCKET_COUNT> {
	pub fn new() -> Self {
		Self::default()
	}

	/// ```rust
	/// # use aoc2023::BitSet;
	/// let mut bitset = BitSet::<1>::new();
	///
	/// assert!(!bitset.has(5));
	///
	/// assert!(bitset.insert(5));
	/// assert!(bitset.has(5));
	///
	/// // Set already contains '5'
	/// assert!(!bitset.insert(5));
	/// ```
	pub fn insert(&mut self, item: usize) -> bool {
		let has_value = self.has(item);
		*self.get_bucket_mut(item) |= get_bucket_value(item);
		!has_value
	}

	/// ```rust
	/// # use aoc2023::BitSet;
	/// let mut bitset = BitSet::<1>::new();
	///
	/// bitset.insert(5);
	///
	/// assert!(bitset.remove(5));
	/// assert!(!bitset.has(5));
	///
	/// // Set doesn't contain '5'
	/// assert!(!bitset.remove(5));
	/// ```
	pub fn remove(&mut self, item: usize) -> bool {
		let has_value = self.has(item);
		*self.get_bucket_mut(item) &= !get_bucket_value(item);
		has_value
	}

	/// ```rust
	/// # use aoc2023::BitSet;
	/// let mut bitset = BitSet::<1>::new();
	///
	/// assert!(!bitset.has(5));
	///
	/// bitset.insert(5);
	/// assert!(bitset.has(5));
	/// ```
	pub fn has(&self, item: usize) -> bool {
		(self.get_bucket(item) & get_bucket_value(item)) > 0
	}

	/// ```rust
	/// # use aoc2023::BitSet;
	/// let mut bitset = BitSet::<1>::new();
	///
	/// assert_eq!(bitset.len(), 0);
	///
	/// bitset.insert(1);
	/// bitset.insert(2);
	/// bitset.insert(3);
	/// assert_eq!(bitset.len(), 3);
	/// ```
	pub fn len(&self) -> usize {
		self.0.iter().map(|b| b.count_ones() as usize).sum()
	}

	/// ```rust
	/// # use aoc2023::BitSet;
	/// let mut bitset = BitSet::<1>::new();
	///
	/// assert!(bitset.is_empty());
	///
	/// bitset.insert(1);
	/// assert!(!bitset.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// ```rust
	/// # use aoc2023::BitSet;
	///
	/// let fst: BitSet<1> = [0b00001111].into();
	/// let snd: BitSet<1> = [0b00111100].into();
	/// let trd: BitSet<1> = [0b00111111].into();
	///
	/// assert_eq!(fst.union(snd), trd);
	/// ```
	pub fn union(&self, other: BitSet<BUCKET_COUNT>) -> Self {
		let mut copy = self.0;

		for (i, it) in copy.iter_mut().enumerate() {
			*it |= other.0[i];
		}

		copy.into()
	}

	/// ```rust
	/// # use aoc2023::BitSet;
	///
	/// let fst: BitSet<1> = [0b00001111].into();
	/// let snd: BitSet<1> = [0b00111100].into();
	/// let trd: BitSet<1> = [0b00001100].into();
	///
	/// assert_eq!(fst.intersection(snd), trd);
	/// ```
	pub fn intersection(&self, other: BitSet<BUCKET_COUNT>) -> Self {
		let mut copy = self.0;

		for (i, it) in copy.iter_mut().enumerate() {
			*it &= other.0[i];
		}

		copy.into()
	}

	pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
		(0..(8 * BUCKET_COUNT)).filter(|&it| self.has(it))
	}

	fn assert_no_overflow(item: usize) {
		if item > BUCKET_COUNT * 8 {
			panic!("Item {} exceeds BitSet capacity of {}", item, BUCKET_COUNT * 8);
		}
	}

	fn get_bucket(&self, item: usize) -> u8 {
		Self::assert_no_overflow(item);
		self.0[get_bucket_index(item)]
	}

	fn get_bucket_mut(&mut self, item: usize) -> &mut u8 {
		Self::assert_no_overflow(item);
		&mut self.0[get_bucket_index(item)]
	}
}

fn get_bucket_index(item: usize) -> usize {
	item >> 3
}

fn get_bucket_value(item: usize) -> u8 {
	1u8 << (item & 0b111)
}

impl<const BUCKET_COUNT: usize> Default for BitSet<BUCKET_COUNT> {
	fn default() -> Self {
		Self([0; BUCKET_COUNT])
	}
}

impl<const BUCKET_COUNT: usize> FromIterator<usize> for BitSet<BUCKET_COUNT> {
	fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
		let mut set = Self::default();

		for item in iter {
			set.insert(item);
		}

		set
	}
}

impl<const BUCKET_COUNT: usize> From<[u8; BUCKET_COUNT]> for BitSet<BUCKET_COUNT> {
	fn from(value: [u8; BUCKET_COUNT]) -> Self {
		BitSet(value)
	}
}

impl<const BUCKET_COUNT: usize> From<BitSet<BUCKET_COUNT>> for [u8; BUCKET_COUNT] {
	fn from(value: BitSet<BUCKET_COUNT>) -> Self {
		value.0
	}
}

impl<const BUCKET_COUNT: usize> core::fmt::Debug for BitSet<BUCKET_COUNT> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.iter()).finish()?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::BitSet;

	#[test]
	fn test_multiple_buckets() {
		let mut set = BitSet::<2>::new();

		for i in (0..16).step_by(2) {
			set.insert(i);
		}

		assert_eq!([0b01010101, 0b01010101], <[u8; 2]>::from(set));

		for i in (0..16).step_by(4) {
			set.remove(i);
		}

		assert_eq!([0b01000100, 0b01000100], <[u8; 2]>::from(set));
	}

	#[test]
	fn test_debug() {
		let mut set = BitSet::<1>::new();
		set.insert(3);
		set.insert(5);
		assert_eq!(format!("{set:?}"), "[3, 5]");
	}
}
