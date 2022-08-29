use advent_of_code_2021::read_lines_from_stdin;

fn main() {
	let input = read_lines_from_stdin::<u32>();

	println!("Part 1: {}", count_increases(&input));

	let windows = (0..(input.len() - 2))
		.map(|i| input[i+0] + input[i+1] + input[i+2])
		.collect::<Vec<_>>()
		;
	println!("Part 2: {}", count_increases(&windows));
}

fn count_increases(items: &[u32]) -> usize {
	(0..(items.len() - 1))
		.filter(|i| items[*i] < items[*i + 1])
		.count()
}
