use std::ops::Range;

/// ```rust
/// # use aoc2023::range::overlaps;
/// // Separate
/// assert!(!overlaps(&(0..10), &(20..30)));
/// assert!(!overlaps(&(20..30), &(0..10)));
///
/// // Touching, but not overlapping
/// assert!(!overlaps(&(0..10), &(10..20)));
/// assert!(!overlaps(&(10..20), &(0..10)));
///
/// // Equal
/// assert!(overlaps(&(0..10), &(0..10)));
///
/// // Partial overlap
/// assert!(overlaps(&(0..10), &(9..19)));
/// assert!(overlaps(&(9..19), &(0..10)));
///
/// // Touching on a side
/// assert!(overlaps(&(0..20), &(10..20)));
/// assert!(overlaps(&(0..20), &(0..10)));
/// assert!(overlaps(&(10..20), &(0..20)));
/// assert!(overlaps(&(0..10), &(0..20)));
///
/// // Containing
/// assert!(overlaps(&(0..30), &(10..20)));
/// assert!(overlaps(&(10..20), &(0..30)));
/// ```
pub fn overlaps<Idx>(r1: &Range<Idx>, r2: &Range<Idx>) -> bool
where
	Idx: PartialOrd<Idx>,
{
	!(r1.end <= r2.start || r2.end <= r1.start)
}

pub fn contains<Idx>(outer: Range<Idx>, inner: &Range<Idx>) -> bool
where
	Idx: PartialOrd<Idx>,
{
	outer.start <= inner.start && inner.end <= outer.end
}
