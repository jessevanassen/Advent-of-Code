use std::ops::Range;

/// Returns the lowest value for which the predicate returns `true`, or None if
/// no predicates return `true`.
///
/// It is expected that the results of the predicate are ordered, such that:
/// - For every argument less than the index, the predicate returns `false`
/// - For every argument greater than or equal to the index, the predicate
///   returns `true`
///
/// # Examples
/// ```rust
/// # use aoc2024::binary_search::find_leftmost;
/// assert_eq!(
///     find_leftmost(0..10, |v| v >= 0),
///     Some(0)
/// );
///
/// assert_eq!(
///     find_leftmost(0..10, |v| v >= 5),
///     Some(5)
/// );
///
/// assert_eq!(
///     find_leftmost(0..10, |v| v >= 9),
///     Some(9)
/// );
///
/// assert_eq!(
///     find_leftmost(0..10, |v| v >= 10),
///     None
/// );
/// ```
pub fn find_leftmost(
	mut range: Range<usize>,
	mut predicate: impl FnMut(usize) -> bool,
) -> Option<usize> {
	let end = range.end;

	while range.start < range.end {
		let midpoint = (range.start + range.end) / 2;
		if predicate(midpoint) {
			range = range.start..midpoint;
		} else {
			range = (midpoint + 1)..range.end;
		}
	}

	(range.start < end).then_some(range.start)
}
