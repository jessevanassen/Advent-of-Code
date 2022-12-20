use std::{
	collections::HashSet,
	fmt::Display,
	io::stdin,
	ops::{Add, Deref, Index, IndexMut, Sub},
	str::FromStr,
};

use aoc2022::triangle_number;
use regex::Regex;

const STARTING_PRODUCTION: Balance = Balance([1, 0, 0, 0]);

fn main() -> anyhow::Result<()> {
	let blueprints = stdin()
		.lines()
		.flatten()
		.map(|ref line| line.parse())
		.collect::<anyhow::Result<Vec<_>>>()?;

	let part1: usize = blueprints
		.iter()
		.map(|blueprint| solve(24, blueprint) as usize)
		.enumerate()
		.map(|(i, x)| (i + 1) * x)
		.sum();
	println!("Part 1: {part1}");

	let part2: usize = blueprints
		.iter()
		.take(3)
		.map(|blueprint| solve(32, blueprint) as usize)
		.product();
	println!("Part 2: {part2}");

	Ok(())
}

fn solve(limit: usize, blueprint: &Blueprint) -> usize {
	let mut max_geodes = 0;
	let mut stack = vec![(
		limit,
		Balance::default(),
		STARTING_PRODUCTION,
		ResourceSet::ALL,
	)];
	let mut seen = HashSet::new();

	while let Some(entry @ (time_remaining, balance, production, options)) = stack.pop() {
		seen.insert(entry);

		if time_remaining == 0 {
			max_geodes = max_geodes.max(balance[Resource::Geode]);
			continue;
		}

		/* If we can't hypothetically produce more geodes than the known maximum
		 * before the time is up, prune the branch. */
		if (balance[Resource::Geode] as usize
			+ production[Resource::Geode] as usize * time_remaining
			/* The hypothetical case where from now on (regardless of resources),
			 * we would only be creating an additional geode robot every turn,
			 * which each would produce one geode per second. */
			+ triangle_number(time_remaining - 1))
			< max_geodes as usize
		{
			continue;
		}

		let mut schedule_next = |balance, new_production, possibilities| {
			let next = (
				time_remaining - 1,
				/* We don't want to use the new_production for the new balance
				 * just yet, because the added robot in new_production won't
				 * produce until the next turn. */
				&balance + &production,
				new_production,
				possibilities,
			);
			if !seen.contains(&next) {
				stack.push(next);
			}
		};

		for resource in options.iter() {
			if balance.can_buy(&blueprint[resource])
				/* No robots require geodes as resource, which means we would
				 * always produce enough geodes to make a robot, which means that
				 * without explicitly allowing geodes, a geode robot would never
				 * be built. */
				&& (resource == Resource::Geode
					|| blueprint.iter().any(|robot_cost| robot_cost[resource] > production[resource]))
			{
				let balance = &balance - &blueprint[resource];

				let mut production = production;
				production[resource] += 1;

				schedule_next(balance, production, ResourceSet::ALL);
			}
		}

		let mut next_options = options;

		for option in options.iter() {
			if balance.can_buy(&blueprint[option]) {
				/* If we can buy a specific robot but decide to wait for
				 * resources anyway, then don't buy this robot the next turn.
				 * Otherwise, we would have wasted a turn waiting for nothing. */
				next_options.remove(option);
			}
		}
		if !next_options.is_empty() {
			schedule_next(balance, production, next_options);
		}
	}

	max_geodes as _
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
	Ore = 0,
	Clay = 1,
	Obsidian = 2,
	Geode = 3,
}

impl TryFrom<isize> for Resource {
	type Error = ();

	fn try_from(value: isize) -> Result<Self, Self::Error> {
		use Resource::*;
		Ok(match value {
			v if v == Ore as isize => Ore,
			v if v == Clay as isize => Clay,
			v if v == Obsidian as isize => Obsidian,
			v if v == Geode as isize => Geode,
			_ => return Err(()),
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceSet(u8);

impl ResourceSet {
	const ALL: ResourceSet = ResourceSet(0b1111);

	pub fn iter(&self) -> impl Iterator<Item = Resource> + '_ {
		(0..4)
			.rev()
			.map(|i| i.try_into().unwrap())
			.filter(|&r| self.contains(r))
	}

	pub fn contains(&self, resource: Resource) -> bool {
		(self.0 & (1 << resource as u8)) > 0
	}

	pub fn insert(&mut self, resource: Resource) {
		self.0 |= 1 << resource as u8;
	}

	pub fn remove(&mut self, resource: Resource) {
		self.0 &= !(1 << resource as u8);
	}

	pub fn is_empty(&self) -> bool {
		self.0 == 0
	}
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Balance([i32; 4]);

impl From<[i32; 4]> for Balance {
	fn from(value: [i32; 4]) -> Self {
		Balance(value)
	}
}

impl Add for &Balance {
	type Output = Balance;

	fn add(self, rhs: Self) -> Self::Output {
		Balance([
			self.0[0] + rhs.0[0],
			self.0[1] + rhs.0[1],
			self.0[2] + rhs.0[2],
			self.0[3] + rhs.0[3],
		])
	}
}

impl Sub for &Balance {
	type Output = Balance;

	fn sub(self, rhs: Self) -> Self::Output {
		Balance([
			self.0[0] - rhs.0[0],
			self.0[1] - rhs.0[1],
			self.0[2] - rhs.0[2],
			self.0[3] - rhs.0[3],
		])
	}
}

impl Display for Balance {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Geode: {}, Obsidian: {}, Clay: {}, Ore: {}",
			self[Resource::Geode],
			self[Resource::Obsidian],
			self[Resource::Clay],
			self[Resource::Ore]
		)
	}
}

impl Index<Resource> for Balance {
	type Output = i32;

	fn index(&self, index: Resource) -> &Self::Output {
		self.0.index(index as usize)
	}
}
impl IndexMut<Resource> for Balance {
	fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
		self.0.index_mut(index as usize)
	}
}

impl Balance {
	pub fn buy(&self, other: &Balance) -> Option<Balance> {
		let diff = self - other;
		let not_negative = diff.0.into_iter().all(|v| v >= 0);
		not_negative.then_some(diff)
	}

	pub fn can_buy(&self, other: &Balance) -> bool {
		self.buy(other).is_some()
	}
}

struct Blueprint([Balance; 4]);

impl Deref for Blueprint {
	type Target = [Balance; 4];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Index<Resource> for Blueprint {
	type Output = Balance;

	fn index(&self, index: Resource) -> &Self::Output {
		&self.0[index as usize]
	}
}

impl FromStr for Blueprint {
	type Err = anyhow::Error;

	fn from_str(line: &str) -> Result<Blueprint, Self::Err> {
		lazy_static::lazy_static! {
			static ref RE: Regex = Regex::new(r#"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."#).unwrap();
		}

		let captures = RE
			.captures(line)
			.ok_or_else(|| anyhow::anyhow!("No match"))?;
		let mut captures = captures
			.iter()
			.skip(1)
			.map(|c| c.unwrap().as_str().parse::<i32>());

		let mut take = || {
			captures
				.next()
				.ok_or_else(|| anyhow::anyhow!("Too few matches"))
				.unwrap()
		};

		take()?;

		Ok(Blueprint([
			[take()?, 0, 0, 0].into(),
			[take()?, 0, 0, 0].into(),
			[take()?, take()?, 0, 0].into(),
			[take()?, 0, take()?, 0].into(),
		]))
	}
}
