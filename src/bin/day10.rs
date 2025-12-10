use std::{convert::Infallible, io::stdin, str::FromStr};

use itertools::Itertools;

fn main() {
	let machines = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| line.parse().unwrap())
		.collect::<Vec<Machine>>();

	let part1 = machines
		.iter()
		.map(|machine| minimum_presses(machine).expect("Expect machine to be solvable"))
		.sum::<usize>();
	println!("Part 1: {part1}");
}

fn minimum_presses(machine: &Machine) -> Option<usize> {
	for buttons in machine.button_wirings.iter().powerset() {
		let mut acc = 0;

		for button in &buttons {
			acc ^= *button;
		}

		if acc == machine.indicator_lights {
			return Some(buttons.len());
		}
	}

	None
}

struct Machine {
	indicator_lights: u16,
	button_wirings: Box<[u16]>,
	#[allow(unused)]
	joltage_requirements: Box<[usize]>,
}

impl FromStr for Machine {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		fn shrink(s: &str) -> &str {
			&s[1..(s.len() - 1)]
		}

		fn parse_digit_group(s: &str) -> impl Iterator<Item = usize> {
			shrink(s).split(',').map(|d| d.parse().unwrap())
		}

		let parts = s.split_ascii_whitespace().collect::<Vec<_>>();

		let indicator_lights = shrink(parts[0]).bytes().enumerate().fold(0, |acc, (i, b)| {
			let bit = ((b == b'#') as u16) << i;
			acc | bit
		});

		let button_wirings = parts[1..(parts.len() - 1)]
			.iter()
			.map(|wiring| parse_digit_group(wiring).fold(0, |acc, b| acc | (1 << b)))
			.collect();

		let joltage_requirements = parse_digit_group(parts.last().unwrap()).collect();

		Ok(Self {
			indicator_lights,
			button_wirings,
			joltage_requirements,
		})
	}
}
