use std::{io::stdin, ops::Add};

macro_rules! mapped_enum {
	($name:ident {
		$($pattern:pat => $case:ident = $value:literal),* $(,)?
	}) => {
		#[repr(i8)]
		#[derive(PartialEq, Eq, Clone, Copy, Debug)]
		enum $name {
			$($case = $value),*
		}

		impl TryFrom<i8> for $name {
			type Error = ();

			fn try_from(value: i8) -> Result<Self, Self::Error> {
				match value {
					$(_ if value == $value => Ok($name::$case) ,)*
					_ => Err(())
				}
			}
		}

		impl TryFrom<char> for $name {
			type Error = ();

			fn try_from(value: char) -> Result<Self, Self::Error> {
				match value {
					$($pattern => Ok($name::$case),)*
					_ => Err(())
				}
			}
		}
	};
}

mapped_enum!(Outcome {
	'X' => Lose = -1,
	'Y' => Draw = 0,
	'Z' => Win = 1,
});

impl Outcome {
	fn score(&self) -> u8 {
		(((*self as i8) + 1) * 3) as u8
	}
}

mapped_enum!(Move {
	'A'|'X' => Rock = 0,
	'B'|'Y' => Paper = 1,
	'C'|'Z' => Scissors = 2,
});

impl Move {
	fn score(&self) -> u8 {
		(*self as u8) + 1
	}
}

impl Add<Move> for Move {
	type Output = Outcome;

	fn add(self, rhs: Move) -> Self::Output {
		match (self as i8 - rhs as i8).rem_euclid(3) {
			0 => Outcome::Draw,
			1 => Outcome::Lose,
			2 => Outcome::Win,
			_ => unreachable!(),
		}
	}
}

impl Add<Outcome> for Move {
	type Output = Move;

	fn add(self, rhs: Outcome) -> Self::Output {
		(self as i8 + rhs as i8).rem_euclid(3).try_into().unwrap()
	}
}

fn main() {
	fn add(x: (u32, u32), y: (u8, u8)) -> (u32, u32) {
		(x.0 + y.0 as u32, x.1 + y.1 as u32)
	}

	let (part1, part2) = stdin()
		.lines()
		.flatten()
		.map(|ref turn| {
			fn score(theirs: Move, ours: Move) -> u8 {
				(theirs + ours).score() + ours.score()
			}

			let fst = turn.as_bytes()[0] as char;
			let snd = turn.as_bytes()[2] as char;

			let their_move: Move = fst.try_into().unwrap();
			let our_move: Move = snd.try_into().unwrap();
			let matchfixed_outcome: Outcome = snd.try_into().unwrap();
			let our_matchfixed_move = their_move + matchfixed_outcome;

			(
				score(their_move, our_move),
				score(their_move, our_matchfixed_move),
			)
		})
		.fold((0, 0), add);

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
	use super::{Move::*, Outcome::*};

	#[test]
	fn test_add_move_to_move() {
		assert_eq!(Rock + Paper, Win);
		assert_eq!(Paper + Scissors, Win);
		assert_eq!(Scissors + Rock, Win);

		assert_eq!(Rock + Rock, Draw);
		assert_eq!(Paper + Paper, Draw);
		assert_eq!(Scissors + Scissors, Draw);

		assert_eq!(Rock + Scissors, Lose);
		assert_eq!(Paper + Rock, Lose);
		assert_eq!(Scissors + Paper, Lose);
	}

	#[test]
	fn test_add_outcome_to_move() {
		assert_eq!(Rock + Win, Paper);
		assert_eq!(Paper + Win, Scissors);
		assert_eq!(Scissors + Win, Rock);

		assert_eq!(Rock + Draw, Rock);
		assert_eq!(Paper + Draw, Paper);
		assert_eq!(Scissors + Draw, Scissors);

		assert_eq!(Rock + Lose, Scissors);
		assert_eq!(Paper + Lose, Rock);
		assert_eq!(Scissors + Lose, Paper);
	}
}
