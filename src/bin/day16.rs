use std::{
	cmp::Ordering,
	collections::{BinaryHeap, HashMap, HashSet},
	io::stdin,
};

use aoc2024::{Grid, Vector2D};
use itertools::Itertools;

type Map = Grid<bool>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entity {
	position: Vector2D,
	direction: Vector2D,
}

fn main() {
	let map = parse_input();
	let start = Entity {
		position: Vector2D {
			x: 1,
			y: map.height() as i64 - 2,
		},
		direction: Vector2D { x: 1, y: 0 },
	};
	let target = Vector2D {
		x: map.width() as i64 - 2,
		y: 1,
	};

	let path = find_path(start, &map);

	let target_entity = path
		.iter()
		.filter(|(k, _)| k.position == target)
		.min_by_key(|(_, PathEntry { cost, .. })| cost)
		.map(|(k, _)| k)
		.expect("No path to the target");

	println!("Part 1: {}", path[target_entity].cost);
	println!("Part 2: {}", count_positions(&path, target_entity));
}

type Path = HashMap<Entity, PathEntry>;
struct PathEntry {
	cost: u64,
	from: Vec<Entity>,
}

/// Finds the shortest path from the start entity to any entity (the HashMap's
/// key) in the Map.
fn find_path(start: Entity, map: &Map) -> Path {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	struct HeapKey {
		cost: u64,
		from: Entity,
		to: Entity,
	}

	impl Ord for HeapKey {
		fn cmp(&self, other: &Self) -> Ordering {
			let ordering = self.cost.cmp(&other.cost);
			/* BinaryHeap is a Max-Heap, but a Min-Heap is required. */
			ordering.reverse()
		}
	}

	impl PartialOrd for HeapKey {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			Some(self.cmp(other))
		}
	}

	let mut path = Path::new();

	let mut todo = BinaryHeap::<HeapKey>::new();
	todo.push(HeapKey {
		cost: 0u64,
		from: start,
		to: start,
	});

	while let Some(HeapKey { cost, from, to }) = todo.pop() {
		if let Some(entry) = path.get_mut(&to) {
			/* Node has already been visited before. */
			match (entry.cost).cmp(&cost) {
				Ordering::Less => { /* Existing path is shorter, skip */ }
				Ordering::Equal => {
					/* Both paths cost the same, merge */
					entry.from.push(from);
				}
				Ordering::Greater => {
					/* Current path is cheaper, replace. */
					/* Note: this path should never happen, because a MaxHeap is
					 * used so shorter paths shouldn't be encountered. */
					entry.from.clear();
					entry.from.push(from);
				}
			}
		} else {
			path.insert(
				to,
				PathEntry {
					cost,
					from: vec![from],
				},
			);
			todo.extend(next_moves(to, map).map(|(entity, c)| HeapKey {
				cost: cost + c,
				from: to,
				to: entity,
			}));
		}
	}

	path
}

/// Counts the unique positions in the possible shortests paths to the target.
fn count_positions(path: &Path, target: &Entity) -> usize {
	let mut seen = HashSet::<&Entity>::new();
	let mut queue = vec![target];

	while let Some(entity) = queue.pop() {
		if seen.insert(entity) {
			queue.extend(path[entity].from.iter());
		}
	}

	seen.into_iter()
		.map(|entity| entity.position)
		.unique()
		.count()
}

fn next_moves(entity: Entity, map: &Map) -> impl Iterator<Item = (Entity, u64)> + '_ {
	let r#move = Entity {
		position: entity.position + entity.direction,
		..entity
	};
	let rotated_cw = Entity {
		direction: entity.direction.rotate_cw(),
		..entity
	};
	let rotated_ccw = Entity {
		direction: entity.direction.rotate_ccw(),
		..entity
	};

	[(r#move, 1), (rotated_cw, 1000), (rotated_ccw, 1000)]
		.into_iter()
		.filter(|(entity, _)| map.get(entity.position) == Some(&true))
}

fn parse_input() -> Map {
	stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| line.into_bytes().into_iter().map(|b| b != b'#'))
		.collect()
}
