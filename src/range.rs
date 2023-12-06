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

pub fn contains<Idx>(outer: &Range<Idx>, inner: &Range<Idx>) -> bool
where
	Idx: PartialOrd<Idx>,
{
	outer.start <= inner.start && inner.end <= outer.end
}

/// ```rust
/// # use aoc2023::range::intersection;
/// // No overlap
/// assert_eq!(None, intersection(&(10..20), &(30..40)));
/// assert_eq!(None, intersection(&(30..40), &(10..20)));
///
/// // Touching, but no overlap
/// assert_eq!(None, intersection(&(10..20), &(20..30)));
/// assert_eq!(None, intersection(&(20..30), &(10..20)));
///
/// // Equal
/// assert_eq!(Some(20..40), intersection(&(20..40), &(20..40)));
///
/// // Partial overlap, not touching
/// assert_eq!(Some(20..30), intersection(&(10..30), &(20..40)));
/// assert_eq!(Some(20..30), intersection(&(20..40), &(10..30)));
///
/// // Partial overlap, touching at the front
/// assert_eq!(Some(20..30), intersection(&(20..30), &(20..40)));
/// assert_eq!(Some(20..30), intersection(&(20..40), &(20..30)));
/// // Partial overlap, touching at the rear
/// assert_eq!(Some(30..40), intersection(&(20..40), &(30..40)));
/// assert_eq!(Some(30..40), intersection(&(30..40), &(20..40)));
///
/// // Containing, not touching
/// assert_eq!(Some(20..30), intersection(&(10..40), &(20..30)));
/// assert_eq!(Some(20..30), intersection(&(20..30), &(10..40)));
/// ```
pub fn intersection<Idx>(r1: &Range<Idx>, r2: &Range<Idx>) -> Option<Range<Idx>>
where
	Idx: PartialOrd<Idx> + Ord + Copy,
{
	if !overlaps(r1, r2) {
		return None;
	}

	let start = r1.start.max(r2.start);
	let end = r1.end.min(r2.end);
	Some(start..end)
}
