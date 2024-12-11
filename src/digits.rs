pub fn count_digits(input: u64) -> u32 {
	if input > 0 {
		input.ilog10() + 1
	} else {
		1
	}
}
