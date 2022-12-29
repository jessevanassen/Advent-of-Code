use std::io::stdin;

type Dependencies = [Option<u32>; 26];

fn main() {
	let dependencies = parse_dependencies(stdin().lines().flatten());
	println!("Part 1: {}", build_order(&dependencies));
	println!("Part 2: {}", build_time(&dependencies));
}

fn build_order(dependencies: &Dependencies) -> String {
	let mut result = String::with_capacity(dependencies.len());
	let value_count = dependencies.iter().flatten().count();
	let mut built: u32 = 0;

	'outer: while (built.count_ones() as usize) < value_count {
		for v in (0u8..26)
			.filter(|v| dependencies[*v as usize].is_some())
			// Not already built
			.filter(|v| built & 1 << v == 0)
			// Dependencies satisfied
			.filter(|v| {
				let deps = dependencies[*v as usize].unwrap();
				built & deps == deps
			}) {
			built |= 1 << v;
			result.push((v + b'A') as char);

			continue 'outer;
		}

		panic!("No modifications, but queue is not empty");
	}

	result
}

fn build_time(dependencies: &Dependencies) -> usize {
	const WORKERS: usize = 5;
	const LEAD_TIME: usize = 60;

	struct Task {
		task: u8,
		time_remaining: usize,
	}
	type Worker = Option<Task>;

	let mut workers: [Worker; WORKERS] = Default::default();

	let value_count = dependencies.iter().flatten().count();
	let mut built: u32 = 0;
	let mut time = 0;

	loop {
		/* First, process all the workers so workers that are done are available again, and the
		 * build resources are updated. */
		for worker in workers
			.iter_mut()
			.filter(|w| w.is_some())
		{
			let task = worker.as_mut().unwrap();

			if task.time_remaining == 0 {
				built |= 1 << task.task;
				*worker = None;
			} else {
				task.time_remaining -= 1;
			}
		}

		// If all tasks are now finished, we're done
		if built.count_ones() as usize == value_count {
			break;
		}

		// Otherwise, allocate remaining tasks over available workers
		let mut tasks_available = (0u8..26)
			.filter(|v| dependencies[*v as usize].is_some())
			// Not already built
			.filter(|v| built & 1 << v == 0)
			// Dependencies satisfied
			.filter(|v| {
				let deps = dependencies[*v as usize].unwrap();
				built & deps == deps
			})
			.filter(|v| !workers.iter().any(|w| matches!(w, Some(t) if t.task == *v)))
			.collect::<Vec<_>>()
			.into_iter();

		for i in 0..workers.len() {
			if workers[i].is_some() {
				continue;
			}

			if let Some(task) = tasks_available.next() {
				workers[i] = Some(Task {
					task,
					time_remaining: LEAD_TIME + task as usize,
				});
			} else {
				break;
			}
		}

		time += 1;
	}

	time
}

fn parse_dependencies(lines: impl IntoIterator<Item = impl AsRef<str>>) -> Dependencies {
	let mut dependencies: Dependencies = Default::default();

	for (item, dependency) in lines
		.into_iter()
		.map(|line| parse_dependency(line.as_ref()))
	{
		let item = item as usize;

		let mask = 1 << dependency;
		if let Some(dependencies) = dependencies[item].as_mut() {
			*dependencies |= mask;
		} else {
			dependencies[item] = Some(mask);
		}

		if dependencies[dependency as usize].is_none() {
			dependencies[dependency as usize] = Some(0);
		}
	}

	dependencies
}

fn parse_dependency(line: &str) -> (u8, u8) {
	let line = line.as_bytes();
	let dependency = line[5];
	let item = line[36];
	(item - b'A', dependency - b'A')
}
