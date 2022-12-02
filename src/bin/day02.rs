use std::io::stdin;

fn score(turn: &str) -> Result<u32, String> {
	Ok(match turn {
		"B X" => 1, // Paper    - Rock     : Lose
		"C Y" => 2, // Scissors - Paper    : Lose
		"A Z" => 3, // Rock     - Scissors : Lose
		"A X" => 4, // Rock     - Rock     : Draw
		"B Y" => 5, // Paper    - Paper    : Draw
		"C Z" => 6, // Scissors - Scissors : Draw
		"C X" => 7, // Scissors - Rock     : Win
		"A Y" => 8, // Rock     - Paper    : Win
		"B Z" => 9, // Paper    - Scissors : Win
		_ => return Err(format!("Invalid pattern: '{turn}'")),
	})
}

fn matchfixing_score(turn: &str) -> Result<u32, String> {
	Ok(match turn {
		"B X" => 1, // Paper    - Rock     : Lose
		"C X" => 2, // Scissors - Paper    : Lose
		"A X" => 3, // Rock     - Scissors : Lose
		"A Y" => 4, // Rock     - Rock     : Draw
		"B Y" => 5, // Paper    - Paper    : Draw
		"C Y" => 6, // Scissors - Scissors : Draw
		"C Z" => 7, // Scissors - Rock     : Win
		"A Z" => 8, // Rock     - Paper    : Win
		"B Z" => 9, // Paper    - Scissors : Win
		_ => return Err(format!("Invalid pattern: '{turn}'")),
	})
}

fn main() {
	fn add(x: (u32, u32), y: (u32, u32)) -> (u32, u32) {
		(x.0 + y.0, x.1 + y.1)
	}

	let (part1, part2) = stdin()
		.lines()
		.flatten()
		.map(|ref turn| (score(turn).unwrap(), matchfixing_score(turn).unwrap()))
		.fold((0, 0), add);

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}
