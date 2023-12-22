use std::{io::stdin, ops::Add, str::FromStr};

fn main() {
	let mut bricks = stdin()
		.lines()
		.flatten()
		.map(|line| Brick::from_str(&line).unwrap())
		.collect::<Vec<_>>();
	bricks.sort_by_key(|b| b.bottom());

	fall_all(&mut bricks);
	bricks.sort_by_key(|b| b.top());

	let (part1, part2) = (0..bricks.len())
		.map(|i| {
			let mut bricks = bricks.clone();
			bricks.remove(i);

			fall_all(&mut bricks)
		})
		.fold((0, 0), |(mut count, mut sum), it| {
			if it == 0 {
				count += 1;
			}
			sum += it;
			(count, sum)
		});
	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}

fn can_fall_for(brick: Brick, bricks: &[Brick]) -> i64 {
	bricks
		.iter()
		.rev()
		.filter_map(|b| {
			let distance = brick.bottom() - b.top();
			let brick = brick + Vector3D::from_z(-distance);
			b.collides_with(&brick).then_some(distance - 1)
		})
		.min()
		.unwrap_or(brick.bottom() - 1)
}

fn fall(bricks: &mut [Brick], index: usize) -> bool {
	let distance = can_fall_for(bricks[index], &bricks[..index]);

	if distance > 0 {
		bricks[index] = bricks[index] + Vector3D::from_z(-distance);
		true
	} else {
		false
	}
}

fn fall_all(bricks: &mut [Brick]) -> usize {
	let mut count = 0;
	for i in 0..bricks.len() {
		if fall(bricks, i) {
			count += 1;
		}
	}
	count
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector3D {
	x: i64,
	y: i64,
	z: i64,
}

impl Vector3D {
	fn from_z(z: i64) -> Self {
		Vector3D { x: 0, y: 0, z }
	}
}

impl Add for Vector3D {
	type Output = Vector3D;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

impl FromStr for Vector3D {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut iter = s.split(',').map(|s| s.parse::<i64>());
		let mut next = || {
			iter.next()
				.ok_or_else(|| anyhow::anyhow!("Not enough components"))
		};

		Ok(Self {
			x: next()??,
			y: next()??,
			z: next()??,
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
	front_bottom_left: Vector3D,
	rear_top_right: Vector3D,
}

impl Brick {
	fn top(&self) -> i64 {
		self.rear_top_right.z
	}

	fn bottom(&self) -> i64 {
		self.front_bottom_left.z
	}

	fn collides_with(&self, other: &Brick) -> bool {
		!(self.front_bottom_left.x > other.rear_top_right.x
			|| self.rear_top_right.x < other.front_bottom_left.x
			|| self.front_bottom_left.y > other.rear_top_right.y
			|| self.rear_top_right.y < other.front_bottom_left.y
			|| self.front_bottom_left.z > other.rear_top_right.z
			|| self.rear_top_right.z < other.front_bottom_left.z)
	}
}

impl FromStr for Brick {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let components = s
			.split_once('~')
			.ok_or_else(|| anyhow::anyhow!("Not enough components"))?;
		Ok(Brick {
			front_bottom_left: components.0.parse()?,
			rear_top_right: components.1.parse()?,
		})
	}
}

impl std::ops::Add<Vector3D> for Brick {
	type Output = Brick;

	fn add(self, rhs: Vector3D) -> Self::Output {
		Self {
			front_bottom_left: self.front_bottom_left + rhs,
			rear_top_right: self.rear_top_right + rhs,
		}
	}
}
