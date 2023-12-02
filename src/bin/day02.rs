use std::io::stdin;

fn main() {
	let lines = stdin().lines().flatten();
	let minimum_cubes_per_game: Vec<[usize; 3]> = lines
		.map(|line| {
			let games = line.split_once(": ").unwrap().1;
			let draws = games.split("; ").flat_map(|draws| draws.split(", "));
			let cubes = draws.map(|draw| {
				let (amount, color) = draw.split_once(' ').unwrap();
				let amount: usize = amount.parse().unwrap();
				let color: usize = parse_color(color).unwrap();
				(color, amount)
			});
			cubes.fold([0; 3], |mut acc, (color, amount)| {
				acc[color] = acc[color].max(amount);
				acc
			})
		})
		.collect();

	println!(
		"Part 1: {}",
		(1..) // Game indices are 1-based, range+zip is easier than .enumerate+applying an offset
			.zip(&minimum_cubes_per_game)
			.filter(|(_, &[r, g, b])| r <= 12 && g <= 13 && b <= 14)
			.map(|(game_index, _)| game_index)
			.sum::<usize>()
	);

	println!(
		"Part 2: {}",
		minimum_cubes_per_game
			.iter()
			.map(|cubes| cubes.iter().product::<usize>())
			.sum::<usize>()
	);
}

fn parse_color(color: &str) -> Result<usize, String> {
	Ok(match color {
		"red" => 0,
		"green" => 1,
		"blue" => 2,

		_ => return Err(format!("Invalid color '{color}'")),
	})
}
