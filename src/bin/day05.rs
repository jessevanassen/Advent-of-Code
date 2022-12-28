use std::io::stdin;

fn main() {
	let input = stdin()
		.lines()
		.flatten()
		.next()
		.unwrap()
		.into_bytes();

	println!("Part 1: {}", react(input.iter().copied()).len());

	let part2 = (b'A'..=b'Z')
		.map(|c| {
			let without_c = input
				.iter()
				.copied()
				.filter(|&x| x.to_ascii_uppercase() != c);
			react(without_c).len()
		})
		.min()
		.unwrap();
	println!("Part 2: {}", part2);
}

fn react(polymers: impl IntoIterator<Item = u8>) -> Vec<u8> {
	fn is_polymer(x: u8, y: u8) -> bool {
		(x.is_ascii_lowercase() ^ y.is_ascii_lowercase())
			&& x.to_ascii_lowercase() == y.to_ascii_lowercase()
	}

	let mut polymers: Vec<Option<u8>> = polymers.into_iter().map(Some).collect();

	let mut i = 0;
	let mut j = 1;

	while j < polymers.len() {
		if polymers[i].is_some() && is_polymer(polymers[i].unwrap(), polymers[j].unwrap()) {
			polymers[i] = None;
			polymers[j] = None;

			if i > 0 {
				i -= 1;
				j += 1;

				while i > 0 && polymers[i].is_none() {
					i -= 1;
				}

				if i > 0 && polymers[i].is_some() {
					continue;
				}
			}
		}

		i = j;
		j = i + 1;
	}

	polymers.into_iter().flatten().collect()
}
