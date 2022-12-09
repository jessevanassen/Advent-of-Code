#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BitSet([u128; 2]);

impl BitSet {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn contains(&self, value: u8) -> bool {
		let (i, bit) = BitSet::bit_value(value);
		(self.0[i] & bit) > 0
	}

	pub fn insert(&mut self, value: u8) -> bool {
		if self.contains(value) {
			return false;
		}

		let (i, bit) = BitSet::bit_value(value);
		self.0[i] |= bit;

		true
	}

	pub fn remove(&mut self, value: u8) -> bool {
		if !self.contains(value) {
			return false;
		}

		let (i, bit) = BitSet::bit_value(value);
		self.0[i] &= !bit;

		true
	}

	pub fn union(&self, other: &Self) -> BitSet {
		BitSet([self.0[0] | other.0[0], self.0[1] | other.0[1]])
	}

	pub fn intersection(&self, other: &Self) -> BitSet {
		BitSet([self.0[0] & other.0[0], self.0[1] & other.0[1]])
	}

	fn bit_value(value: u8) -> (usize, u128) {
		((value >> 7) as usize, 1 << (value & 0x7f))
	}

	pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
		(0..=0xFF).filter(|&b| self.contains(b))
	}

	pub fn len(&self) -> u8 {
		self.iter().count() as u8
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

impl FromIterator<u8> for BitSet {
	fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
		let mut set = BitSet::default();
		for value in iter {
			set.insert(value);
		}
		set
	}
}

impl<'a> FromIterator<&'a u8> for BitSet {
	fn from_iter<T: IntoIterator<Item = &'a u8>>(iter: T) -> Self {
		iter.into_iter().copied().collect()
	}
}

#[cfg(test)]
mod tests {
	use super::BitSet;

	#[test]
	fn test_insert_and_remove() {
		let mut set = BitSet::default();

		assert!(!set.contains(0));
		assert!(!set.contains(1));
		assert!(!set.contains(254));
		assert!(!set.contains(255));

		assert!(set.insert(0));
		assert!(!set.insert(0));
		assert!(set.insert(254));

		assert!(set.contains(0));
		assert!(!set.contains(1));
		assert!(set.contains(254));
		assert!(!set.contains(255));

		assert!(set.remove(0));
		assert!(!set.remove(1));
		assert!(set.remove(254));
		assert!(!set.remove(255));

		assert!(!set.contains(0));
		assert!(!set.contains(1));
		assert!(!set.contains(254));
		assert!(!set.contains(255));
	}

	#[test]
	fn test_iter() {
		let mut set = BitSet::default();

		assert_eq!(0, set.len());

		set.insert(0);
		set.insert(255);
		set.insert(127);
		set.insert(128);

		assert_eq!(4, set.len());

		let mut iter = set.iter();
		assert_eq!(Some(0), iter.next());
		assert_eq!(Some(127), iter.next());
		assert_eq!(Some(128), iter.next());
		assert_eq!(Some(255), iter.next());
		assert_eq!(None, iter.next());
	}

	#[test]
	fn test_collect_to_bitset() {
		// From u8
		let input = vec![0, 2, 4, 8];
		let set: BitSet = input.iter().copied().collect();
		assert_eq!(input, set.iter().collect::<Vec<_>>());

		// From &u8
		let input = vec![0, 2, 4, 8];
		let set: BitSet = input.iter().collect();
		assert_eq!(input, set.iter().collect::<Vec<_>>());
	}
}
