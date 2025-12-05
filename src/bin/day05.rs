use std::{io::stdin, ops::Range};

fn main() {
	let mut lines = stdin().lines().map(Result::unwrap);
	let ranges: Vec<Range<u64>> = (&mut lines)
		.take_while(|line| !line.is_empty())
		.map(|line| {
			let (start, end) = line
				.split_once('-')
				.expect("Input error, range not separated by '-'");
			let start: u64 = start.parse().unwrap();
			let end: u64 = end.parse().unwrap();
			start..(end + 1)
		})
		.collect();
	let ranges = merge_overlapping_ranges(ranges);

	let ids: Box<[u64]> = lines.map(|line| line.parse().unwrap()).collect();

	println!(
		"Part 1: {}",
		ids.iter()
			.filter(|id| ranges.iter().any(|range| range.contains(id)))
			.count(),
	);

	println!(
		"Part 2: {}",
		ranges
			.iter()
			.map(|range| range.end - range.start)
			.sum::<u64>(),
	)
}

fn merge_overlapping_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
	if ranges.is_empty() {
		return Vec::new();
	}

	ranges.sort_by_key(|range| range.start);
	let mut ranges = ranges.into_iter();

	let mut result = Vec::with_capacity(ranges.len());
	result.push(ranges.next().expect("ranges is non empty"));

	for range in ranges {
		let previous = result.last_mut().unwrap();

		if range.start <= previous.end {
			/* `max` is used to prevent making the range smaller, in case the
			 * current range is a subrange of the previous. */
			previous.end = u64::max(previous.end, range.end);
		} else {
			result.push(range);
		}
	}

	result
}
