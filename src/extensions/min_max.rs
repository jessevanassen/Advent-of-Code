pub trait MinMaxExt<T> {
	fn min_max(self) -> Option<(T, T)>;
}

impl<Iter> MinMaxExt<Iter::Item> for Iter
where
	Iter: Iterator,
	Iter::Item: Ord + Copy,
{
	fn min_max(self) -> Option<(Iter::Item, Iter::Item)> {
		self.fold(None, |acc, it| match acc {
			None => Some((it, it)),
			Some(acc) => Some((acc.0.min(it), acc.1.max(it))),
		})
	}
}
