use std::io::stdin;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
	let input: Vec<u8> = stdin()
		.lines()
		.next()
		.context("Expect stdin input")??
		.bytes()
		.map(|b| b - b'0')
		.collect();

	let captcha = |offset: usize| (0..input.len())
		.filter(|&i| input[i] == input[(i + offset) % input.len()])
		.map(|i| input[i] as u32)
		.sum::<u32>();

	println!("Part 1: {}", captcha(1));
	println!("Part 2: {}", captcha(input.len() / 2));

	Ok(())
}
