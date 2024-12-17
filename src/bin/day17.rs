use std::{fmt::Display, io::stdin};

use itertools::Itertools;

type Integer = u64;
type Registers = [Integer; 3];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Machine {
	pc: usize,
	registers: Registers,
}

impl Machine {
	pub fn new(registers: Registers) -> Self {
		Self { pc: 0, registers }
	}

	pub fn with_register_a(self, a: Integer) -> Self {
		Self {
			pc: self.pc,
			registers: [a, self.registers[1], self.registers[2]],
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Opcode {
	Adv = 0,
	Bxl = 1,
	Bst = 2,
	Jnz = 3,
	Bxc = 4,
	Out = 5,
	Bdv = 6,
	Cdv = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::From)]
struct Instruction {
	opcode: Opcode,
	argument: u8,
}

impl Instruction {
	fn execute(self, mut machine: Machine) -> (Machine, Option<u8>) {
		let Instruction { opcode, argument } = self;

		let literal_operand = || argument as Integer;
		let compo_operand = || match argument {
			v @ (0..=3) => v as Integer,
			v @ (4..=6) => machine.registers[(v - 4) as usize],
			_ => unimplemented!(),
		};
		let dv = || machine.registers[0] / (1 << compo_operand());

		let mut result = None;

		match opcode {
			Opcode::Adv => {
				machine.registers[0] = dv();
				machine.pc += 1;
			}
			Opcode::Bxl => {
				machine.registers[1] ^= literal_operand();
				machine.pc += 1;
			}
			Opcode::Bst => {
				machine.registers[1] = compo_operand() & 0b111;
				machine.pc += 1;
			}
			Opcode::Jnz => {
				if machine.registers[0] != 0 {
					machine.pc = (literal_operand() / 2) as _;
				} else {
					machine.pc += 1;
				}
			}
			Opcode::Bxc => {
				machine.registers[1] ^= machine.registers[2];
				machine.pc += 1;
			}
			Opcode::Out => {
				result = Some((compo_operand() & 0b111) as _);
				machine.pc += 1;
			}
			Opcode::Bdv => {
				machine.registers[1] = dv();
				machine.pc += 1;
			}
			Opcode::Cdv => {
				machine.registers[2] = dv();
				machine.pc += 1;
			}
		}

		(machine, result)
	}
}

impl TryFrom<(u8, u8)> for Instruction {
	type Error = <Opcode as TryFrom<u8>>::Error;

	fn try_from((opcode, argument): (u8, u8)) -> Result<Self, Self::Error> {
		let opcode = opcode.try_into()?;
		Ok(Self { opcode, argument })
	}
}

impl Display for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let combo_argument = || match self.argument {
			arg @ (0..=3) => (b'0' + arg) as char,
			arg @ (4..=6) => (b'A' + arg - 4) as char,
			_ => unreachable!("Unsupported"),
		};
		match self.opcode {
			Opcode::Adv => write!(f, "A = A / 2**{}", combo_argument()),
			Opcode::Bxl => write!(f, "B = B ^ {}", self.argument),
			Opcode::Bst => write!(f, "B = {} % 8", combo_argument()),
			Opcode::Jnz => write!(f, "if A != 0 {{ JUMP({}) }}", self.argument),
			Opcode::Bxc => write!(f, "B = B ^ C"),
			Opcode::Out => write!(f, "OUT({} % 8)", combo_argument()),
			Opcode::Bdv => write!(f, "B = A / 2**{}", combo_argument()),
			Opcode::Cdv => write!(f, "C = A / 2**{}", combo_argument()),
		}
	}
}

type Program<'a> = &'a [Instruction];

fn main() {
	let (registers, raw_program) = parse_input();
	let program = parse_program(raw_program.iter().copied()).unwrap();
	let machine = Machine::new(registers);

	println!("PROGRAM:");
	for instruction in program.iter() {
		println!("\t{}", instruction);
	}
	println!();

	{
		let (output, _) = run(machine, &program);
		println!("Part 1: {}", output.into_iter().join(","));
	}

	{
		let a = find_quine(machine, &program, &raw_program).expect("Expect answer for part 2");
		println!("Part 2: {a}");
	}
}

fn find_quine(machine: Machine, program: Program, raw_program: &[u8]) -> Option<u64> {
	fn find_quine(
		machine: Machine,
		pos: usize,
		program: Program,
		raw_program: &[u8],
	) -> Option<u64> {
		for v in 0..=0b111 {
			let a = (machine.registers[0] << 3) | v as u64;

			let machine = machine.with_register_a(a);
			let output = run(machine, program).0;

			if output == raw_program {
				return Some(a);
			}

			if output == last_n(raw_program, pos + 1) {
				let x = find_quine(machine, pos + 1, program, raw_program);
				if x.is_some() {
					return x;
				}
			}
		}

		None
	}

	find_quine(machine.with_register_a(0), 0, program, raw_program)
}

fn last_n<T>(slice: &[T], n: usize) -> &[T] {
	&slice[(slice.len() - n)..]
}

fn run(mut machine: Machine, program: Program) -> (Vec<u8>, Machine) {
	let mut output = Vec::new();

	while let Some(result) = step(machine, program) {
		machine = result.0;
		if let Some(v) = result.1 {
			output.push(v);
		}
	}

	(output, machine)
}

fn step(machine: Machine, instructions: &[Instruction]) -> Option<(Machine, Option<u8>)> {
	let instruction = instructions.get(machine.pc)?;
	Some(instruction.execute(machine))
}

fn parse_program<I, T>(
	program: I,
) -> Result<Vec<Instruction>, <Instruction as TryFrom<(u8, u8)>>::Error>
where
	I: IntoIterator<Item = T>,
	T: Into<u8>,
{
	program
		.into_iter()
		.map(|v| v.into())
		.tuples::<(_, _)>()
		.map(Instruction::try_from)
		.collect()
}

fn parse_input() -> (Registers, Vec<u8>) {
	let mut lines = stdin().lines().map(Result::unwrap);

	let mut register = || lines.next().unwrap()[12..].parse().unwrap();
	let registers = [register(), register(), register()];

	lines.next().unwrap();

	let raw_program = lines.next().unwrap()[9..]
		.split(',')
		.map(|d| d.parse().unwrap())
		.collect();

	(registers, raw_program)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example1() {
		assert_eq!(
			step(Machine::new([0, 0, 9]), &parse_program([2, 6]).unwrap()),
			Some((
				Machine {
					pc: 1,
					registers: [0, 1, 9]
				},
				None
			))
		);
	}

	#[test]
	fn test_example2() {
		assert_eq!(
			run(
				Machine::new([10, 0, 0]),
				&parse_program([5, 0, 5, 1, 5, 4]).unwrap()
			)
			.0,
			vec![0, 1, 2]
		);
	}

	#[test]
	fn test_example3() {
		let result = run(
			Machine::new([2024, 0, 0]),
			&parse_program([0, 1, 5, 4, 3, 0]).unwrap(),
		);
		assert_eq!(result.0, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
		assert_eq!(result.1.registers[0], 0);
	}

	#[test]
	fn test_example4() {
		assert_eq!(
			step(Machine::new([0, 29, 0]), &parse_program([1, 7]).unwrap()),
			Some((
				Machine {
					pc: 1,
					registers: [0, 26, 0]
				},
				None
			))
		);
	}

	#[test]
	fn test_example5() {
		assert_eq!(
			step(
				Machine::new([0, 2024, 43690]),
				&parse_program([4, 0]).unwrap()
			),
			Some((
				Machine {
					pc: 1,
					registers: [0, 44354, 43690]
				},
				None
			))
		);
	}
}
