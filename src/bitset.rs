use std::fmt::Debug;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct BitSet(Vec<u8>);

impl BitSet {
	pub fn with_size(capacity: usize) -> Self {
		Self(vec![0; capacity.div_ceil(8)])
	}

	pub fn len(&self) -> usize {
		self.0
			.iter()
			.map(|b| b.count_ones())
			.fold(0, |acc, x| acc + x as usize)
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn capacity(&self) -> usize {
		self.0.len() * 8
	}

	pub fn contains(&self, index: impl IntoIndex) -> bool {
		let (bucket, bit) = location(index.into_index());
		self.0
			.get(bucket)
			.map(|b| b & (1 << bit) != 0)
			.unwrap_or(false)
	}

	pub fn toggle(&mut self, index: impl IntoIndex, value: bool) -> bool {
		let index = index.into_index();

		if value == self.contains(index) {
			return false;
		}

		let (bucket, bit) = location(index.into_index());

		if bucket >= self.0.len() {
			self.0.resize(bucket + 1, 0);
		}

		let byte = &mut self.0[bucket];

		if value {
			*byte |= 1 << bit;
		} else {
			*byte &= !(1 << bit);
		}

		true
	}

	pub fn insert(&mut self, index: impl IntoIndex) -> bool {
		self.toggle(index, true)
	}

	pub fn remove(&mut self, index: impl IntoIndex) -> bool {
		self.toggle(index, false)
	}

	pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
		(0..self.capacity()).filter(|&i| self.contains(i))
	}
}

/// Returns the bucket and the bit, respectively
fn location(index: usize) -> (usize, usize) {
	(index >> 3, index & 7)
}

impl<IntoIndex: self::IntoIndex> FromIterator<IntoIndex> for BitSet {
	fn from_iter<T: IntoIterator<Item = IntoIndex>>(iter: T) -> Self {
		let mut set = Self::default();
		set.extend(iter);
		set
	}
}

impl<IntoIndex: self::IntoIndex> Extend<IntoIndex> for BitSet {
	fn extend<T: IntoIterator<Item = IntoIndex>>(&mut self, iter: T) {
		for index in iter.into_iter() {
			self.insert(index);
		}
	}
}

impl Debug for BitSet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_set().entries(self.iter()).finish()
	}
}

pub trait IntoIndex {
	fn into_index(self) -> usize;
}

impl IntoIndex for usize {
	fn into_index(self) -> usize {
		self
	}
}

impl<I> IntoIndex for &I
where
	I: IntoIndex + Copy,
{
	fn into_index(self) -> usize {
		(*self).into_index()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut set = BitSet::default();

		assert!(!set.contains(0));

		assert!(set.insert(0));
		assert!(!set.insert(0));
		assert!(set.contains(0));

		assert!(set.remove(0));
		assert!(!set.remove(0));
		assert!(!set.contains(0));
	}
}
