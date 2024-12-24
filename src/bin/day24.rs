use std::{
	collections::{BTreeMap, BTreeSet},
	io::{stdin, Read},
};

use clap::Parser;

#[derive(clap::Parser)]
enum Arguments {
	/// Evaluates to part 1's result
	Part1,
	/// Find suspicious parts of the graph
	Sus,
	/// Prints a Graphviz program
	Graphviz,
}

fn main() {
	let (expressions, identifiers, initial_arguments) = parse_input();

	match Arguments::parse() {
		Arguments::Part1 => {
			let part1 = evaluate(&expressions, &identifiers, initial_arguments);
			println!("Part1: {part1}");
		}
		Arguments::Graphviz => {
			println!("digraph {{");
			for (index, identifier) in identifiers.iter().enumerate() {
				match expressions[index] {
					Node::Value { .. } => {
						println!("\t{identifier} [ style=\"filled\"; color=\"lightblue\" ];");
					},
					Node::Operation { lhs, operation, rhs } => {
						let (operation, color) = match operation {
							Operation::And => ("AND", "green"),
							Operation::Or => ("OR", "purple"),
							Operation::Xor => ("XOR", "orange"),
						};
						println!("\t{identifier} [ label=\"{identifier}: {operation}\"; color=\"{color}\"; penwidth=2.0; ];");

						if identifier.starts_with('z') {
							println!("\t{identifier} [ style=\"filled,dashed\"; fillcolor=\"pink\"; penwidth=4.0; ];");
						}

						println!("\t{identifier} -> {{ {}, {} }};", identifiers[lhs], identifiers[rhs]);
					},
				}
			}
			println!("}}");
		}
		Arguments::Sus => {
			let mut sus = BTreeSet::new();

			for i in 0..44 {
				let input = 1 << i;

				if evaluate(&expressions, &identifiers, [input, 0]) != input {
					sus.insert(format!("z{:02}", i));
				}

				if evaluate(&expressions, &identifiers, [0, input]) != input {
					sus.insert(format!("z{:02}", i));
				}

				if evaluate(&expressions, &identifiers, [input, input]) != input * 2 {
					sus.insert(format!("z{:02}", i + 1));
				}
			}

			for s in sus {
				println!("{s}");
			}
		}
	}
}

fn evaluate(expressions: &[Node], identifiers: &[String], arguments: [u64; 2]) -> u64 {
	identifiers
		.iter()
		.enumerate()
		.filter_map(|(index, name)| name.starts_with('z').then_some(index))
		.map(|index| evaluate_node(expressions, index, arguments))
		.rfold(0u64, |acc, bit| (acc << 1) | bit as u64)
}

fn evaluate_node(expressions: &[Node], index: usize, arguments: [u64; 2]) -> bool {
	match expressions[index] {
		Node::Value { index, bit } => arguments[index] & (1 << bit) != 0,
		Node::Operation { lhs, operation, rhs } => {
			let lhs = evaluate_node(expressions, lhs, arguments);
			let rhs = evaluate_node(expressions, rhs, arguments);
			operation.eval(lhs, rhs)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
	Value {
		index: usize,
		bit: usize,
	},
	Operation {
		lhs: usize,
		operation: Operation,
		rhs: usize,
	},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
	And,
	Or,
	Xor,
}

impl Operation {
	fn eval(self, lhs: bool, rhs: bool) -> bool {
		match self {
			Operation::And => lhs & rhs,
			Operation::Or  => lhs | rhs,
			Operation::Xor => lhs ^ rhs,
		}
	}
}

fn parse_input() -> (Vec<Node>, Vec<String>, [u64; 2]) {
	let input = {
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		buf
	};

	let identifiers = {
		let mut identifiers = BTreeSet::new();
		let mut iter = input.lines();

		identifiers.extend(
			(&mut iter)
				.take_while(|line| !line.is_empty())
				.map(|line| line.split_once(':').expect("Expect initial value").0),
		);
		identifiers.extend(iter.map(|line| line.split_ascii_whitespace().last().unwrap()));

		identifiers
			.into_iter()
			.enumerate()
			.map(|(index, key)| (key, index))
			.collect::<BTreeMap<_, _>>()
	};

	let mut expressions = vec![None; identifiers.len()];

	let mut iter = input.lines();

	let mut arguments = [0u64; 2];
	for line in (&mut iter).take_while(|line| !line.is_empty()) {
		let (identifier, value) = line.split_once(": ").expect("Expect initial value");

		let argument_index = match identifier.as_bytes()[0] {
			b @ (b'x' | b'y') => (b - b'x') as usize,
			other => panic!("Unexpected parameter {}", other as char),
		};

		let bit_index: usize = identifier[1..].parse().expect("Expected bit index");

		expressions[identifiers[identifier]] = Some(Node::Value {
			index: argument_index,
			bit: bit_index,
		});

		if value == "1" {
			arguments[argument_index] |= 1 << bit_index;
		}
	}

	for line in iter {
		let mut parts = line.split_whitespace();
		let lhs = parts.next().expect("Expect lhs");
		let operator = parts.next().expect("Expect operator");
		let rhs = parts.next().expect("Expect rhs");
		let _ = parts.next();
		let target = parts.next().expect("Expect target");

		let lhs = identifiers[lhs];
		let rhs = identifiers[rhs];
		let target = identifiers[target];

		let operation = match operator {
			"AND" => Operation::And,
			"OR" => Operation::Or,
			"XOR" => Operation::Xor,
			_ => panic!("Unsupported operator {operator}"),
		};

		expressions[target] = Some(Node::Operation { lhs, operation, rhs });
	}

	let expressions = expressions
		.into_iter()
		.map(|node| node.expect("Expect expression"))
		.collect();
	let identifiers = identifiers.into_keys().map(String::from).collect();

	(expressions, identifiers, arguments)
}
