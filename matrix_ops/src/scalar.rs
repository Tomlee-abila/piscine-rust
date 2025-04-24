use std::ops::{Add, Sub, Mul, Div};

/// Scalar trait: defines operations required for scalar values
pub trait Scalar: 
    Copy + Clone + PartialEq + Add<Output = Self> + Sub<Output = Self> + 
    Mul<Output = Self> + Div<Output = Self> 
{
    fn zero() -> Self;
    fn one() -> Self;
}

// Implement Scalar for all required numeric types
impl Scalar for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for u64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}
