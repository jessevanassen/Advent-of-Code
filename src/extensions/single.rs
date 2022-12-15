use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum SingleError {
	#[error("Expected one result, but got none")]
	None,
	#[error("Expected one result, but got more")]
	MoreThanOne,
}

pub trait SingleExt<T> {
	fn single(self) -> Result<T, SingleError>;
}

impl<Iter: Iterator> SingleExt<Iter::Item> for Iter {
	fn single(mut self) -> Result<Iter::Item, SingleError> {
		let value = self.next().ok_or(SingleError::None)?;
		if self.next().is_none() {
			Ok(value)
		} else {
			Err(SingleError::MoreThanOne)
		}
	}
}
