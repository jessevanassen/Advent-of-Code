use std::{collections::HashMap, io::stdin, ops::Range, str::FromStr};

use regex::Regex;

fn main() {
	let mut lines = stdin().lines().flatten();

	let (workflows, start_workflow) =
		parse_workflows((&mut lines).take_while(|line| !line.is_empty()));
	let start_workflow = &workflows[start_workflow];

	let parts = parse_parts(lines);

	let part1 = parts
		.filter(|part| {
			let mut rules = start_workflow;
			'outer: loop {
				let result = rules.iter().find_map(|rule| rule.apply(part)).unwrap();
				match result {
					RuleResult::End(v) => return v,
					RuleResult::Jump(label) => {
						rules = &workflows[label];
						continue 'outer;
					}
				}
			}
		})
		.map(|part| part.value())
		.sum::<usize>();
	println!("Part 1: {part1}");

	println!(
		"Part 2: {}",
		find_part_combination_count(&workflows, start_workflow)
	);
}

fn find_part_combination_count(workflows: &Vec<Workflow>, initial_workflow: &Workflow) -> usize {
	fn find_part_combination_count(
		mut pp: PossibleParts,
		workflows: &Vec<Workflow>,
		workflow: &Workflow,
	) -> usize {
		if pp.iter().all(|x| x.is_empty()) {
			return 0;
		}

		let apply_rule_result = |pp: PossibleParts, rule_result: RuleResult| -> usize {
			match rule_result {
				RuleResult::End(false) => 0,
				RuleResult::End(true) => pp.iter().map(|f| f.len()).product::<usize>(),
				RuleResult::Jump(destination) => {
					find_part_combination_count(pp, workflows, &workflows[destination])
				}
			}
		};

		let mut count = 0;

		for rule in workflow.iter() {
			match *rule {
				Rule::Always(rule_result) => {
					count += apply_rule_result(pp, rule_result);
					break;
				}
				Rule::Conditional { condition, result } => {
					let (current, remainder) = condition.split_possible_parts(&pp);
					count += apply_rule_result(current, result);

					pp = remainder;
				}
			}
		}

		count
	}

	find_part_combination_count(
		[1..4001, 1..4001, 1..4001, 1..4001],
		workflows,
		initial_workflow,
	)
}

fn parse_workflows(lines: impl Iterator<Item = String>) -> (Vec<Workflow>, usize) {
	let lines = lines.collect::<Vec<_>>();

	let label_map = lines
		.iter()
		.enumerate()
		.map(|(i, line)| (line.split_once('{').unwrap().0, i))
		.collect::<HashMap<_, _>>();

	let workflows = lines
		.iter()
		.map(|line| {
			let (_, rules) = line.split_once('{').unwrap();
			let rules = &rules[..(rules.len() - 1)];

			rules
				.split(',')
				.map(|rule| {
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

						let operator: Operator = captures.get(2).unwrap().as_str().as_bytes()[0]
							.try_into()
							.unwrap();

						let value = captures.get(3).unwrap().as_str().parse::<u16>().unwrap();

						let destination = parse_destination(captures.get(4).unwrap().as_str());

						Rule::Conditional {
							condition: RuleCondition {
								field,
								value,
								operator,
							},
							result: destination,
						}
					} else {
						let destination = parse_destination(rule);
						Rule::Always(destination)
					}
				})
				.collect()
		})
		.collect();

	let start = label_map["in"];

	(workflows, start)
}

fn parse_parts(lines: impl Iterator<Item = String>) -> impl Iterator<Item = Part> {
	lines.map(|line| line.parse().unwrap())
}

type Workflow = Vec<Rule>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
	GreaterThan,
	LessThan,
}

impl TryFrom<u8> for Operator {
	type Error = anyhow::Error;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		Ok(match value {
			b'<' => Self::LessThan,
			b'>' => Self::GreaterThan,
			_ => anyhow::bail!("Unexpected character '{value}'"),
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RuleCondition {
	field: usize,
	value: u16,
	operator: Operator,
}

impl RuleCondition {
	fn matches(&self, part: &Part) -> bool {
		let RuleCondition {
			field,
			value,
			operator,
		} = *self;

		let field_value = part.0[field];
		match operator {
			Operator::GreaterThan => field_value > value,
			Operator::LessThan => field_value < value,
		}
	}

	fn split_possible_parts(
		&self,
		possible_parts: &PossibleParts,
	) -> (PossibleParts, PossibleParts) {
		let RuleCondition {
			field,
			value,
			operator,
		} = *self;

		match operator {
			Operator::LessThan => {
				let mut head = possible_parts.clone();
				head[field] = head[field].start..value;

				let mut remainder = possible_parts.clone();
				remainder[field] = value..remainder[field].end;

				(head, remainder)
			}
			Operator::GreaterThan => {
				let (remainder, head) = RuleCondition {
					field,
					value: value + 1,
					operator: Operator::LessThan,
				}
				.split_possible_parts(possible_parts);
				(head, remainder)
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule {
	Always(RuleResult),
	Conditional {
		condition: RuleCondition,
		result: RuleResult,
	},
}

impl Rule {
	fn matches(&self, part: &Part) -> bool {
		match self {
			Rule::Always(_) => true,
			Rule::Conditional { condition, .. } => condition.matches(part),
		}
	}

	fn destination(&self) -> RuleResult {
		match self {
			Rule::Always(result) => *result,
			Rule::Conditional { result, .. } => *result,
		}
	}

	fn apply(&self, part: &Part) -> Option<RuleResult> {
		self.matches(part).then(|| self.destination())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleResult {
	End(bool),
	Jump(usize),
}

type PossibleParts = [Range<u16>; 4];

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
