use std::io::stdin;

use aoc2025::{minmax, vector::Vector3D};
use itertools::Itertools;

fn main() {
	let input = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| {
			let mut iter = line.split(',').map(|n| n.parse().unwrap());
			let x = iter.next().unwrap();
			let y = iter.next().unwrap();
			let z = iter.next().unwrap();
			assert_eq!(iter.next(), None);
			Vector3D { x, y, z }
		})
		.collect::<Vec<_>>();

	let distances = input
		.iter()
		.enumerate()
		.array_combinations::<2>()
		.map(|combination| {
			(
				combination.map(|(i, _)| i),
				distance_between(*combination[0].1, *combination[1].1),
			)
		})
		.sorted_by_key(|(_, distance)| *distance)
		.collect::<Vec<_>>();

	let mut connections_index_iter = 0usize..;
	let mut connections: Vec<Option<usize>> = vec![None; input.len()];

	for (index, &([i, j], _)) in distances.iter().enumerate() {
		match (connections[i], connections[j]) {
			(None, None) => {
				let connection = connections_index_iter.next().unwrap();
				connections[i] = Some(connection);
				connections[j] = Some(connection);
			}
			(None, Some(connection)) => {
				connections[i] = Some(connection);
			}
			(Some(connection), None) => {
				connections[j] = Some(connection);
			}
			(Some(connection1), Some(connection2)) => {
				let [min, max] = minmax(connection1, connection2);

				for g in connections.iter_mut() {
					if *g == Some(max) {
						*g = Some(min);
					}
				}
			}
		}

		if index == 1000 {
			let part1 = connections
				.iter()
				.flatten()
				.counts()
				.values()
				.sorted()
				.rev()
				.take(3)
				.product::<usize>();
			println!("Part 1: {part1}");
		}

		if connections.iter().all(|v| *v == Some(0)) {
			let part2 = input[i].x * input[j].x;
			println!("Part 2: {part2}");
			break;
		}
	}
}

fn distance_between(a: Vector3D, b: Vector3D) -> i64 {
	/* Because we're not interested in the actual distance, but only in sorting
	 * between the distances, we can avoid doing a sqrt and converting to f64. */
	<[i64; 3]>::from(a)
		.into_iter()
		.zip(<[i64; 3]>::from(b))
		.map(|(a, b)| (b - a).checked_pow(2).unwrap())
		.reduce(|a, b| i64::checked_add(a, b).unwrap())
		.unwrap()
}
