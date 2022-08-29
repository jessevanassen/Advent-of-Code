use advent_of_code_2021::read_lines_from_stdin;

fn main() {
	let bytes = read_lines_from_stdin::<String>().into_iter()
		.map(|line| -> [bool; 5] {
			let bits = line.bytes()
				.map(|b| b == b'1')
				.collect::<Vec<_>>()
				;
			bits.try_into().unwrap()
		})
		.collect::<Vec<_>>()
		;
	let occurrences = bytes.iter()
		.fold([0; 5], |acc, i| add(acc, *i));

	let gamma_bin = occurrences.map(|x| x >= bytes.len() as i32 / 2);
	let gamma = from_binary(&gamma_bin);
	let epsilon = 0b11111 ^ gamma;

	println!("Part 1: Gamma: {}, Epsilon: {}, Power consumption: {}", gamma, epsilon, gamma * epsilon);
}

fn add(lhs: [i32; 5], rhs: [bool; 5]) -> [i32; 5] {
	(0..lhs.len())
		.map(|i| lhs[i] + to_number(rhs[i]))
		.collect::<Vec<_>>().try_into().unwrap()
}

fn from_binary(binary: &[bool]) -> i32 {
	binary.iter()
		.fold(0, |acc, x| (acc << 1) | to_number(*x))
}

fn to_number(bit: bool) -> i32 { if bit { 1 } else { 0 } }
