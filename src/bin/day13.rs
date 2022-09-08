use std::{io::stdin, str::FromStr, collections::HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
	x: i32,
	y: i32,
}

impl FromStr for Point {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (x, y) = s.split_once(',').ok_or(())?;
		let x = x.parse().map_err(|_| ())?;
		let y = y.parse().map_err(|_| ())?;
		Ok(Point { x, y })
	}
}

#[derive(Debug, PartialEq)]
enum Fold {
	Horizontal(i32),
	Vertical(i32),
}

impl FromStr for Fold {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (direction, coordinate) = s[11..].split_once('=').ok_or(())?;
		let coordinate: i32 = coordinate.parse().map_err(|_| ())?;
		let fold = if direction == "x" { Self::Horizontal(coordinate) } else { Self::Vertical(coordinate) };
		Ok(fold)
	}
}

fn apply_fold<'a>(points: impl Iterator<Item = &'a Point>, fold: &Fold) -> HashSet<Point> {
	match fold {
		Fold::Horizontal(x) => {
			let (mut before_fold, after_fold): (HashSet<Point>, HashSet<Point>) = points
				.partition(|&p| p.x <= *x)
				;
			let folded = after_fold.into_iter()
				.map(|p| Point { x: (p.x - x) * -1 + x, ..p })
				;
			before_fold.extend(folded);
			before_fold
		},
		Fold::Vertical(y) => {
			let (mut before_fold, after_fold): (HashSet<Point>, HashSet<Point>) = points
				.partition(|&p| p.y <= *y)
				;
			let folded = after_fold.into_iter()
				.map(|p| Point { y: (p.y - y) * -1 + y, ..p })
				;
			before_fold.extend(folded);
			before_fold
		},
	}
}

fn parse_input() -> (Vec<Point>, Vec<Fold>) {
	let mut lines = stdin().lines().flatten();
	let mut points: Vec<Point> = Vec::default();
	let mut folds: Vec<Fold> = Vec::default();

	while let Some(line) = lines.next() {
		if line.is_empty() {
			break;
		}

		if let Ok(point) = line.parse() {
			points.push(point);
		}
	}

	while let Some(line) = lines.next() {
		if let Ok(fold) = line.parse() {
			folds.push(fold);
		}
	}

	(points, folds)
}

fn print(points: &HashSet<Point>) -> String {
	let max_x = points.iter().map(|p| p.x).max().unwrap();
	let max_y = points.iter().map(|p| p.y).max().unwrap();

	let mut out = String::new();
	for y in 0..=max_y {
		for x in 0..=max_x {
			let c = if points.contains(&Point { x, y }) { '#' } else { ' ' };
			out.push(c);
		}
		out.push('\n');
	}
	out
}

fn main() {
	let (points, folds) = parse_input();
	let points: HashSet<_> = points.into_iter().collect();
	println!("Part 1: {}", apply_fold(points.iter(), &folds[0]).len());

	let f = folds.into_iter()
		.fold(points, |acc, fold| apply_fold(acc.iter(), &fold))
		;
	print!("Part 2:\n{}", print(&f));
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_fold() {
		assert_eq!(Ok(Fold::Horizontal(5)), "fold along x=5".parse());
		assert_eq!(Ok(Fold::Vertical(7)),   "fold along y=7".parse());
	}
}
