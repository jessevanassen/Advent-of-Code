use std::io::stdin;

use aoc2018::opcode::{Arguments, OpCode, Registers};
use itertools::Itertools;

fn main() {
	let (ip_register, instructions) = parse_input(stdin().lines().flatten());

	let mut registers: Registers = [0, 0, 0, 0, 0, 0];

	while let Some((opcode, arguments)) = instructions
		.get(registers[ip_register] as usize)
		.copied()
	{
		registers = opcode.apply(arguments, registers);
		registers[ip_register] += 1;
	}
	println!("Part 1: {}", registers[0]);
	println!();
	println!("{}", pretty_print_instructions(ip_register, &instructions));
}

type Instruction = (OpCode, Arguments);

fn pretty_print_instructions<'a>(
	ip_register: usize,
	instructions: impl IntoIterator<Item = &'a Instruction>,
) -> String {
	fn pretty_print((opcode, [i0, i1, o]): Instruction) -> String {
		match opcode {
			OpCode::AddRegister => {
				if i0 == o {
					format!("reg[{o}] += reg[{i1}]")
				} else if i1 == o {
					format!("reg[{o}] += reg[{i0}]")
				} else if i1 < i0 {
					format!("reg[{o}] = reg[{i1}] + reg[{i0}]")
				} else {
					format!("reg[{o}] = reg[{i0}] + reg[{i1}]")
				}
			}
			OpCode::AddImmediate => {
				if i0 == o {
					format!("reg[{o}] += {i1}")
				} else {
					format!("reg[{o}] = reg[{i0}] + {i1}")
				}
			}
			OpCode::MultiplyRegister => {
				if i0 == o {
					format!("reg[{o}] *= reg[{i1}]")
				} else if i1 == o {
					format!("reg[{o}] *= reg[{i0}]")
				} else if i1 < i0 {
					format!("reg[{o}] = reg[{i1}] * reg[{i0}]")
				} else {
					format!("reg[{o}] = reg[{i0}] * reg[{i1}]")
				}
			}
			OpCode::MultiplyImmediate => {
				if i0 == o {
					format!("reg[{o}] *= {i1}")
				} else {
					format!("reg[{o}] = reg[{i0}] * {i1}")
				}
			}
			OpCode::SetRegister => format!("reg[{o}] = reg[{i0}]"),
			OpCode::SetImmediate => format!("reg[{o}] = {i0}"),
			OpCode::GreaterThanRegisterRegister => format!("reg[{o}] = reg[{i0}] > reg[{i1}]"),
			OpCode::EqualRegisterRegister => format!("reg[{o}] = reg[{i0}] == reg[{i1}]"),
			_ => todo!(),
		}
	}

	let result = instructions
		.into_iter()
		.enumerate()
		.map(|(i, instruction)| format!("#{i:02}: {}", pretty_print(*instruction)))
		.join("\n");

	result.replace(&format!("reg[{ip_register}]"), "ip")
}

fn parse_input(lines: impl IntoIterator<Item = impl AsRef<str>>) -> (usize, Vec<Instruction>) {
	let mut iter = lines.into_iter();

	let ip = {
		let line = iter.next().unwrap();
		line.as_ref().as_bytes()[4] - b'0'
	} as usize;

	let instructions = iter
		.map(|line| {
			let line = line.as_ref();
			let mut iter = line.split(' ');

			let opcode = iter.next().unwrap().parse().unwrap();

			let mut argument = || iter.next().unwrap().parse().unwrap();
			let arguments = [argument(), argument(), argument()];

			(opcode, arguments)
		})
		.collect();

	(ip, instructions)
}
