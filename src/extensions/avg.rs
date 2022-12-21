pub trait AvgExt {
	fn avg(self, other: Self) -> Self;
}

macro_rules! impl_AvgExt_for_integer {
	($($type:ty),+) => {
		$(
			impl super::AvgExt for $type {
				fn avg(self, other: Self) -> Self {
					(self / 2) + (other / 2) + (self % 2 + other % 2) / 2
				}
			}
		)+
	};
}
impl_AvgExt_for_integer!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

macro_rules! impl_AvgExt_for_float {
	($($type:ty),+) => {
		$(
			impl super::AvgExt for $type {
				fn avg(self, other: Self) -> Self {
					(self / 2.0) + (other / 2.0)
				}
			}
		)+
	};
}
impl_AvgExt_for_float!(f32, f64);
