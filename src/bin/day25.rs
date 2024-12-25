use std::io::stdin;

use itertools::Itertools;

fn main() {
	let Input { locks, keys } = parse_input();

	let part1 = locks
		.iter()
		.cartesian_product(keys.iter())
		.filter(|(lock, key)| {
			lock.iter()
				.zip(key.iter())
				.all(|(available, filled)| available >= filled)
		})
		.count();
	println!("Part 1: {part1}");
}

type Combination = [u8; 5];
struct Input {
	/// Locks contain a combination which represents the available spaces
	locks: Vec<Combination>,
	/// Keys contain a combination which represents the filled spaces
	keys: Vec<Combination>,
}
fn parse_input() -> Input {
	let mut locks = Vec::new();
	let mut keys = Vec::new();

	let chunks = stdin().lines().map(Result::unwrap).chunks(8);

	for mut chunk in &chunks {
		let is_lock = chunk.next().unwrap().as_bytes()[0] == b'#';

		let mut combination = [0u8; 5];
		for row in chunk.take(5) {
			for (i, b) in row.bytes().enumerate() {
				if is_lock == (b == b'.') {
					combination[i] += 1;
				}
			}
		}

		let collection = if is_lock { &mut locks } else { &mut keys };
		collection.push(combination);
	}

	Input { locks, keys }
}
