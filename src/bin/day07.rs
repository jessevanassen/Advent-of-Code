use std::{
	collections::{HashMap, HashSet},
	io::stdin,
};

type Coord = (usize, usize);

fn main() {
	let lines = stdin().lines().map(Result::unwrap).collect::<Vec<_>>();
	let start: Coord = {
		let x = lines
			.first()
			.unwrap()
			.bytes()
			.position(|b| b == b'S')
			.unwrap();
		(x, 0)
	};
	let splitters: HashSet<Coord> = lines
		.iter()
		/* Empty lines can be skipped to compress the input and make the problem smaller */
		.filter(|line| !line.bytes().all(|b| b == b'.'))
		.enumerate()
		.flat_map(|(y, line)| {
			line.bytes()
				.enumerate()
				.filter(|(_, b)| *b == b'^')
				.map(move |(x, _)| (x, y))
		})
		.collect();

	let Solution { part1, part2 } = solve(start, &splitters);
	println!("Part 1: {part1}\nPart 2: {part2}");
}

#[derive(derive_more::Add)]
struct Solution {
	part1: usize,
	part2: usize,
}

fn solve(start: Coord, splitters: &HashSet<Coord>) -> Solution {
	fn solve(
		pos: Coord,
		splitters: &HashSet<Coord>,
		cache: &mut HashMap<Coord, usize>,
		y_limit: usize,
	) -> Solution {
		if pos.1 >= y_limit {
			return Solution { part1: 0, part2: 1 };
		}

		if let Some(v) = cache.get(&pos) {
			/* Return 0 for part 1 if the position has already been seen, as
			 * duplicate splits shouldn't be counted. */
			return Solution {
				part1: 0,
				part2: *v,
			};
		}

		let down = down(pos);
		let result = if splitters.contains(&down) {
			let mut solution = down_diagonally(pos)
				.map(|pos| solve(pos, splitters, cache, y_limit))
				.into_iter()
				.reduce(<Solution as std::ops::Add>::add)
				.unwrap();
			solution.part1 += 1;
			solution
		} else {
			solve(down, splitters, cache, y_limit)
		};

		cache.insert(pos, result.part2);
		result
	}

	let mut cache = HashMap::new();
	let y_limit = splitters.iter().map(|s| s.1).max().unwrap();
	solve(start, splitters, &mut cache, y_limit)
}

fn down((x, y): Coord) -> Coord {
	(x, y + 1)
}

fn down_diagonally((x, y): Coord) -> [Coord; 2] {
	[(x - 1, y + 1), (x + 1, y + 1)]
}
