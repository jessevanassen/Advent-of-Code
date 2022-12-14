/// Takes an `Option<Result<T, E>>` and propagates it if it is either `None` or
/// `Some(Err(_))`. \
/// If the input is `Some(Ok(value))`, `value` is yielded from the block.
///
/// This works as an addition to the `?` operator, which works on `Option` or
/// `Result`, but not on `Option<Result<_>>`.
#[macro_export]
macro_rules! propagate {
	($expr: expr) => {
		match $expr {
			None => return None,
			Some(Err(err)) => return Some(Err(err)),
			Some(Ok(value)) => value,
		}
	};
}
