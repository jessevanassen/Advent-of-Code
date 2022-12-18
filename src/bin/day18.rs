use std::{collections::HashSet, io::stdin};

use anyhow::{anyhow, Context};

type Coord = [i32; 3];

fn main() -> anyhow::Result<()> {
	let lava: HashSet<Coord> = stdin()
		.lines()
		.flatten()
		.map(|ref line| {
			parse_coord(line)
				.with_context(|| anyhow::format_err!("Unable to parse coordinate {line}"))
		})
		.collect::<anyhow::Result<_>>()?;

	let water = water_surrounding(&lava);

	let (part1, part2) = lava
		.iter()
		.flat_map(adjacent)
		.fold((0, 0), |acc, ref c| {
			(
				acc.0 + !lava.contains(c) as usize,
				acc.1 + water.contains(c) as usize,
			)
		});

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");

	Ok(())
}

/// Contains the box of water coordinates surrounding the lava blob.
///
/// Excludes coordinates that are not reachable from outside the blob (i.e.
/// pockets that are completely surrounded by lava).
///
/// # Example
/// If the lava blob would look like this:
/// ```text
///   X
///  X X
///   X
/// ```
/// The result will be like this:
/// ```text
/// ~~~~~
/// ~~X~~
/// ~X X~
/// ~~X~~
/// ~~~~~
/// ```
fn water_surrounding(lava: &HashSet<Coord>) -> HashSet<Coord> {
	let (min, max) = min_max(lava).unwrap();

	/* Contains the range of the lava blob, and the border around it. */
	let is_in_range = {
		let xs = (min[0] - 1)..=(max[0] + 1);
		let ys = (min[1] - 1)..=(max[1] + 1);
		let zs = (min[2] - 1)..=(max[2] + 1);
		move |[x, y, z]: &Coord| xs.contains(x) && ys.contains(y) && zs.contains(z)
	};

	let mut water: HashSet<Coord> = HashSet::new();
	let mut to_check: Vec<Coord> = vec![min];

	while let Some(c) = to_check.pop() {
		for c in adjacent(&c) {
			if is_in_range(&c) && !water.contains(&c) && !lava.contains(&c) {
				water.insert(c);
				to_check.push(c)
			}
		}
	}

	water
}

fn adjacent(&[x, y, z]: &Coord) -> [Coord; 6] {
	[
		[x - 1, y, z],
		[x + 1, y, z],
		[x, y - 1, z],
		[x, y + 1, z],
		[x, y, z - 1],
		[x, y, z + 1],
	]
}

fn min_max<'a>(coords: impl IntoIterator<Item = &'a Coord>) -> Option<(Coord, Coord)> {
	coords
		.into_iter()
		.fold(None, |acc, c| match acc {
			Some((min, max)) => Some((
				[min[0].min(c[0]), min[1].min(c[1]), min[2].min(c[2])],
				[max[0].max(c[0]), max[1].max(c[1]), max[2].max(c[2])],
			)),
			None => Some(((*c), (*c))),
		})
}

fn parse_coord(line: &str) -> anyhow::Result<Coord> {
	let mut points = line.splitn(3, ',');
	let mut next = || -> anyhow::Result<_> {
		Ok(points
			.next()
			.ok_or_else(|| anyhow!("Too few elements in coordinate"))?
			.parse()?)
	};
	Ok([next()?, next()?, next()?])
}
