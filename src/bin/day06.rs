use std::{
	io::{stdin, Read},
	ops::RangeInclusive,
};

use itertools::Itertools;

fn main() {
	let input = {
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		buf
	};

	let lines = input
		.lines()
		.map(|line| line.split_whitespace().skip(1).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	fn parse<'a>(numbers: &'a [&str]) -> impl Iterator<Item = usize> + 'a {
		numbers
			.iter()
			.map(|number| number.parse::<usize>().unwrap())
	}
	let times = parse(&lines[0]);
	let distances = parse(&lines[1]);

	let races = times.zip(distances);
	let part1: usize = races
		.map(|(time, distance)| hold_times(time, distance).count())
		.product::<usize>();
	println!("Part 1: {part1}");

	let parse = |numbers: &[&str]| numbers.iter().join("").parse().unwrap();
	let time = parse(&lines[0]);
	let distance = parse(&lines[1]);
	let part2 = hold_times(time, distance).count();
	println!("Part 2: {part2}");
}

fn hold_times(race_time: usize, target: usize) -> RangeInclusive<usize> {
	let mut hold_times = (1..)
		.map(move |hold_time| (hold_time, distance(hold_time, race_time)))
		.skip_while(move |(_, distance)| *distance <= target)
		.take_while(move |(_, distance)| *distance > target)
		.map(|(hold_time, _)| hold_time);

	let start = hold_times.next().unwrap();
	let end = hold_times.last().unwrap();
	start..=end
}

fn distance(hold_time: usize, race_time: usize) -> usize {
	hold_time * (race_time - hold_time)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hold_times() {
		assert_eq!(hold_times(7, 9), 2..=5);
		assert_eq!(hold_times(15, 40), 4..=11);
		assert_eq!(hold_times(30, 200), 11..=19);
	}

	#[test]
	fn test_distance() {
		assert_eq!(distance(1, 7), 6);
		assert_eq!(distance(2, 7), 10);
		assert_eq!(distance(3, 7), 12);
		assert_eq!(distance(4, 7), 12);
		assert_eq!(distance(5, 7), 10);
		assert_eq!(distance(6, 7), 6);
	}
}
