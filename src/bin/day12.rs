use std::{collections::HashMap, io::stdin};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringCondition {
	Operational,
	Damaged,
	Unknown,
}

impl TryFrom<u8> for SpringCondition {
	type Error = anyhow::Error;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		Ok(match value {
			b'.' => Self::Operational,
			b'#' => Self::Damaged,
			b'?' => Self::Unknown,
			_ => anyhow::bail!("Invalid spring condition '{}'", value),
		})
	}
}

fn main() {
	let input = stdin()
		.lines()
		.flatten()
		.map(|line| {
			let (report, damaged_groups) = line.split_once(' ').unwrap();
			let report = report
				.bytes()
				.map(|c| SpringCondition::try_from(c).unwrap())
				.collect::<Vec<_>>();
			let damaged_groups = damaged_groups
				.split(',')
				.map(|n| n.parse::<usize>().unwrap())
				.collect::<Vec<_>>();
			(report, damaged_groups)
		})
		.collect::<Vec<_>>();

	println!(
		"Part 1: {}",
		input
			.iter()
			.map(|(pattern, counts)| count_possible_arrangements(pattern, counts))
			.sum::<usize>()
	);
	println!(
		"Part 2: {}",
		input
			.iter()
			.map(|(pattern, counts)| {
				let (pattern, counts) = unwrap(pattern, counts);
				count_possible_arrangements(&pattern, &counts)
			})
			.sum::<usize>()
	);
}

fn count_possible_arrangements(report: &[SpringCondition], damaged_groups: &[usize]) -> usize {
	fn count_possible_arrangements(
		report: &[SpringCondition],
		damaged_groups: &[usize],
		cache: &mut HashMap<(usize, usize), usize>,
	) -> usize {
		let cache_key = (report.len(), damaged_groups.len());
		if let Some(result) = cache.get(&cache_key) {
			return *result;
		}

		if damaged_groups.is_empty() {
			/* If we don't expect any more damaged groups, the rest of the springs shouldn't be
			 * damaged. */
			return if report.iter().all(|s| *s != SpringCondition::Damaged) {
				1
			} else {
				0
			};
		}

		if report.is_empty() {
			/* Report is empty, but more damaged springs are expected. */
			return 0;
		}

		if report.len() < damaged_groups.iter().sum::<usize>() + (damaged_groups.len() - 1) {
			/* The report's length should at least be the total sum of damaged springs, plus the
			 * separators between the groups. */
			return 0;
		}

		let match_operational = |cache| {
			/* A group of operational springs can be considered like a single operational spring, so the whole
			 * group can be skipped. */
			let first_non_operational = report
				.iter()
				.skip(1)
				.position(|s| *s != SpringCondition::Operational)
				.map(|x| x + 1);
			let rest = first_non_operational
				.map(|start| &report[start..])
				.unwrap_or_default();
			count_possible_arrangements(rest, damaged_groups, cache)
		};

		let match_damaged = |cache| {
			let count = *damaged_groups.first().unwrap();

			if report.len() < count {
				/* Not enough springs remaining in the report to satisfy the count */
				return 0;
			}

			if report[..count]
				.iter()
				.any(|s| *s == SpringCondition::Operational)
			{
				/* The group of (possible) damaged springs contains a operational one */
				return 0;
			}

			if report
				.get(count)
				.is_some_and(|s| *s == SpringCondition::Damaged)
			{
				/* The group of (possible) damaged springs is not terminated by either a
				 * operational spring or the end of the report */
				return 0;
			}

			/* Skip the remaining damaged springs, and the separator after it (if it exists) */
			let rest = report.get(count + 1..).unwrap_or_default();
			count_possible_arrangements(rest, damaged_groups.get(1..).unwrap_or_default(), cache)
		};

		let result = match report[0] {
			SpringCondition::Operational => match_operational(cache),
			SpringCondition::Damaged => match_damaged(cache),
			SpringCondition::Unknown => match_operational(cache) + match_damaged(cache),
		};

		cache.insert(cache_key, result);

		result
	}

	let mut cache = HashMap::new();
	count_possible_arrangements(report, damaged_groups, &mut cache)
}

fn unwrap(pattern: &[SpringCondition], counts: &[usize]) -> (Vec<SpringCondition>, Vec<usize>) {
	let mut new_pattern = Vec::with_capacity(pattern.len() * 5 + 4);
	let mut new_counts = Vec::with_capacity(counts.len() * 5);

	for i in 0..5 {
		if i != 0 {
			new_pattern.push(SpringCondition::Unknown);
		}
		new_pattern.extend(pattern);

		new_counts.extend(counts);
	}

	(new_pattern, new_counts)
}
