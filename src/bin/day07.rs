use std::{
	io::stdin,
	iter::{self, Peekable},
};

use aoc2022::extensions::Pipe;

type FileSize = usize;

fn main() {
	let directory_sizes = stdin()
		.lines()
		.flatten()
		.map_self(parse_commands)
		.map_self(directory_sizes);

	println!(
		"Part 1: {}",
		directory_sizes
			.iter()
			.filter(|&&s| s < 100000)
			.sum::<usize>()
	);

	const TOTAL_SPACE: usize = 70000000;
	const REQUIRED_SPACE: usize = 30000000;
	let unused_space = TOTAL_SPACE - directory_sizes.iter().max().unwrap();

	println!(
		"Part 2: {}",
		directory_sizes
			.iter()
			.filter(|&&s| unused_space + s >= REQUIRED_SPACE)
			.min()
			.unwrap()
	);
}

fn directory_sizes(commands: impl IntoIterator<Item = Command>) -> Vec<FileSize> {
	fn directory_sizes(
		commands: &mut Peekable<impl Iterator<Item = Command>>,
		acc: &mut Vec<FileSize>,
	) {
		if !matches!(commands.next(), Some(Command::CdDown)) {
			panic!("Expected first command to be cd");
		};

		let mut directory_size = if let Some(Command::Ls { file_sizes }) = commands.next() {
			file_sizes.iter().copied().sum()
		} else {
			panic!("Expected second command to be ls")
		};

		while matches!(commands.peek(), Some(Command::CdDown)) {
			directory_sizes(commands, acc);
			directory_size += acc.last().copied().unwrap_or(0);
		}

		acc.push(directory_size);

		let next = commands.next();
		if !matches!(next, None | Some(Command::CdUp)) {
			panic!("Expected either no remaining commands, or 'cd ..', but got {next:?}");
		}
	}

	let mut acc = Vec::new();
	directory_sizes(&mut commands.into_iter().peekable(), &mut acc);
	acc
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Command {
	Ls { file_sizes: Vec<FileSize> },
	CdUp,
	CdDown,
}

fn parse_commands(lines: impl IntoIterator<Item = String>) -> impl Iterator<Item = Command> {
	fn is_command(line: &str) -> bool {
		line.starts_with("$ ")
	}

	fn parse_command(lines: &mut Peekable<impl Iterator<Item = String>>) -> Option<Command> {
		let command = lines.next()?;
		Some(match &command[2..4] {
			"cd" => {
				let directory = &command[5..];
				if directory == ".." {
					Command::CdUp
				} else {
					Command::CdDown
				}
			}
			"ls" => {
				let mut file_sizes = Vec::new();

				while let Some(entry) = lines.next_if(|l| !is_command(l)) {
					let marker = entry
						.split(' ')
						.next()
						.expect("Expect ls entries to be separated by a space");
					if marker != "dir" {
						let size = marker
							.parse()
							.expect("Expect file size to be an integer");
						file_sizes.push(size);
					}
				}

				Command::Ls { file_sizes }
			}
			_ => panic!("Unknown command {command}"),
		})
	}

	let mut iter = lines.into_iter().peekable();
	iter::from_fn(move || parse_command(&mut iter))
}
