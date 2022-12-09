use std::io::stdin;

use aoc2022::BitSet;

fn main() {
	let input = stdin().lines().next().unwrap().unwrap();
	let data = input
		.as_bytes()
		.iter()
		.map(|b| {
			if !b.is_ascii_lowercase() {
				panic!("Expected all input characters to be lowercase ASCII");
			}
			b - b'a'
		})
		.collect::<Vec<_>>();

	println!("Part 1: {}", first_marker(4, &data).unwrap());
	println!("Part 2: {}", first_marker(14, &data).unwrap());
}

fn first_marker(checksum_size: usize, data: &[u8]) -> Option<usize> {
	data.windows(checksum_size)
		.enumerate()
		.find(|(_, window)| all_unique(*window))
		.map(|(index, _)| index + checksum_size)
		.filter(|&index| index < data.len())
}

fn all_unique<'a>(input: impl IntoIterator<Item = &'a u8>) -> bool {
	let mut bitset = BitSet::new();
	for &item in input {
		if !bitset.insert(item) {
			return false;
		}
	}
	true
}

#[cfg(test)]
mod tests {
	use super::first_marker;

	#[test]
	fn test_first_marker() {
		assert_eq!(Some(4), first_marker(4, &[1, 2, 3, 4, 5]));
		assert_eq!(Some(7), first_marker(4, &[1, 1, 1, 1, 2, 3, 4, 5]));

		assert_eq!(None, first_marker(4, &[1, 2]));
		assert_eq!(None, first_marker(4, &[1, 2, 3, 4]));
		assert_eq!(None, first_marker(4, &[1, 1, 1, 2, 3, 4]));
		assert_eq!(None, first_marker(4, &[1, 2, 1, 2, 1, 2, 1, 2]));
	}
}
