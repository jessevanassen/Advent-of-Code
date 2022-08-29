use std::str::FromStr;

use advent_of_code_2021::read_lines_from_stdin;

#[derive(Debug, PartialEq)]
struct Command { x: i32, y: i32 }
impl FromStr for Command {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (command, amount) = s.split_once(' ').ok_or(())?;
		let amount: i32 = amount.parse().map_err(|_| ())?;
		match command {
			"forward" => Ok(Self { x: amount, y: 0 }),
			"down" => Ok(Self { x: 0, y: amount }),
			"up" => Ok(Self { x: 0, y: -amount }),
			_ => Err(())
		}
	}
}

#[derive(Debug, Default, PartialEq)]
struct Position { x: i32, y: i32, aim: i32 }
impl Position {
	fn add_absolute(self, rhs: &Command) -> Self {
		Self { x: self.x + rhs.x, y: self.y + rhs.y, ..self }
	}

	fn add_relative(self, rhs: &Command) -> Self {
		match rhs {
			Command { x: 0, y } => Self {
				aim: self.aim + y,
				..self },
			Command { y: 0, x} => Self {
				x: self.x + x,
				y: self.y + self.aim * x,
				..self },
			_ => self,
		}
	}
}

fn main() {
	let commands = read_lines_from_stdin::<Command>();

	let Position { x, y, .. } = commands.iter().fold(Position::default(), Position::add_absolute);
	println!("Part 1: {}", x * y);

	let Position { x, y, .. } = commands.iter().fold(Position::default(), Position::add_relative);
	println!("Part 2: {}", x * y);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_forward() {
		assert_eq!(Ok(Command { x: 10, y: 0 }), "forward 10".parse());
	}

	#[test]
	fn test_parse_up() {
		assert_eq!(Ok(Command { x: 0, y: -10 }), "up 10".parse());
	}

	#[test]
	fn test_parse_down() {
		assert_eq!(Ok(Command { x: 0, y: 10 }), "down 10".parse());
	}

	#[test]
	fn test_commands_absolute_to_position() {
		let position = Position::default()
			.add_absolute(&Command { x: 0, y: 10 })
			.add_absolute(&Command { x: 10, y: 0 })
			.add_absolute(&Command { x: 0, y: -5 })
			;
		assert_eq!(Position { x: 10, y: 5, aim: 0 }, position);
	}

	#[test]
	fn test_commands_relative_to_position() {
		let position = Position::default()
			.add_relative(&Command { x: 0, y: 10 })
			.add_relative(&Command { x: 0, y: -5 })
			.add_relative(&Command { x: 10, y: 0 })
			;
		assert_eq!(Position { x: 10, y: 50, aim: 5 }, position);
	}
}
