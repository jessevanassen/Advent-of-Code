use std::{io::stdin, iter};

fn main() {
	let mut lines = stdin().lines().flatten();
	let monkeys = iter::from_fn(move || {
		let monkey = parse_monkey(&mut lines);
		lines.next(); // Consume separator, if exists
		monkey
	})
	.collect::<Vec<_>>();

	let keep_worry_manageable_strategy = |x| x / 3;
	let part1 = solve(&monkeys, 20, keep_worry_manageable_strategy);
	println!("Part 1: {part1}");

	let smallest_common_product: u64 = monkeys
		.iter()
		.map(|m| m.test.divisible_by)
		.product();
	let keep_worry_manageable_strategy = |x| x % smallest_common_product;
	let part2 = solve(&monkeys, 10000, keep_worry_manageable_strategy);
	println!("Part 2: {part2}");
}

fn solve(
	monkeys: &[Monkey],
	iterations: usize,
	keep_worry_manageable_strategy: impl Fn(u64) -> u64,
) -> usize {
	let mut monkeys = monkeys.to_owned();
	let mut inspects_acc = vec![0; monkeys.len()];

	for _ in 0..iterations {
		turn(
			&mut monkeys,
			&mut inspects_acc,
			&keep_worry_manageable_strategy,
		);
	}

	inspects_acc.sort();
	inspects_acc
		.iter()
		.rev()
		.take(2)
		.product::<usize>()
}

fn turn(
	monkeys: &mut [Monkey],
	inspects_acc: &mut [usize],
	keep_worry_manageable_strategy: &impl Fn(u64) -> u64,
) {
	for i in 0..monkeys.len() {
		inspects_acc[i] += monkeys[i].items.len();

		while let Some(item) = monkeys[i].items.pop() {
			let new_worry_level = keep_worry_manageable_strategy(monkeys[i].operation.apply(item));
			let to_monkey = monkeys[i].test.apply(new_worry_level);
			monkeys[to_monkey]
				.items
				.push(new_worry_level);
		}
	}
}

fn parse_monkey(iter: &mut impl Iterator<Item = String>) -> Option<Monkey> {
	iter.next()?; // Consume monkey index

	Some(Monkey {
		items: iter.next()?[18..]
			.split(", ")
			.map(|n| n.parse().unwrap())
			.collect(),
		operation: match &iter.next()?[23..] {
			"* old" => Operation::Square,
			mul if mul.starts_with('*') => Operation::MultiplyBy(mul[2..].parse().unwrap()),
			add if add.starts_with('+') => Operation::Add(add[2..].parse().unwrap()),
			other => panic!("Invalid operation {other}"),
		},
		test: Test {
			divisible_by: iter.next()?[21..].parse().unwrap(),
			if_true: iter.next()?[29..].parse().unwrap(),
			if_false: iter.next()?[30..].parse().unwrap(),
		},
	})
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
	items: Vec<u64>,
	operation: Operation,
	test: Test,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Test {
	divisible_by: u64,
	if_true: usize,
	if_false: usize,
}

impl Test {
	fn apply(&self, input: u64) -> usize {
		match input % self.divisible_by == 0 {
			true => self.if_true,
			false => self.if_false,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
	Square,
	MultiplyBy(u64),
	Add(u64),
}

impl Operation {
	fn apply(&self, input: u64) -> u64 {
		use Operation::*;
		match self {
			Square => input * input,
			MultiplyBy(n) => input * n,
			Add(n) => input + n,
		}
	}
}
