use std::{io::stdin, str::FromStr};

pub fn read_lines_from_stdin<T>() -> Vec<T>
where
	T: FromStr,
{
	stdin().lines()
		.flat_map(|line| line)
		.flat_map(|line| line.parse())
		.collect()
}
