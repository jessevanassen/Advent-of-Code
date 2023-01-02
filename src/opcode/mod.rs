use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpCode {
	AddRegister,
	AddImmediate,

	MultiplyRegister,
	MultiplyImmediate,

	BitwiseAndRegister,
	BitwiseAndImmediate,

	BitwiseOrRegister,
	BitwiseOrImmediate,

	SetRegister,
	SetImmediate,

	GreaterThanImmediateRegister,
	GreaterThanRegisterImmediate,
	GreaterThanRegisterRegister,

	EqualImmediateRegister,
	EqualRegisterImmediate,
	EqualRegisterRegister,
}

pub type Value = u32;
pub type Arguments = [Value; 3];
pub type Registers = [Value; 6];

impl OpCode {
	pub fn apply(self, args: Arguments, mut register: Registers) -> Registers {
		let i0 = args[0];
		let i1 = args[1];
		let o = args[2] as usize;

		use OpCode::*;
		match self {
			AddRegister => {
				register[o] = register[i0 as usize] + register[i1 as usize];
			}
			AddImmediate => {
				register[o] = register[i0 as usize] + i1 as Value;
			}

			MultiplyRegister => {
				register[o] = register[i0 as usize] * register[i1 as usize];
			}
			MultiplyImmediate => {
				register[o] = register[i0 as usize] * i1 as Value;
			}

			BitwiseAndRegister => {
				register[o] = register[i0 as usize] & register[i1 as usize];
			}
			BitwiseAndImmediate => {
				register[o] = register[i0 as usize] & i1 as Value;
			}

			BitwiseOrRegister => {
				register[o] = register[i0 as usize] | register[i1 as usize];
			}
			BitwiseOrImmediate => {
				register[o] = register[i0 as usize] | i1 as Value;
			}

			SetRegister => {
				register[o] = register[i0 as usize];
			}
			SetImmediate => {
				register[o] = i0 as Value;
			}

			GreaterThanImmediateRegister => {
				register[o] = (i0 as Value > register[i1 as usize]) as Value;
			}
			GreaterThanRegisterImmediate => {
				register[o] = (register[i0 as usize] > i1 as Value) as Value;
			}
			GreaterThanRegisterRegister => {
				register[o] = (register[i0 as usize] > register[i1 as usize]) as Value;
			}

			EqualImmediateRegister => {
				register[o] = (i0 as Value == register[i1 as usize]) as Value;
			}
			EqualRegisterImmediate => {
				register[o] = (register[i0 as usize] == i1 as Value) as Value;
			}
			EqualRegisterRegister => {
				register[o] = (register[i0 as usize] == register[i1 as usize]) as Value;
			}
		}

		register
	}
}

impl FromStr for OpCode {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use OpCode::*;
		Ok(match s {
			"addr" => AddRegister,
			"addi" => AddImmediate,
			"mulr" => MultiplyRegister,
			"muli" => MultiplyImmediate,
			"banr" => BitwiseAndRegister,
			"bani" => BitwiseAndImmediate,
			"borr" => BitwiseOrRegister,
			"bori" => BitwiseOrImmediate,
			"setr" => SetRegister,
			"seti" => SetImmediate,
			"gtir" => GreaterThanImmediateRegister,
			"gtri" => GreaterThanRegisterImmediate,
			"gtrr" => GreaterThanRegisterRegister,
			"eqir" => EqualImmediateRegister,
			"eqri" => EqualRegisterImmediate,
			"eqrr" => EqualRegisterRegister,
			other => anyhow::bail!("Unknown opcode {other}"),
		})
	}
}
