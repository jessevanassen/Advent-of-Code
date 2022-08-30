use std::io::stdin;

fn main() {
	let positions = get_positions();
	let min = *positions.iter().min().unwrap();
	let max = *positions.iter().max().unwrap();

	let min_cost = |cost_fn: fn(i32) -> i32| {
		(min..=max)
			.map(|destination| positions.iter()
				.map(|origin| (origin - destination).abs())
				.map(cost_fn)
				.fold(0, |x, y| x + y)
			)
			.min().unwrap()
	};

	println!("Part 1: {:?}", min_cost(linear));
	println!("Part 2: {:?}", min_cost(triangular));
}

fn linear(n: i32) -> i32 { n }
fn triangular(n: i32) -> i32 { (n * (n + 1)) / 2 }

fn get_positions() -> Vec<i32> {
	stdin().lines().next().unwrap().unwrap()
		.split(',')
		.map(|x| x.parse().unwrap())
		.collect()
}

#[cfg(test)]
mod test {
    use crate::triangular;

	#[test]
	fn test_triangular() {
		assert_eq!(0,  triangular(0));
		assert_eq!(1,  triangular(1));
		assert_eq!(3,  triangular(2));
		assert_eq!(6,  triangular(3));
		assert_eq!(10, triangular(4));
	}
}
