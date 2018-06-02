use std::cmp::Ordering;

pub trait SemiOrd {
    fn semi_cmp(&self, rhs: &Self) -> Ordering;
}

macro_rules! impl_partial_ord (
    ($x:ty) => {
        impl SemiOrd for $x {
            fn semi_cmp(&self, rhs: &$x) -> Ordering {
                self.partial_cmp(rhs).unwrap_or(Ordering::Less)
            }
        }
    };
);

macro_rules! impl_ord (
    ($x:ty) => {
        impl SemiOrd for $x {
            fn semi_cmp(&self, rhs: &$x) -> Ordering {
                self.cmp(rhs)
            }
        }
    };
);

impl_partial_ord!(f32);
impl_partial_ord!(f64);

impl_ord!(u8);
impl_ord!(u16);
impl_ord!(u32);
impl_ord!(u64);
impl_ord!(i8);
impl_ord!(i16);
impl_ord!(i32);
impl_ord!(i64);
