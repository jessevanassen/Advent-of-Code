use std::ops::{RangeBounds, RangeInclusive};

pub fn contains<Idx: PartialOrd>(x: &impl RangeBounds<Idx>, y: &RangeInclusive<Idx>) -> bool {
	x.contains(y.start()) && x.contains(y.end())
}

pub fn overlaps<Idx: PartialOrd>(x: &RangeInclusive<Idx>, y: &RangeInclusive<Idx>) -> bool {
	!(x.start() > y.end() || y.start() > x.end())
}

#[cfg(test)]
mod tests {
	use super::*;

	mod contains {
		use super::contains;

		#[test]
		fn test_contains() {
			assert!(contains(&(10..=20), &(10..=20)), "Same range");
			assert!(contains(&(10..=20), &(10..=15)), "Same start");
			assert!(contains(&(10..=20), &(15..=20)), "Same end");
			assert!(contains(&(10..=20), &(12..=18)), "Fully contained");
		}

		#[test]
		fn test_does_not_contain() {
			assert!(!contains(&(10..=20), &(0..=15)), "Partial overlap at start");
			assert!(!contains(&(10..=20), &(15..=30)), "Partial overlap at end");
			assert!(!contains(&(10..=20), &(0..=5)), "No overlap, x before y");
			assert!(!contains(&(10..=20), &(25..=30)), "No overlap, y before x");
		}
	}

	mod overlaps {
		use super::overlaps;

		#[test]
		fn test_overlaps() {
			assert!(overlaps(&(10..=20), &(10..=20)), "Same range");
			assert!(overlaps(&(10..=20), &(10..=15)), "Same start");
			assert!(overlaps(&(10..=20), &(15..=20)), "Same end");
			assert!(overlaps(&(10..=20), &(12..=18)), "Fully contained");
			assert!(overlaps(&(10..=20), &(0..=15)), "Partial overlap at start");
			assert!(overlaps(&(10..=20), &(15..=30)), "Partial overlap at end");
		}

		#[test]
		fn test_does_not_overlap() {
			assert!(!overlaps(&(10..=20), &(0..=5)), "No overlap, x before y");
			assert!(!overlaps(&(10..=20), &(25..=30)), "No overlap, y before x");
		}
	}
}
