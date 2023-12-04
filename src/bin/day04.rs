use std::io::stdin;

fn main() {
	let lines = stdin().lines().flatten();
	let scratchcard_scores = lines
		.map(|line| {
			let numbers = line.split_once(": ").unwrap().1;
			let (winning_numbers, my_numbers) = numbers.split_once(" | ").unwrap();

			let collect_to_bitset = |xs: &str| {
				xs.split_whitespace()
					.map(|n| n.parse::<u8>().unwrap())
					.fold(0u128, |acc, it| acc | 1 << it)
			};

			let winning_numbers = collect_to_bitset(winning_numbers);
			let my_numbers = collect_to_bitset(my_numbers);

			(winning_numbers & my_numbers).count_ones() as usize
		})
		.collect::<Box<_>>();

	println!(
		"Part 1: {}",
		scratchcard_scores
			.iter()
			.filter(|&&score| score > 0)
			.map(|&score| 1 << (score - 1))
			.sum::<usize>()
	);

	let mut scratchcard_amounts = vec![1; scratchcard_scores.len()];

	for i in 0..scratchcard_amounts.len() {
		/* I'm ignoring that these indices might exceed the scratchcards range, as that would feel
		 * like a bug in the input. */
		let next_card_indices = (i + 1)..(i + 1 + scratchcard_scores[i]);

		for j in next_card_indices {
			scratchcard_amounts[j] += scratchcard_amounts[i];
		}
	}

	println!("Part 2: {}", scratchcard_amounts.iter().sum::<usize>());
}
