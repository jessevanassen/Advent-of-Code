use std::io::stdin;

fn main() {
	let lines = stdin().lines().flatten();
	let input = lines
		.map(|l| {
			l.split_whitespace()
				.map(|n| n.parse::<i64>().unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	let part1 = input.iter().map(solve).sum::<i64>();
	let part2 = input
		.iter()
		.map(|line| solve(line.iter().rev()))
		.sum::<i64>();
	dbg!(part1, part2);
}

fn solve<'a>(numbers: impl IntoIterator<Item = &'a i64>) -> i64 {
	fn solve(mut numbers: Vec<i64>) -> i64 {
		/// Calculate the differences between `numbers[i] and `numbers[i + 1]`, and stores it in
		/// place of numbers[i].
		/// Because the differences have one less item than the source they are based on, the last
		/// item in `numbers` will be left as-is.
		fn calculate_differences(numbers: &mut [i64]) {
			for i in 0..(numbers.len() - 1) {
				numbers[i] = numbers[i + 1] - numbers[i];
			}
		}

		/* This algorithm takes the numbers, and stores the differences in-place in the same Vec.
		 * It keeps recursing on the differences, until the differences all become 0.
		 * Because the differences have always one length less than the the numbers they are based
		 * on, the last number of the differences is preserved.
		 *
		 * 1   3   6  10  15   21 |
		 * 2   3   4   5   6 | 21
		 * 1   1   1   1 | 6   21
		 * 0   0   0 | 1   6   21
		 *
		 * When the iteration stops, the beginning of the Vec only contains `0` (the differences
		 * compared to the previous iteration), and the ending numbers of all the prior iterations.
		 * The solution can now be calculated by summing all numbers together. */
		for end in (0..=numbers.len()).rev() {
			let slice = &mut numbers[..(end - 1)];

			calculate_differences(slice);

			if slice.iter().all(|x| *x == 0) {
				break;
			}
		}

		numbers.iter().sum()
	}

	solve(numbers.into_iter().copied().collect())
}
