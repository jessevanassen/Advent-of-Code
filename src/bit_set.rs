/// A set for storing values as their bit value. This is a more storage
/// efficient version of `Vec<bool>`, taking 8x less space in memory.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BitSet {
	data: Vec<u8>,
	capacity: usize,
}

impl BitSet {
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			data: vec![0; div_ceil(capacity, u8::BITS as usize)],
			capacity,
		}
	}

	fn within_capacity(&self, value: usize) -> bool {
		value <= self.capacity
	}


	pub fn contains(&self, value: usize) -> bool {
		if !self.within_capacity(value) {
			return false;
		}

		let (index, value) = bucketed(value);
		self.data[index] & (1 << value) != 0
	}

	pub fn insert(&mut self, value: usize) -> bool {
		if self.contains(value) {
			return false;
		}

		let (index, value) = bucketed(value);
		self.data[index] |= 1 << value;
		true
	}

	pub fn remove(&mut self, value: usize) -> bool {
		if !self.within_capacity(value) || !self.contains(value) {
			return false;
		}

		let (index, value) = bucketed(value);
		self.data[index] &= !(1 << value);

		true
	}

	pub fn len(&self) -> usize {
		self.data.iter().map(|b| b.count_ones() as usize).sum()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
		(0..=(self.capacity)).filter(|&v| self.contains(v))
	}

	pub fn union(&self, other: &Self) -> Self {
		let mut result = BitSet::with_capacity(Ord::max(self.capacity, other.capacity));

		for i in 0..Ord::max(self.data.len(), other.data.len()) {
			result.data[i] = self.data.get(i).unwrap_or(&0u8) | other.data.get(i).unwrap_or(&0u8);
		}

		result
	}

	pub fn intersection(&self, other: &Self) -> Self {
		let mut result = BitSet::with_capacity(Ord::max(self.capacity, other.capacity));

		for i in 0..Ord::max(self.data.len(), other.data.len()) {
			result.data[i] = self.data.get(i).unwrap_or(&0u8) & other.data.get(i).unwrap_or(&0u8);
		}

		result
	}
}

fn bucketed(value: usize) -> (usize, u8) {
	(value >> 3, (value & 0x7) as u8)
}

fn div_ceil(dividend: usize, divisor: usize) -> usize {
	let div = dividend / divisor;
	if dividend % divisor != 0 {
		div + 1
	} else {
		div
	}
}

#[cfg(test)]
mod tests {
	use super::BitSet;

	#[test]
	fn test_insert_and_remove() {
		let mut set = BitSet::with_capacity(0xff);

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
		let mut set = BitSet::with_capacity(0xff);

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
	fn test_union() {
		let mut sets = (BitSet::with_capacity(8), BitSet::with_capacity(9));
		assert_eq!(sets.0.data.capacity(), 1);
		assert_eq!(sets.1.data.capacity(), 2);

		assert!(sets.0.insert(6));
		assert!(sets.0.insert(7));
		assert!(sets.1.insert(7));
		assert!(sets.1.insert(8));

		let union = sets.0.union(&sets.1);

		assert_eq!(3, union.len());
		assert_eq!(2, union.data.capacity());

		let mut iter = union.iter();
		assert_eq!(Some(6), iter.next());
		assert_eq!(Some(7), iter.next());
		assert_eq!(Some(8), iter.next());
	}

	#[test]
	fn test_intersection() {
		let mut sets = (BitSet::with_capacity(8), BitSet::with_capacity(9));
		assert_eq!(sets.0.data.capacity(), 1);
		assert_eq!(sets.1.data.capacity(), 2);

		assert!(sets.0.insert(6));
		assert!(sets.0.insert(7));
		assert!(sets.1.insert(7));
		assert!(sets.1.insert(8));

		let intersection = sets.0.intersection(&sets.1);

		assert_eq!(1, intersection.len());
		assert_eq!(2, intersection.data.capacity());

		let mut iter = intersection.iter();
		assert_eq!(Some(7), iter.next());
	}
}
