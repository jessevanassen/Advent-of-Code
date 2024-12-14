use std::{borrow::Borrow, cmp::Ordering, io::stdin};

use aoc2024::{Grid, Vector2D};

// const WIDTH: i64 = 11;
// const HEIGHT: i64 = 7;
const WIDTH: usize = 101;
const HEIGHT: usize = 103;

fn main() {
	let entities = parse_input().collect::<Vec<_>>();

	let part1 = entities
		.iter()
		.copied()
		.map(|entity| entity.step(100))
		.fold([0usize; 4], |mut acc, entity| {
			let Vector2D { x, y } = entity.position;
			if let Some(quadrant) = match (x.cmp(&(WIDTH as i64 / 2)), y.cmp(&(HEIGHT as i64 / 2)))
			{
				(Ordering::Less, Ordering::Less) => Some(0),
				(Ordering::Less, Ordering::Greater) => Some(1),
				(Ordering::Greater, Ordering::Less) => Some(2),
				(Ordering::Greater, Ordering::Greater) => Some(3),
				_ => None,
			} {
				acc[quadrant] += 1;
			}
			acc
		})
		.into_iter()
		.product::<usize>();
	println!("Part 1: {part1}");

	for i in 1.. {
		if let Some(easter_egg) = display_easter_egg(i, &entities) {
			println!("Part 2: {i}\n{easter_egg}");
			break;
		}
	}
}

fn display_easter_egg(i: usize, entities: &[Entity]) -> Option<String> {
	let mut positions = Grid::new(WIDTH as _, HEIGHT as _, false);
	for entity in entities {
		let position = entity.step(i as _).position;
		positions[position] = true;
	}

	/* Most of the robots form themselves into a picture, which means a lot of
	 * robots should be touchting each other at that time. Use this as a
	 * heuristic: if at least half of the robots are touching another robot, it
	 * is a match and a picture should be drawn. */
	let touching = positions
		.enumerate()
		.filter_map(|(index, value)| value.then_some(index))
		.filter(|&index| positions.neighbors(index).any(|index| positions[index]))
		.count();

	if touching < entities.len() / 2 {
		return None;
	}

	let mut output = String::with_capacity(WIDTH * (HEIGHT + 1) + 1);
	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			output.push(if positions[Vector2D::new(x as _, y as _)] {
				'X'
			} else {
				' '
			});
		}
		output.push('\n');
	}
	Some(output)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Entity {
	position: Vector2D,
	velocity: Vector2D,
}

impl Entity {
	pub fn step(self, n: u64) -> Self {
		Self {
			position: Vector2D {
				x: (self.position.x + self.velocity.x * n as i64).rem_euclid(WIDTH as _),
				y: (self.position.y + self.velocity.y * n as i64).rem_euclid(HEIGHT as _),
			},
			..self
		}
	}
}

fn parse_input() -> impl Iterator<Item = Entity> {
	stdin().lines().map(Result::unwrap).map(parse_entity)
}

fn parse_entity(input: impl Borrow<str>) -> Entity {
	use nom::{
		bytes::complete::tag,
		character::complete::{self, space1},
		combinator::map,
		sequence::{preceded, separated_pair},
	};

	type IResult<'a, T> = nom::IResult<&'a str, T>;

	fn parse_vector(input: &str) -> IResult<Vector2D> {
		map(
			separated_pair(complete::i64, tag(","), complete::i64),
			Vector2D::from,
		)(input)
	}

	fn parse_entity(input: &str) -> IResult<Entity> {
		map(
			separated_pair(
				preceded(tag("p="), parse_vector),
				space1,
				preceded(tag("v="), parse_vector),
			),
			|(position, velocity)| Entity { position, velocity },
		)(input)
	}

	parse_entity(input.borrow()).unwrap().1
}
