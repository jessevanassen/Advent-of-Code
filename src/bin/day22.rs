use std::{io::stdin, iter};

use itertools::Itertools;

fn main() {
	let input = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| line.parse().unwrap())
		.collect::<Vec<u64>>();

	let part1 = input
		.iter()
		.map(|n| secret_numbers(*n).nth(2000).unwrap())
		.sum::<u64>();
	println!("Part 1: {part1}");

	let part2 = {
		/// Every number within the pattern is in the -9..=9 range, which means
		/// the max value is 19**4. This is small enough to be represented as
		/// an array index.
		fn index(v: [i8; 4]) -> usize {
			(v[0] as usize + 9)
				+ (v[1] as usize + 9) * 19
				+ (v[2] as usize + 9) * 19 * 19
				+ (v[3] as usize + 9) * 19 * 19 * 19
		}

		let len = index([9, 9, 9, 9]) + 1;
		let mut total_bananas_for_pattern = vec![0; len];

		for &magic_number in input.iter() {
			let mut seen = vec![false; len];

			for (pattern, price) in price_change_patterns(magic_number) {
				let index = index(pattern);

				if !seen[index] {
					seen[index] = true;
					total_bananas_for_pattern[index] += price as u64;
				}
			}
		}

		total_bananas_for_pattern.into_iter().max().unwrap()
	};
	println!("Part 2: {part2}");
}

fn next_secret_number(n: u64) -> u64 {
	#[inline]
	fn mix(secret_number: u64, n: u64) -> u64 {
		secret_number ^ n
	}

	#[inline]
	fn prune(secret_number: u64) -> u64 {
		secret_number % 16777216
	}

	let n = prune(mix(n, n * 64));
	let n = prune(mix(n, n / 32));
	prune(mix(n, n * 2048))
}

fn secret_numbers(initial: u64) -> impl Iterator<Item = u64> {
	iter::successors(Some(initial), |n| Some(next_secret_number(*n)))
}

fn price_history(initial: u64) -> impl Iterator<Item = (u8, i8)> {
	secret_numbers(initial).scan((initial % 10) as u8, |state, n| {
		let n = (n % 10) as u8;
		let diff = n as i8 - (*state) as i8;

		*state = n;
		Some((n, diff))
	})
}

fn price_change_patterns(initial: u64) -> impl Iterator<Item = ([i8; 4], u8)> {
	price_history(initial)
		.skip(1) // First one is not a price change, but the initial price
		.take(2000) // At most 2000 changes are generated
		.tuple_windows()
		.map(|((_, d1), (_, d2), (_, d3), (price, d4))| ([d1, d2, d3, d4], price))
}

#[cfg(test)]
mod tests {
	use std::iter;

	use super::*;

	#[test]
	fn test_next_secret_number() {
		assert_eq!(
			iter::successors(Some(123), |&n| Some(next_secret_number(n)))
				.skip(1)
				.take(10)
				.collect::<Vec<_>>(),
			vec![
				15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
				5908254,
			]
		);
	}

	#[test]
	fn test_price_history() {
		assert_eq!(
			price_history(123).take(10).collect::<Vec<_>>(),
			vec![
				(3, 0),
				(0, -3),
				(6, 6),
				(5, -1),
				(4, -1),
				(4, 0),
				(6, 2),
				(4, -2),
				(4, 0),
				(2, -2)
			]
		);
	}

	#[test]
	fn test_price_change_patterns() {
		assert_eq!(
			price_change_patterns(123).take(3).collect::<Vec<_>>(),
			vec![
				([-3, 6, -1, -1], 4),
				([6, -1, -1, 0], 4),
				([-1, -1, 0, 2], 6),
			]
		)
	}
}
