#![allow(clippy::identity_op)] // Makes the index operations look nicer

use std::io::stdin;

type Grid = aoc2024::Grid<u8>;

fn main() {
	let grid = parse_input();

	println!("Part 1: {}", windows(&grid).filter(is_xmas).count());
	println!("Part 2: {}", crosses(&grid).filter(is_xmax_cross).count());
}

fn windows(grid: &Grid) -> impl Iterator<Item = [u8; 4]> + '_ {
	/// ```txt
	/// 0123
	/// ```
	fn horizontal_windows(grid: &Grid) -> impl Iterator<Item = [u8; 4]> + '_ {
		coords(grid.width() - 3, grid.height()).map(|(x, y)| {
			[
				grid[(x + 0, y)],
				grid[(x + 1, y)],
				grid[(x + 2, y)],
				grid[(x + 3, y)],
			]
		})
	}

	/// ```txt
	/// 0
	/// 1
	/// 2
	/// 3
	/// ```
	fn vertical_windows(grid: &Grid) -> impl Iterator<Item = [u8; 4]> + '_ {
		coords(grid.width(), grid.height() - 3).map(|(x, y)| {
			[
				grid[(x, y + 0)],
				grid[(x, y + 1)],
				grid[(x, y + 2)],
				grid[(x, y + 3)],
			]
		})
	}

	/// ```txt
	/// 0..3
	/// .12.
	/// .12.
	/// 0..3
	/// ```
	fn diagonal_windows(grid: &Grid) -> impl Iterator<Item = [u8; 4]> + '_ {
		coords(grid.width() - 3, grid.height() - 3).flat_map(|(x, y)| {
			[
				[
					grid[(x + 0, y + 0)],
					grid[(x + 1, y + 1)],
					grid[(x + 2, y + 2)],
					grid[(x + 3, y + 3)],
				],
				[
					grid[(x + 0, y + 3)],
					grid[(x + 1, y + 2)],
					grid[(x + 2, y + 1)],
					grid[(x + 3, y + 0)],
				],
			]
		})
	}

	horizontal_windows(grid)
		.chain(vertical_windows(grid))
		.chain(diagonal_windows(grid))
}

/// ```txt
/// 0.1
/// .2.
/// 3.4
/// ```
fn crosses(grid: &Grid) -> impl Iterator<Item = [u8; 5]> + '_ {
	coords(grid.width() - 2, grid.height() - 2).map(|(x, y)| {
		[
			grid[(x + 0, y + 0)],
			grid[(x + 2, y + 0)],
			grid[(x + 1, y + 1)],
			grid[(x + 0, y + 2)],
			grid[(x + 2, y + 2)],
		]
	})
}

fn is_xmas(window: &[u8; 4]) -> bool {
	const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];
	const SAMX: [u8; 4] = [b'S', b'A', b'M', b'X'];

	window == &XMAS || window == &SAMX
}

fn is_xmax_cross(cross: &[u8; 5]) -> bool {
	cross[2] == b'A'
		&& ((cross[0] == b'M' && cross[4] == b'S') || (cross[4] == b'M' && cross[0] == b'S'))
		&& ((cross[1] == b'M' && cross[3] == b'S') || (cross[3] == b'M' && cross[1] == b'S'))
}

fn coords(width: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
	(0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))
}

fn parse_input() -> Grid {
	stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| line.into_bytes())
		.collect()
}
