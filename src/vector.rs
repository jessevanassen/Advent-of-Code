#[derive(
	Debug,
	Default,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	derive_more::Add,
	derive_more::AddAssign,
	derive_more::Sub,
	derive_more::SubAssign,
)]
pub struct Vector3D {
	pub x: i64,
	pub y: i64,
	pub z: i64,
}

impl From<Vector3D> for [i64; 3] {
	fn from(value: Vector3D) -> Self {
		[value.x, value.y, value.z]
	}
}
