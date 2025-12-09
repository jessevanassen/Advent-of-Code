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
pub struct Vector2D {
	pub x: i64,
	pub y: i64,
}

impl From<Vector2D> for [i64; 2] {
	fn from(value: Vector2D) -> Self {
		[value.x, value.y]
	}
}

impl From<[i64; 2]> for Vector2D {
	fn from([x, y]: [i64; 2]) -> Self {
		Self { x, y }
	}
}

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
