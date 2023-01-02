use std::io::stdin;

use aoc2018::opcode::{OpCode, Registers};

static ALL_OPCODES: [OpCode; 16] = [
	OpCode::AddRegister,
	OpCode::AddImmediate,
	OpCode::MultiplyRegister,
	OpCode::MultiplyImmediate,
	OpCode::BitwiseAndRegister,
	OpCode::BitwiseAndImmediate,
	OpCode::BitwiseOrRegister,
	OpCode::BitwiseOrImmediate,
	OpCode::SetRegister,
	OpCode::SetImmediate,
	OpCode::GreaterThanImmediateRegister,
	OpCode::GreaterThanRegisterImmediate,
	OpCode::GreaterThanRegisterRegister,
	OpCode::EqualImmediateRegister,
	OpCode::EqualRegisterImmediate,
	OpCode::EqualRegisterRegister,
];

fn main() {
	let (samples, example_program) = parse_input(stdin().lines().flatten());

	let part1 = samples
		.iter()
		.filter(|sample| possible_opcodes(sample, &ALL_OPCODES).count() >= 3)
		.count();
	println!("Part 1: {part1}");

	let opcodes = derive_opcode_values(&samples);

	let mut registers: Registers = Default::default();
	for instruction in example_program {
		registers = opcodes[instruction.opcode as usize].apply(instruction.arguments, registers);
	}

	println!("Part 2: {}", registers[0]);
}

fn derive_opcode_values(samples: &[Sample]) -> [OpCode; 16] {
	let mut potential_opcodes: [Vec<OpCode>; 16] = Default::default();
	for it in potential_opcodes.iter_mut() {
		it.extend_from_slice(&ALL_OPCODES);
	}

	while !potential_opcodes
		.iter()
		.all(|p| p.len() == 1)
	{
		for sample in samples {
			potential_opcodes[sample.instruction.opcode as usize] =
				possible_opcodes(sample, &potential_opcodes[sample.instruction.opcode as usize])
					.collect();
		}

		// If there's only a single possibility, remove it from the other entries
		for i in 0..potential_opcodes.len() {
			if potential_opcodes[i].len() > 1 {
				continue;
			}

			let opcode = potential_opcodes[i][0];

			for j in (0..potential_opcodes.len()).filter(|j| *j != i) {
				potential_opcodes[j].retain(|it| *it != opcode);
			}
		}
	}

	potential_opcodes.map(|xs| xs[0])
}

fn possible_opcodes<'a>(
	sample: &'a Sample,
	possible_opcodes: &'a [OpCode],
) -> impl Iterator<Item = OpCode> + 'a {
	possible_opcodes
		.iter()
		.copied()
		.filter(move |opcode| opcode.apply(sample.instruction.arguments, sample.before) == sample.after)
}

type Arguments = [u8; 3];

#[derive(Debug)]
struct Instruction {
	opcode: u8,
	arguments: Arguments,
}

#[derive(Debug)]
struct Sample {
	before: Registers,
	instruction: Instruction,
	after: Registers,
}

fn parse_input(
	lines: impl IntoIterator<Item = impl AsRef<str>>,
) -> (Vec<Sample>, Vec<Instruction>) {
	fn ascii_digit(char: u8) -> u8 {
		char - b'0'
	}

	fn parse_register(line: &str) -> Registers {
		let mut iter = line
			.bytes()
			.skip(9)
			.step_by(3)
			.take(4)
			.map(|x| ascii_digit(x) as u32);
		let mut next = || iter.next().unwrap();
		[next(), next(), next(), next()]
	}

	fn parse_instruction(line: &str) -> Instruction {
		let mut iter = line
			.split(' ')
			.map(|p| p.parse().unwrap());
		let mut next = || iter.next().unwrap();
		Instruction {
			opcode: next(),
			arguments: [next(), next(), next()],
		}
	}

	let mut iter = lines.into_iter().peekable();
	let mut samples = Vec::new();

	while matches!(iter.peek(), Some(line) if !line.as_ref().is_empty()) {
		samples.push(Sample {
			before: parse_register(iter.next().unwrap().as_ref()),
			instruction: parse_instruction(iter.next().unwrap().as_ref()),
			after: parse_register(iter.next().unwrap().as_ref()),
		});

		iter.next(); // Consume separator
	}

	iter.next();
	iter.next();

	let example_program = iter
		.map(|line| parse_instruction(line.as_ref()))
		.collect();

	(samples, example_program)
}
