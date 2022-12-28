use itertools::Itertools;
use std::io::stdin;

fn main() {
	let input: Vec<String> = stdin().lines().flatten().collect();

	let part1 = {
		let (twos, threes) = input
			.iter()
			.map(|line| {
				let mut occurrences = [0u8; 26];
				for b in line.bytes() {
					let index = (b - b'a') as usize;
					occurrences[index] += 1;
				}

				(
					occurrences
						.iter()
						.any(|count| *count == 2),
					occurrences
						.iter()
						.any(|count| *count == 3),
				)
			})
			.fold((0, 0), |acc, it| (acc.0 + it.0 as u32, acc.1 + it.1 as u32));
		twos * threes
	};
	println!("Part 1: {part1}");

	let part2 = input
		.iter()
		.combinations(2)
		.find_map(|combination| {
			let Ok((common_char, _)) = combination[0].bytes()
					.zip(combination[1].bytes())
					.filter(|(x, y)| x != y)
					.exactly_one() else {
						return None;
					};

			Some(
				combination[0]
					.bytes()
					.filter(|&c| c != common_char)
					.map(|b| b as char)
					.collect::<String>(),
			)
		})
		.expect("Expected a solution");
	println!("Part 2: {part2}");
}
