use std::{cell::Cell, io::stdin};

use aoc2024::{Grid, Vector2D};

type Map = Grid<Option<Item>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
	Wall,
	Box,
	BoxLeft,
	BoxRight,
}

fn main() {
	let Input {
		map,
		robot_position,
		directions,
	} = parse_input();

	{
		let mut map = map.clone();
		let mut robot_position = robot_position;

		for &direction in directions.iter() {
			if do_move(&mut map, robot_position, direction) {
				robot_position += direction;
			}
		}

		println!("Part 1: {}", calculate_gps_sum(&map));
	}

	{
		let mut map = expand_map(&map);
		let mut robot_position = robot_position * Vector2D { x: 2, y: 1 };

		for &direction in directions.iter() {
			/* Some boxes might already have moved before we realize we cannot
			 * move other boxes.
			 * So, take a clone of the map, and only if all the boxes were able
			 * to move, the modification is made final. */
			let mut map_clone = map.clone();
			if do_move(&mut map_clone, robot_position, direction) {
				robot_position += direction;
				map = map_clone;
			}
		}

		println!("Part 2: {}", calculate_gps_sum(&map));
	}
}

fn do_move(map: &mut Map, from: Vector2D, direction: Vector2D) -> bool {
	let to = from + direction;
	let vertical = direction.y != 0;
	match (vertical, map[from], map[to]) {
		(_, None, None) => true,
		(_, Some(Item::Wall), _) | (_, _, Some(Item::Wall)) => false,
		(_, Some(Item::Box | Item::BoxLeft | Item::BoxRight), None) => {
			map.swap(from, to);
			true
		}
		(false, _, Some(Item::Box | Item::BoxLeft | Item::BoxRight)) => {
			if do_move(map, to, direction) {
				do_move(map, from, direction)
			} else {
				false
			}
		}
		(true, _, Some(Item::Box)) => {
			if do_move(map, to, direction) {
				do_move(map, from, direction)
			} else {
				false
			}
		}
		(true, _, Some(item @ (Item::BoxLeft | Item::BoxRight))) => {
			let other = match item {
				Item::BoxLeft => to + Vector2D { x: 1, y: 0 },
				Item::BoxRight => to - Vector2D { x: 1, y: 0 },
				_ => unreachable!(),
			};
			if do_move(map, to, direction) && do_move(map, other, direction) {
				do_move(map, from, direction)
			} else {
				false
			}
		}
	}
}

fn expand_map(map: &Map) -> Map {
	let mut result = Map::new(map.width() * 2, map.height(), None);

	for (i, v) in map.enumerate() {
		let i = i * Vector2D { x: 2, y: 1 };
		let j = i + Vector2D { x: 1, y: 0 };

		match v {
			Some(Item::Box) => {
				result[i] = Some(Item::BoxLeft);
				result[j] = Some(Item::BoxRight);
			}
			Some(Item::Wall) => {
				result[i] = Some(Item::Wall);
				result[j] = Some(Item::Wall);
			}
			None => { /* Nothing to do, prefilled with None */ }
			Some(Item::BoxLeft | Item::BoxRight) => {
				panic!("Cannot expand already expanded map");
			}
		}
	}

	result
}

fn calculate_gps_sum(map: &Map) -> i64 {
	map.enumerate()
		.filter_map(|(i, v)| matches!(v, Some(Item::Box | Item::BoxLeft)).then_some(i))
		.map(|position| position.x + position.y * 100)
		.sum::<i64>()
}

struct Input {
	map: Map,
	robot_position: Vector2D,
	directions: Vec<Vector2D>,
}

fn parse_input() -> Input {
	let mut stdin = stdin().lines().map(Result::unwrap);

	/* Misuse a Cell to let the value escape the closure */
	let robot_position = &Cell::new(None);

	let map: Map = (&mut stdin)
		.take_while(|line| !line.is_empty())
		.enumerate()
		.map(|(y, line)| {
			line.into_bytes()
				.into_iter()
				.enumerate()
				.map(move |(x, b)| match b {
					b'@' => {
						robot_position.set(Some(Vector2D {
							x: x as _,
							y: y as _,
						}));
						None
					}
					b'.' => None,
					b'#' => Some(Item::Wall),
					b'O' => Some(Item::Box),
					other => {
						panic!("Input error: unexpected {}", other as char);
					}
				})
		})
		.collect();

	let directions = stdin
		.flat_map(|line| {
			line.into_bytes().into_iter().map(|b| match b {
				b'^' => Vector2D { x: 0, y: -1 },
				b'>' => Vector2D { x: 1, y: 0 },
				b'v' => Vector2D { x: 0, y: 1 },
				b'<' => Vector2D { x: -1, y: 0 },
				other => {
					panic!("Input error: unexpected {}", other as char);
				}
			})
		})
		.collect();

	Input {
		robot_position: robot_position.get().expect("Position not set"),
		map,
		directions,
	}
}

#[allow(unused)]
fn render_map(map: &Map, robot: Vector2D) -> String {
	let mut output = String::with_capacity(((map.width() + 1) * map.height()) as usize);

	for y in 0..map.height() {
		for x in 0..map.width() {
			let position = Vector2D::new(x as _, y as _);
			output.push(match (position == robot, map[position]) {
				(true, _) => '@',
				(false, None) => '.',
				(false, Some(Item::Box)) => 'O',
				(false, Some(Item::BoxLeft)) => '[',
				(false, Some(Item::BoxRight)) => ']',
				(false, Some(Item::Wall)) => '#',
			});
		}
		output.push('\n');
	}

	output.pop();
	output
}
