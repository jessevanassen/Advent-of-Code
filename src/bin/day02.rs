use std::io::stdin;

fn main() {
	let input = stdin().lines().next().unwrap().unwrap();
	let ranges = input
		.split(',')
		.map(|range| {
			let (min, max) = range.split_once('-').expect("Expect '-' delimiter");
			let min: u64 = min.parse().unwrap();
			let max: u64 = max.parse().unwrap();
			min..=max
		})
		.collect::<Vec<_>>();

	let values = ranges.iter().cloned().flatten().collect::<Vec<_>>();

	let mut part1 = 0;
	let mut part2 = 0;

	for id in values {
		let str = id.to_string();

		if str.len().is_multiple_of(2) && !valid_id(&str, str.len() / 2) {
			part1 += id;
			part2 += id;
			continue;
		}

		if (1..=(str.len() / 2)).any(|size| !valid_id(&str, size)) {
			part2 += id;
			continue;
		}
	}

	dbg!(part1, part2);
}

fn valid_id(id: &str, pattern_size: usize) -> bool {
	if !id.len().is_multiple_of(pattern_size) {
		return true;
	}

	let mut chunks = id.as_bytes().chunks_exact(pattern_size);

	let first = chunks.next().unwrap();
	for chunk in chunks {
		if first != chunk {
			return true;
		}
	}

	false
}
