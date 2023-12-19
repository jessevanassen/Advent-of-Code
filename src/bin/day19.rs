use std::{
	collections::HashMap,
	io::{stdin, Read},
	ops::Range,
	str::FromStr,
};

use regex::Regex;

fn main() {
	let input = {
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		buf
	};

	let (rules, parts) = input.split_once("\n\n").unwrap();
	let rules = parse_rules(rules);
	let parts = parse_parts(parts);

	let part1 = parts
		.filter(|&part| {
			let mut rule = rules[0];

			'outer: loop {
				let result = rule.apply(part);
				match result {
					RuleResult::End(v) => return v,
					RuleResult::Jump(i) => {
						rule = rules[i];
						continue 'outer;
					}
				}
			}
		})
		.map(|part| part.value())
		.sum::<usize>();
	println!("Part 1: {part1}");

	println!("Part 2: {}", find_part_combination_count(&rules));
}

fn find_part_combination_count(rules: &[Rule]) -> usize {
	fn find_part_combination_count(pp: PossibleParts, rules: &[Rule], rule: Rule) -> usize {
		if pp.is_empty() {
			return 0;
		}

		let apply_rule_result = |pp: PossibleParts, rule_result: RuleResult| -> usize {
			match rule_result {
				RuleResult::End(false) => 0,
				RuleResult::End(true) => pp.combinations(),
				RuleResult::Jump(destination) => {
					find_part_combination_count(pp, rules, rules[destination])
				}
			}
		};

		match rule {
			Rule::Always(result) => apply_rule_result(pp, result),
			Rule::Conditional {
				condition,
				left,
				right,
			} => pp
				.split(condition)
				.into_iter()
				.zip([left, right])
				.map(|(pp, result)| apply_rule_result(pp, result))
				.sum(),
		}
	}

	find_part_combination_count(PossibleParts::new(), rules, rules[0])
}

fn parse_rules(input: &str) -> Vec<Rule> {
	let mut lines = input.lines().collect::<Vec<_>>();

	/* This makes working with the graph easier, because the starting rule will now always be the
	 * first one in the resulting Vec. */
	let start_position = lines
		.iter()
		.position(|line| line.starts_with("in{"))
		.expect("Expected a start workflow");
	lines.swap(0, start_position);

	/* Map from the rule's label to it's position */
	let label_map = lines
		.iter()
		.scan(0, |acc, line| {
			let start = *acc;

			let (label, rest) = line.split_once('{').unwrap();
			let rule_count = rest.split(',').count();

			*acc += rule_count;

			Some((label, start))
		})
		.collect::<HashMap<_, _>>();

	lines
		.iter()
		.flat_map(|line| {
			let (_, rules) = line.split_once('{').unwrap();
			let rules = &rules[..(rules.len() - 1)];
			rules.split(',')
		})
		.enumerate()
		.map(|(i, rule)| {
			lazy_static::lazy_static! {
				static ref PREDICATE_PATTERN: Regex = Regex::new(r"^([xmas])([<>])(\d+):(\w+)$").unwrap();
			}

			let parse_destination = |destination: &str| -> RuleResult {
				match destination {
					"A" => RuleResult::End(true),
					"R" => RuleResult::End(false),
					label => RuleResult::Jump(label_map[label]),
				}
			};

			if let Some(captures) = PREDICATE_PATTERN.captures(rule) {
				let field = captures.get(1).unwrap().as_str();
				let field = Part::label_offset(field.as_bytes()[0]);

				let split_point = captures.get(3).unwrap().as_str();
				let split_point = split_point.parse::<u16>().unwrap();

				let destination = captures.get(4).unwrap().as_str();
				let destination = parse_destination(destination);

				let operator = captures.get(2).unwrap().as_str();
				let operator = operator.as_bytes()[0];

				let next_rule = RuleResult::Jump(i + 1);
				let (split_point, left, right) = match operator {
					b'<' => (split_point, destination, next_rule),
					b'>' => (split_point + 1, next_rule, destination),
					_ => panic!("Unexpected operator '{operator}'"),
				};

				Rule::Conditional {
					condition: RuleCondition { field, split_point },
					left,
					right,
				}
			} else {
				let destination = parse_destination(rule);
				Rule::Always(destination)
			}
		})
		.collect()
}

fn parse_parts(input: &str) -> impl Iterator<Item = Part> + '_ {
	input.lines().map(|line| line.parse().unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule {
	Always(RuleResult),
	Conditional {
		condition: RuleCondition,
		left: RuleResult,
		right: RuleResult,
	},
}

impl Rule {
	fn apply(&self, part: Part) -> RuleResult {
		match *self {
			Self::Always(result) => result,
			Self::Conditional {
				condition,
				left,
				right,
			} => {
				if condition.evaluate(part) {
					left
				} else {
					right
				}
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RuleCondition {
	field: usize,
	split_point: u16,
}
impl RuleCondition {
	fn evaluate(&self, part: Part) -> bool {
		part.0[self.field] < self.split_point
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleResult {
	End(bool),
	Jump(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PossibleParts([Range<u16>; 4]);

impl PossibleParts {
	fn new() -> Self {
		Self([1..4001, 1..4001, 1..4001, 1..4001])
	}

	fn split(&self, RuleCondition { field, split_point }: RuleCondition) -> [Self; 2] {
		let (mut left, mut right) = (self.0.clone(), self.0.clone());
		left[field] = left[field].start..split_point;
		right[field] = split_point..right[field].end;

		[Self(left), Self(right)]
	}

	fn combinations(&self) -> usize {
		self.0.iter().map(|r| r.len()).product()
	}

	fn is_empty(&self) -> bool {
		self.combinations() == 0
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part([u16; 4]);

impl Part {
	fn value(&self) -> usize {
		self.0.iter().fold(0usize, |acc, &it| acc + it as usize)
	}

	fn label_offset(label: u8) -> usize {
		match label {
			b'x' => 0,
			b'm' => 1,
			b'a' => 2,
			b's' => 3,
			_ => panic!(),
		}
	}
}

impl FromStr for Part {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		lazy_static::lazy_static! {
			static ref PATTERN: Regex = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
		}

		let captures = PATTERN
			.captures(s)
			.ok_or_else(|| anyhow::anyhow!("Invalid input"))?;
		let x = captures.get(1).unwrap().as_str().parse().unwrap();
		let m = captures.get(2).unwrap().as_str().parse().unwrap();
		let a = captures.get(3).unwrap().as_str().parse().unwrap();
		let s = captures.get(4).unwrap().as_str().parse().unwrap();
		Ok(Self([x, m, a, s]))
	}
}
