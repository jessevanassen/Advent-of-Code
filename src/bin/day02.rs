use std::{io::stdin, str::FromStr};

type Score = u8;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Outcome {
	Win,
	Lose,
	Draw,
}

impl Outcome {
	fn score(&self) -> Score {
		match self {
			Outcome::Win => 6,
			Outcome::Draw => 3,
			Outcome::Lose => 0,
		}
	}
}

impl FromStr for Outcome {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Outcome::*;
		Ok(match s {
			"X" => Lose,
			"Y" => Draw,
			"Z" => Win,
			_ => return Err(()),
		})
	}
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Move {
	Rock,
	Paper,
	Scissors,
}

impl Move {
	fn score(&self) -> Score {
		match self {
			Move::Rock => 1,
			Move::Paper => 2,
			Move::Scissors => 3,
		}
	}

	fn play_against(&self, other: Move) -> Outcome {
		use self::Outcome::*;
		match other {
			_ if other == self.wins_against() => Win,
			_ if other == self.loses_against() => Lose,
			_ => Draw,
		}
	}

	fn wins_against(&self) -> Move {
		use Move::*;
		match self {
			Rock => Scissors,
			Paper => Rock,
			Scissors => Paper,
		}
	}

	fn loses_against(&self) -> Move {
		use Move::*;
		match self {
			Rock => Paper,
			Paper => Scissors,
			Scissors => Rock,
		}
	}
}

impl FromStr for Move {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"A" | "X" => Move::Rock,
			"B" | "Y" => Move::Paper,
			"C" | "Z" => Move::Scissors,
			_ => return Err(()),
		})
	}
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Turn {
	their_move: Move,
	our_move: Move,
	desired_outcome: Outcome,
}

impl FromStr for Turn {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (x, y) = s.split_once(' ').ok_or(())?;
		Ok(Turn {
			their_move: x.parse()?,
			our_move: y.parse()?,
			desired_outcome: y.parse()?,
		})
	}
}

impl Turn {
	fn play(&self, strategy: Strategy) -> Score {
		strategy.play(self)
	}
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Strategy {
	PlayTurn,
	ForceDesiredOutcome,
}

impl Strategy {
	fn play(&self, turn: &Turn) -> Score {
		use self::{Outcome::*, Strategy::*};
		match self {
			PlayTurn => turn.our_move.play_against(turn.their_move).score() + turn.our_move.score(),
			ForceDesiredOutcome => {
				let our_move = match turn.desired_outcome {
					Win => turn.their_move.loses_against(),
					Lose => turn.their_move.wins_against(),
					Draw => turn.their_move,
				};
				our_move.score() + turn.desired_outcome.score()
			}
		}
	}
}

fn main() {
	let (part1, part2) = stdin()
		.lines()
		.flatten()
		.map(|line| line.parse::<Turn>().unwrap())
		.map(|turn| {
			(
				turn.play(Strategy::PlayTurn),
				turn.play(Strategy::ForceDesiredOutcome),
			)
		})
		.fold((0u32, 0u32), |(x1, y1), (x2, y2)| {
			(x1 + x2 as u32, y1 + y2 as u32)
		});
	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let turns: Vec<Turn> = vec!["A Y", "B X", "C Z"]
			.into_iter()
			.map(|x| x.parse().unwrap())
			.collect();

		assert_eq!(
			vec![8, 1, 6],
			turns
				.iter()
				.map(|turn| turn.play(Strategy::PlayTurn))
				.collect::<Vec<_>>(),
		);

		assert_eq!(
			vec![4, 1, 7],
			turns
				.iter()
				.map(|turn| turn.play(Strategy::ForceDesiredOutcome))
				.collect::<Vec<_>>(),
		);
	}
}
