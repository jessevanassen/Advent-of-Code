use std::io::stdin;

fn main() {
	let input = stdin()
		.lines()
		.flatten()
		.map(|line| line.parse::<i64>().unwrap())
		.enumerate()
		.collect::<Vec<_>>();

	let mut part1 = input.clone();
	process(&mut part1);
	println!("Part 1: {}", grove_coordinates(&part1));

	let mut part2 = input
		.iter()
		.map(|&(i, v)| (i, v * 811589153))
		.collect::<Vec<_>>();
	for _ in 0..10 {
		process(&mut part2);
	}
	println!("Part 2: {}", grove_coordinates(&part2));
}

fn process(xs: &mut Vec<(usize, i64)>) {
	for i in 0..xs.len() {
		let index = xs
			.iter()
			.position(|(index, _)| &i == index)
			.unwrap();
		let item = xs.remove(index);
		let mut new_index = (index as i64 + item.1).rem_euclid(xs.len() as i64) as usize;
		if new_index == 0 {
			/* Technically not needed, but this ensures that the array matches
			 * the result in the example. */
			new_index = xs.len();
		}
		xs.insert(new_index, item);
	}
}

fn grove_coordinates(xs: &[(usize, i64)]) -> i64 {
	let start = xs
		.iter()
		.position(|(_, v)| v == &0)
		.unwrap();
	((1000..=3000).step_by(1000))
		.map(|v| (v + start) % xs.len())
		.map(|i| xs[i].1)
		.sum()
}
