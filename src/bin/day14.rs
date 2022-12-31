use std::env::args;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
	let input = args()
		.skip(1)
		.next()
		.context("Missing required program argument")?
		.bytes()
		.map(|b| b - b'0')
		.collect::<Vec<_>>();
	let recipe_count = input
		.iter()
		.fold(0usize, |acc, it| acc * 10 + *it as usize);

	{
		let mut factory = Factory::new();
		while factory.recipes().len() < recipe_count + 10 {
			factory.produce();
		}

		println!(
			"Part 1: {}",
			factory.recipes()[recipe_count..]
				.iter()
				.map(|b| (b + b'0') as char)
				.collect::<String>()
		);
	}

	{
		let mut factory = Factory::new();

		let preceding = loop {
			if factory.recipes().len() > input.len() {
				let start_index = factory.recipes().len() - input.len() - 1;
				if let Some(found) = factory.recipes()[start_index..]
					.windows(input.len())
					.position(|w| w == input)
				{
					break start_index + found;
				}
			}

			factory.produce();
		};
		println!("Part 2: {preceding}");
	}

	Ok(())
}

pub struct Factory {
	recipes: Vec<u8>,
	elves: [usize; 2],
}

impl Factory {
	fn new() -> Self {
		Factory {
			recipes: vec![3, 7],
			elves: [0, 1],
		}
	}

	pub fn produce(&mut self) {
		let new_recipe = self.recipes[self.elves[0]] + self.recipes[self.elves[1]];

		if new_recipe >= 10 {
			self.recipes.push(new_recipe / 10);
		}
		self.recipes.push(new_recipe % 10);

		for elf in self.elves.iter_mut() {
			*elf = (*elf + self.recipes[*elf] as usize + 1) % self.recipes.len();
		}
	}

	pub fn recipes(&self) -> &[u8] {
		&self.recipes
	}
}
