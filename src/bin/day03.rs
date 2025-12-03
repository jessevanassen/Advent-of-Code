use std::io::stdin;

fn main() {
	let batteries = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<_>>())
		.collect::<Vec<_>>();

	println!(
		"Part 1: {}",
		batteries.iter().map(|b| maximum_joltage(b, 2)).sum::<u64>(),
	);

	println!(
		"Part 2: {}",
		batteries
			.iter()
			.map(|b| maximum_joltage(b, 12))
			.sum::<u64>(),
	);
}

fn maximum_joltage(battery: &[u8], banks: usize) -> u64 {
	if banks == 0 {
		return 0;
	}

	assert!(battery.len() >= banks);

	let (index, v) = battery
		.iter()
		.copied()
		.enumerate()
		.take(battery.len() - (banks - 1)) /* Ensure enough remaining battery cells for the remaining recursive calls */
		.rev() /* max_by_key takes the last item in case of duplicates, but the first is needed for the correct index */
		.max_by_key(|(_, v)| *v)
		.unwrap();

	let banks = banks - 1;
	v as u64 * 10u64.pow(banks as _) /* The bank acts as the digit's place value  */
		+ maximum_joltage(&battery[(index + 1)..], banks)
}
