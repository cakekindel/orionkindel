use crate::convert;

pub struct TimeDelta(pub f64);
convert!(impl From<f64> for newtype TimeDelta {});
convert!(impl Deref<f64> for newtype TimeDelta {});
