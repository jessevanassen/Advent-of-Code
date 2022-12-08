pub struct TakeWhileInclusive<I, P> {
	iter: I,
	predicate: P,
	done: bool,
}

pub trait TakeWhileInclusiveExt<I: Iterator> {
	fn take_while_inclusive<P>(self, predicate: P) -> TakeWhileInclusive<I, P>
	where
		P: Fn(&I::Item) -> bool;
}

impl<I: Iterator> TakeWhileInclusiveExt<I> for I {
	fn take_while_inclusive<P>(self, predicate: P) -> TakeWhileInclusive<I, P>
	where
		P: Fn(&I::Item) -> bool,
	{
		TakeWhileInclusive {
			iter: self,
			predicate,
			done: false,
		}
	}
}

impl<I, P> Iterator for TakeWhileInclusive<I, P>
where
	I: Iterator,
	P: Fn(&I::Item) -> bool,
{
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		if self.done {
			return None;
		}

		match self.iter.next() {
			None => {
				self.done = true;
				None
			}
			Some(value) => {
				if !(self.predicate)(&value) {
					self.done = true;
				}
				Some(value)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_take_everything() {
		let input = vec![1, 2, 3, 4, 5];
		let actual = input
			.iter()
			.copied()
			.take_while_inclusive(|_| true)
			.collect::<Vec<_>>();
		assert_eq!(input, actual);
	}

	#[test]
	fn test_take_some() {
		let input = vec![1, 2, 3, 1, 2, 3];
		let actual = input
			.iter()
			.copied()
			.take_while_inclusive(|&x| x < 3)
			.collect::<Vec<_>>();
		assert_eq!(vec![1, 2, 3], actual);
	}

	#[test]
	fn test_predicate_does_not_match_only_takes_first_item() {
		let input = vec![1, 2, 3, 4, 5];
		let actual = input
			.iter()
			.copied()
			.take_while_inclusive(|_| false)
			.collect::<Vec<_>>();
		assert_eq!(vec![1], actual);
	}
}
