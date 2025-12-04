use std::io::stdin;

use aoc2025::grid::{self, Grid};

fn main() {
	let grid: Vec<Vec<bool>> = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| line.bytes().map(|b| b == b'@').collect())
		.collect();
	let mut grid: Grid<_> = grid.try_into().unwrap();

	println!("Part 1: {}", removable(&grid).count());

	let mut part2 = 0;
	loop {
		let removable = removable(&grid).collect::<Vec<_>>();
		if removable.is_empty() {
			break;
		}

		part2 += removable.len();

		for index in removable {
			grid[index] = false;
		}
	}
	println!("Part 2: {part2}");
}

fn removable(grid: &Grid<bool>) -> impl Iterator<Item = grid::Index> {
	grid.indices().filter(|index| grid[*index]).filter(|index| {
		surrounding_indices(grid, *index)
			.filter(|index| grid[*index])
			.count() < 4
	})
}

fn surrounding_indices<T>(grid: &Grid<T>, idx: grid::Index) -> impl Iterator<Item = grid::Index> {
	let rows = idx.row.saturating_sub(1)..=usize::min(idx.row + 1, grid.height() - 1);
	let columns = idx.column.saturating_sub(1)..=usize::min(idx.column + 1, grid.width() - 1);

	rows.flat_map(move |row| {
		columns
			.clone()
			.map(move |column| grid::Index { row, column })
	})
	.filter(move |&i| i != idx)
}
