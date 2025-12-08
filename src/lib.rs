pub mod grid;
pub mod vector;

pub fn minmax<T: Ord>(v1: T, v2: T) -> [T; 2] {
	if v2 < v1 { [v2, v1] } else { [v1, v2] }
}
