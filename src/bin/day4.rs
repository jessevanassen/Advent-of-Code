use std::fmt::Display;

use advent_of_code_2021::read_lines_from_stdin;

#[derive(Debug)]
struct Board([u8; 25]);
impl Board {
	const LINES: [[u8; 5]; 10] = [
		[0,  1,  2,  3,  4],
		[5,  6,  7,  8,  9],
		[10, 11, 12, 13, 14],
		[15, 16, 17, 18, 19],
		[20, 21, 22, 23, 24],

		[0,  5,  10, 15, 20],
		[1,  6,  11, 16, 21],
		[2,  7,  12, 17, 22],
		[3,  8,  13, 18, 23],
		[4,  9,  14, 19, 24],
	];

	pub fn is_complete(&self, drawn_numbers: &[u8]) -> bool {
		Self::LINES.iter()
			.any(|line| line.iter()
				.map(|i| self.0[*i as usize])
				.all(|n| drawn_numbers.contains(&n)))
	}

	pub fn score(&self, drawn_numbers: &[u8]) -> u32 {
		let sum = self.0.into_iter()
			.filter(|n| !drawn_numbers.contains(n))
			.fold(0u32, |x, y| x + y as u32);
		sum * (*drawn_numbers.last().unwrap() as u32)
	}
}
impl From<[u8; 25]> for Board {
	fn from(src: [u8; 25]) -> Self { Self(src) }
}
impl TryFrom<Vec<u8>> for Board {
	type Error = ();

	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
		let array: [u8; 25] = value.try_into().map_err(|_| ())?;
		Ok(Self(array))
	}
}
impl Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for (i, number) in self.0.iter().enumerate() {
			write!(f, "{:>2}", number)?;
			if i < self.0.len() - 1 && (i + 1) % 5 == 0 {
				writeln!(f)?;
			} else {
				write!(f, " ")?;
			}
		}
		Ok(())
	}
}

fn main() {
	let lines = read_lines_from_stdin::<String>();
	let drawn_numbers: Vec<u8> = lines[0].split(',').map(|n| n.parse().unwrap()).collect();
	let boards = (2..lines.len()).step_by(6)
		.map(|i| &lines[i..(i+5)])
		.map(parse_board)
		.collect::<Vec<_>>();

	let winning_boards = boards.iter()
		.map(|board| {
			let drawn_count = (1..=drawn_numbers.len())
				.filter(|end| board.is_complete(&drawn_numbers[0..*end]))
				.next().unwrap();
			(drawn_count, board.score(&drawn_numbers[0..drawn_count]))
		})
		.collect::<Vec<_>>();
	let (_, first) = winning_boards.iter().min_by_key(|(score, _)| score).unwrap();
	let (_, last) = winning_boards.iter().max_by_key(|(score, _)| score).unwrap();
	println!("First: {}, last: {}", first, last);
}

fn parse_board(lines: &[String]) -> Board {
	lines
		.iter()
		.flat_map(|line| line.split_whitespace())
		.map(|number| number.parse().unwrap())
		.collect::<Vec<u8>>()
		.try_into().unwrap()
}
