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

pub type Register = u32;
pub type Registers = [Register; 4];

impl OpCode {
	pub fn apply(self, args: [u8; 3], mut register: Registers) -> Registers {
		let i0 = args[0];
		let i1 = args[1];
		let o = args[2] as usize;

		use OpCode::*;
		match self {
			AddRegister => {
				register[o] = register[i0 as usize] + register[i1 as usize];
			}
			AddImmediate => {
				register[o] = register[i0 as usize] + i1 as Register;
			}

			MultiplyRegister => {
				register[o] = register[i0 as usize] * register[i1 as usize];
			}
			MultiplyImmediate => {
				register[o] = register[i0 as usize] * i1 as Register;
			}

			BitwiseAndRegister => {
				register[o] = register[i0 as usize] & register[i1 as usize];
			}
			BitwiseAndImmediate => {
				register[o] = register[i0 as usize] & i1 as Register;
			}

			BitwiseOrRegister => {
				register[o] = register[i0 as usize] | register[i1 as usize];
			}
			BitwiseOrImmediate => {
				register[o] = register[i0 as usize] | i1 as Register;
			}

			SetRegister => {
				register[o] = register[i0 as usize];
			}
			SetImmediate => {
				register[o] = i0 as Register;
			}

			GreaterThanImmediateRegister => {
				register[o] = (i0 as Register > register[i1 as usize]) as Register;
			}
			GreaterThanRegisterImmediate => {
				register[o] = (register[i0 as usize] > i1 as Register) as Register;
			}
			GreaterThanRegisterRegister => {
				register[o] = (register[i0 as usize] > register[i1 as usize]) as Register;
			}

			EqualImmediateRegister => {
				register[o] = (i0 as Register == register[i1 as usize]) as Register;
			}
			EqualRegisterImmediate => {
				register[o] = (register[i0 as usize] == i1 as Register) as Register;
			}
			EqualRegisterRegister => {
				register[o] = (register[i0 as usize] == register[i1 as usize]) as Register;
			}
		}

		register
	}
}
