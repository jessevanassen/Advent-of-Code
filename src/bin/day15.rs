use std::io::stdin;

use itertools::Itertools;

fn main() {
	let input = stdin()
		.lines()
		.flatten()
		.exactly_one()
		.map_err(|_| anyhow::anyhow!("Expected exactly one line of input"))
		.unwrap();

	let commands = input
		.split(',')
		.map(|command| Command::try_from(command).unwrap())
		.collect::<Vec<_>>();

	let part1 = commands
		.iter()
		.map(|Command { checksum, .. }| *checksum as u64)
		.sum::<u64>();
	println!("Part 1: {part1}");

	let mut boxes: Vec<Vec<(&str, u8)>> = vec![Vec::new(); 0xff + 1];
	for &Command {
		box_nr,
		label,
		action,
		..
	} in &commands
	{
		let box_ = &mut boxes[box_nr as usize];
		let lens_position = box_.iter().position(|(l, _)| *l == label);
		match action {
			Action::Remove => {
				if let Some(lens_position) = lens_position {
					box_.remove(lens_position);
				}
			}
			Action::Add { focal_length } => {
				if let Some(lens_position) = lens_position {
					box_[lens_position].1 = focal_length;
				} else {
					box_.push((label, focal_length));
				}
			}
		}
	}

	let part2 = boxes
		.iter()
		.zip(1..)
		.flat_map(|(box_, box_nr)| {
			box_.iter()
				.zip(1..)
				.map(move |((_, focal_length), lens_nr)| (box_nr, lens_nr, *focal_length))
		})
		.map(|(box_nr, lens_nr, focal_length)| box_nr * lens_nr * focal_length as usize)
		.sum::<usize>();

	println!("Part 2: {part2}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Command<'a> {
	checksum: u8,
	box_nr: u8,
	label: &'a str,
	action: Action,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
	Remove,
	Add { focal_length: u8 },
}

impl<'a> TryFrom<&'a str> for Command<'a> {
	type Error = anyhow::Error;

	fn try_from(input: &'a str) -> Result<Self, Self::Error> {
		let checksum = hash(input);

		let separator_pos = input
			.find(['-', '='])
			.ok_or_else(|| anyhow::anyhow!("Expected '-' or '=' separator"))?;
		let (label, input) = input.split_at(separator_pos);
		let box_nr = hash(label);

		let (separator, value) = input.split_at(1);

		let action = match separator {
			"-" => Action::Remove,
			"=" => {
				let focal_length = value.parse()?;
				Action::Add { focal_length }
			}
			_ => panic!(),
		};

		Ok(Self {
			checksum,
			box_nr,
			label,
			action,
		})
	}
}

fn hash(input: &str) -> u8 {
	input.bytes().fold(0u8, |acc, it| {
		let mut acc = acc as u16;
		acc += it as u16;
		acc *= 17;
		acc as u8
	})
}
