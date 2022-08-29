use std::{io::stdin, str::FromStr};

pub fn read_lines_from_stdin<T>() -> Vec<T>
where
	T: FromStr,
{
	stdin().lines()
		.filter_map(|line| line.ok())
		.filter_map(|line| line.parse().ok())
		.collect()
}
