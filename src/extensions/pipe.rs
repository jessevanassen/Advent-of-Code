pub trait Pipe<T> {
	fn map_self<U>(self, fun: impl FnOnce(T) -> U) -> U;
	fn apply_to_self(self, fun: impl FnOnce(&mut T)) -> T;
}

impl<T> Pipe<T> for T {
	fn map_self<U>(self, fun: impl FnOnce(T) -> U) -> U {
		fun(self)
	}

	fn apply_to_self(mut self, fun: impl FnOnce(&mut T)) -> T {
		fun(&mut self);
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_map_self() {
		let x = vec![10, 11, 12];
		let y = x.map_self(|x| x.len());

		assert_eq!(y, 3);

		// Ownership transferred, `x` is no longer accessible
	}

	#[test]
	fn test_map_self_on_reference() {
		let x = vec![10, 11, 12];
		let y = (&x).map_self(|x| x.len());

		assert_eq!(y, 3);

		// Ownership did not transfer, `x` is still accessible
		assert_eq!(x.len(), 3);
	}

	#[test]
	fn test_apply_to_self() {
		let x = vec![12, 11, 10];
		let y = x.apply_to_self(|x| x.sort());

		assert_eq!(y, vec![10, 11, 12]);

		// Ownership transferred, `x` is no longer accessible
	}
}
