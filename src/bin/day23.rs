use std::{
	collections::{HashMap, HashSet},
	io::stdin,
};

use aoc2023::{
	grid2d::{Grid2D, Grid2DGet},
	vector2d::Vector2D,
};

type Map = Grid2D<Square>;
type DistanceGraph = Vec<Vec<(usize, u64)>>;

fn main() {
	let input = stdin()
		.lines()
		.flatten()
		.map(|line| {
			line.bytes()
				.map(|b| b.try_into().unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Map>();

	{
		let graph = to_distance_graph(&input);
		println!("Part 1: {}", find_longest_path(&graph));
	}

	{
		let mut input = input.clone();
		for value in input.values_mut() {
			if matches!(value, Square::Directional(_)) {
				*value = Square::Path;
			}
		}
		let graph = to_distance_graph(&input);
		println!("Part 2: {}", find_longest_path(&graph));
	}
}

fn to_distance_graph(map: &Map) -> DistanceGraph {
	let mut nodes = vec![start(map)];
	nodes.extend(find_nodes(map));
	nodes.push(destination(map));

	let node_mapping = nodes.iter().copied().zip(0..).collect::<HashMap<_, _>>();

	fn find_connected_nodes(
		map: &Grid2D<Square>,
		node_mapping: &HashMap<Vector2D, usize>,
		start: Vector2D,
	) -> Vec<(usize, u64)> {
		let mut seen: HashSet<Vector2D> = HashSet::new();
		seen.insert(start);

		let mut connections = Vec::new();
		let mut queue: Vec<(Vector2D, u64)> =
			accessible_around(map, start).map(|p| (p, 1)).collect();

		while let Some((position, distance)) = queue.pop() {
			if seen.contains(&position) {
				continue;
			}

			seen.insert(position);

			if let Some(i) = node_mapping.get(&position) {
				connections.push((*i, distance));
			} else {
				let next = accessible_around(map, position).map(|p| (p, distance + 1));
				queue.extend(next);
			}
		}

		connections
	}

	nodes
		.iter()
		.map(|&start| find_connected_nodes(map, &node_mapping, start))
		.collect()
}

fn around(map: &Grid2D<Square>, position: Vector2D) -> impl Iterator<Item = Vector2D> + '_ {
	DIRECTIONS
		.iter()
		.map(move |&direction| position + direction)
		.filter(move |&position| map.get(position).is_some_and(|&s| s != Square::Forest))
}

fn accessible_around(
	map: &Grid2D<Square>,
	position: Vector2D,
) -> impl Iterator<Item = Vector2D> + '_ {
	DIRECTIONS.iter().filter_map(move |&direction| {
		let position = position + direction;
		map.get(position)
			.is_some_and(|s| s.is_accessible(direction))
			.then_some(position)
	})
}

/// Nodes are places in the Map that are connected by 3 or more edges.
fn find_nodes(map: &Map) -> impl Iterator<Item = Vector2D> + '_ {
	map.enumerate()
		.filter(|(_, v)| **v != Square::Forest)
		.map(|(i, _)| Vector2D::from(i))
		.filter(|&position| around(map, position).count() > 2)
}

fn start(_: &Grid2D<Square>) -> Vector2D {
	Vector2D(1, 0)
}

fn destination(map: &Grid2D<Square>) -> Vector2D {
	Vector2D::from((map.width() - 2, map.height() - 1))
}

fn find_longest_path(distances: &DistanceGraph) -> u64 {
	fn find_paths(
		mut seen: u64,
		start: usize,
		distance: u64,
		distances: &DistanceGraph,
	) -> Option<u64> {
		if start == distances.len() - 1 {
			return Some(distance);
		}

		seen |= 1 << start;

		distances[start]
			.iter()
			.filter(|(i, _)| seen & (1 << i) == 0)
			.flat_map(|&(i, d)| find_paths(seen, i, distance + d, distances))
			.max()
	}

	find_paths(0, 0, 0, distances).into_iter().max().unwrap()
}

const NORTH: Vector2D = Vector2D(0, -1);
const SOUTH: Vector2D = Vector2D(0, 1);
const EAST: Vector2D = Vector2D(1, 0);
const WEST: Vector2D = Vector2D(-1, 0);
static DIRECTIONS: [Vector2D; 4] = [NORTH, EAST, SOUTH, WEST];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
	Path,
	Forest,
	Directional(Direction),
}

impl Square {
	fn is_accessible(self, direction: Vector2D) -> bool {
		match self {
			Square::Path => true,
			Square::Forest => false,
			Square::Directional(d) => direction == d.into(),
		}
	}
}

impl TryFrom<u8> for Square {
	type Error = anyhow::Error;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		Ok(match value {
			b'.' => Self::Path,
			b'#' => Self::Forest,
			b'^' => Self::Directional(Direction::North),
			b'v' => Self::Directional(Direction::South),
			b'>' => Self::Directional(Direction::East),
			b'<' => Self::Directional(Direction::West),
			_ => anyhow::bail!("Unknown character '{}'", value as char),
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
	North,
	South,
	East,
	West,
}

impl From<Direction> for Vector2D {
	fn from(value: Direction) -> Self {
		match value {
			Direction::North => NORTH,
			Direction::South => SOUTH,
			Direction::East => EAST,
			Direction::West => WEST,
		}
	}
}
