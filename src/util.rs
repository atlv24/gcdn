use core::cmp::{max, min};

pub use num_traits::{
    NumCast, PrimInt, Signed, ToPrimitive, WrappingShl, WrappingShr, WrappingSub, signum,
};

/// Minimum of 2 arguments
#[inline(always)]
pub fn min2<T>(a: T, b: T) -> T
where
    T: PrimInt,
{
    min(a, b)
}

/// Minimum of 3 arguments
#[inline(always)]
pub fn min3<T>(a: T, b: T, c: T) -> T
where
    T: PrimInt,
{
    min(min(a, b), c)
}

/// Minimum of 4 arguments
#[inline(always)]
pub fn min4<T>(a: T, b: T, c: T, d: T) -> T
where
    T: PrimInt,
{
    min(min(a, b), min(c, d))
}

/// Maximum of 2 arguments
#[inline(always)]
pub fn max2<T>(a: T, b: T) -> T
where
    T: PrimInt,
{
    max(a, b)
}

/// Maximum of 3 arguments
#[inline(always)]
pub fn max3<T>(a: T, b: T, c: T) -> T
where
    T: PrimInt,
{
    max(max(a, b), c)
}

/// Maximum of 4 arguments
#[inline(always)]
pub fn max4<T>(a: T, b: T, c: T, d: T) -> T
where
    T: PrimInt,
{
    max(max(a, b), max(c, d))
}

/// Sorts a tuple of 2
#[inline(always)]
pub fn sort2<T>(a: T, b: T) -> (T, T)
where
    T: PrimInt,
{
    (max(a, b), min(a, b))
}

/// Sorts a tuple of 3
#[inline(always)]
pub fn sort3<T>(a: T, b: T, c: T) -> (T, T, T)
where
    T: PrimInt,
{
    let mx = max3(a, b, c);
    let mn = min3(a, b, c);
    (mx, a ^ b ^ c ^ mn ^ mx, mn)
}

/// Sorts a tuple of 4
#[inline(always)]
pub fn sort4<T>(a: T, b: T, c: T, d: T) -> (T, T, T, T)
where
    T: PrimInt,
{
    let (a, b) = sort2(a, b);
    let (c, d) = sort2(c, d);
    let (a, c) = sort2(a, c);
    let (b, d) = sort2(b, d);
    let (b, c) = sort2(b, c);
    (a, b, c, d)
}

/// Removes all power of two factors
#[inline(always)]
pub fn unpot<T>(a: T) -> T
where
    T: PrimInt + WrappingShr,
{
    a.wrapping_shr(a.trailing_zeros())
}

/// Extracts largest power of two divisor
#[inline(always)]
pub fn expot<T>(a: T) -> T
where
    T: PrimInt + WrappingSub,
{
    if a == T::zero() {
        return T::max_value();
    }
    a & !a.wrapping_sub(&T::one())
}

/// Integer type with signed and unsigned variants of the same bit width.
pub trait UAbs<T> {
    /// Unsigned absolute value of the argument.
    fn uabs(self) -> T;
    /// Signed absolute value of the argument.
    fn iabs(a: T) -> Self;
}

impl UAbs<usize> for isize {
    #[inline(always)]
    fn uabs(self) -> usize {
        self.wrapping_abs() as usize
    }
    #[inline(always)]
    fn iabs(a: usize) -> Self {
        a as Self
    }
}
impl UAbs<u8> for i8 {
    #[inline(always)]
    fn uabs(self) -> u8 {
        self.wrapping_abs() as u8
    }
    #[inline(always)]
    fn iabs(a: u8) -> Self {
        a as Self
    }
}
impl UAbs<u16> for i16 {
    #[inline(always)]
    fn uabs(self) -> u16 {
        self.wrapping_abs() as u16
    }
    #[inline(always)]
    fn iabs(a: u16) -> Self {
        a as Self
    }
}
impl UAbs<u32> for i32 {
    #[inline(always)]
    fn uabs(self) -> u32 {
        self.wrapping_abs() as u32
    }
    #[inline(always)]
    fn iabs(a: u32) -> Self {
        a as Self
    }
}
impl UAbs<u64> for i64 {
    #[inline(always)]
    fn uabs(self) -> u64 {
        self.wrapping_abs() as u64
    }
    #[inline(always)]
    fn iabs(a: u64) -> Self {
        a as Self
    }
}
impl UAbs<u128> for i128 {
    #[inline(always)]
    fn uabs(self) -> u128 {
        self.wrapping_abs() as u128
    }
    #[inline(always)]
    fn iabs(a: u128) -> Self {
        a as Self
    }
}
impl UAbs<usize> for usize {
    #[inline(always)]
    fn uabs(self) -> usize {
        self
    }
    #[inline(always)]
    fn iabs(a: usize) -> Self {
        a
    }
}
impl UAbs<u8> for u8 {
    #[inline(always)]
    fn uabs(self) -> u8 {
        self
    }
    #[inline(always)]
    fn iabs(a: u8) -> Self {
        a
    }
}
impl UAbs<u16> for u16 {
    #[inline(always)]
    fn uabs(self) -> u16 {
        self
    }
    #[inline(always)]
    fn iabs(a: u16) -> Self {
        a
    }
}
impl UAbs<u32> for u32 {
    #[inline(always)]
    fn uabs(self) -> u32 {
        self
    }
    #[inline(always)]
    fn iabs(a: u32) -> Self {
        a
    }
}
impl UAbs<u64> for u64 {
    #[inline(always)]
    fn uabs(self) -> u64 {
        self
    }
    #[inline(always)]
    fn iabs(a: u64) -> Self {
        a
    }
}
impl UAbs<u128> for u128 {
    #[inline(always)]
    fn uabs(self) -> u128 {
        self
    }
    #[inline(always)]
    fn iabs(a: u128) -> Self {
        a
    }
}

/// Unsigned absolute value of the argument.
#[inline(always)]
pub fn uabs<T, U>(a: T) -> U
where
    T: UAbs<U>,
{
    a.uabs()
}

/// Signed absolute value of the argument.
#[inline(always)]
pub fn iabs<T, U>(a: U) -> T
where
    T: UAbs<U>,
{
    T::iabs(a)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_min2() {
        assert_eq!(min2(0, 0), 0);
        assert_eq!(min2(0, 5), 0);
        assert_eq!(min2(5, 0), 0);
        assert_eq!(min2(1, 1), 1);
        assert_eq!(min2(1, 5), 1);
        assert_eq!(min2(5, 1), 1);
        assert_eq!(min2(-1, 1), -1);
        assert_eq!(min2(-1, 5), -1);
        assert_eq!(min2(5, -1), -1);
    }

    #[test]
    fn test_min3() {
        assert_eq!(min3(0, 0, 0), 0);
        assert_eq!(min3(0, 3, 5), 0);
        assert_eq!(min3(3, 5, 0), 0);
        assert_eq!(min3(-1, 4, 2), -1);
        assert_eq!(min3(7, 4, 2), 2);
    }

    #[test]
    fn test_min4() {
        assert_eq!(min4(0, 0, 0, 0), 0);
        assert_eq!(min4(0, 4, 3, 5), 0);
        assert_eq!(min4(2, 4, 1, 5), 1);
        assert_eq!(min4(-1, 4, 2, -2), -2);
        assert_eq!(min4(7, 4, 8, 5), 4);
    }

    #[test]
    fn test_max2() {
        assert_eq!(max2(0, 0), 0);
        assert_eq!(max2(0, 5), 5);
        assert_eq!(max2(5, 0), 5);
        assert_eq!(max2(1, 1), 1);
        assert_eq!(max2(1, 5), 5);
        assert_eq!(max2(5, 1), 5);
        assert_eq!(max2(-1, 1), 1);
        assert_eq!(max2(-1, 5), 5);
        assert_eq!(max2(5, -1), 5);
    }

    #[test]
    fn test_max3() {
        assert_eq!(max3(0, 0, 0), 0);
        assert_eq!(max3(0, 3, 5), 5);
        assert_eq!(max3(3, 5, 0), 5);
        assert_eq!(max3(-1, 4, 2), 4);
        assert_eq!(max3(7, 4, 2), 7);
    }

    #[test]
    fn test_max4() {
        assert_eq!(max4(0, 0, 0, 0), 0);
        assert_eq!(max4(0, 5, 3, 4), 5);
        assert_eq!(max4(2, 4, 1, 5), 5);
        assert_eq!(max4(-1, 4, 2, -2), 4);
        assert_eq!(max4(7, 4, 8, 5), 8);
    }

    #[test]
    fn test_sort2() {
        assert_eq!(sort2(0, 0), (0, 0));
        assert_eq!(sort2(2, 3), (3, 2));
        assert_eq!(sort2(3, 2), (3, 2));
        assert_eq!(sort2(-3, -2), (-2, -3));
        assert_eq!(sort2(-1, -2), (-1, -2));
    }

    #[test]
    fn test_sort3() {
        assert_eq!(sort3(0, 0, 0), (0, 0, 0));
        assert_eq!(sort3(1, 2, 3), (3, 2, 1));
        assert_eq!(sort3(1, 3, 2), (3, 2, 1));
        assert_eq!(sort3(2, 3, 1), (3, 2, 1));
        assert_eq!(sort3(2, 1, 3), (3, 2, 1));
        assert_eq!(sort3(3, 1, 2), (3, 2, 1));
        assert_eq!(sort3(3, 2, 1), (3, 2, 1));
        assert_eq!(sort3(1, -3, -2), (1, -2, -3));
        assert_eq!(sort3(-1, -1, -2), (-1, -1, -2));
        assert_eq!(sort3(-2, -1, -1), (-1, -1, -2));
    }

    #[test]
    fn test_sort4() {
        assert_eq!(sort4(0, 0, 0, 0), (0, 0, 0, 0));
        assert_eq!(sort4(1, 0, 2, 3), (3, 2, 1, 0));
        assert_eq!(sort4(0, 1, 3, 2), (3, 2, 1, 0));
        assert_eq!(sort4(2, 0, 3, 1), (3, 2, 1, 0));
        assert_eq!(sort4(0, 2, 1, 3), (3, 2, 1, 0));
        assert_eq!(sort4(3, 0, 1, 2), (3, 2, 1, 0));
        assert_eq!(sort4(0, 3, 2, 1), (3, 2, 1, 0));
        assert_eq!(sort4(0, 1, -3, -2), (1, 0, -2, -3));
        assert_eq!(sort4(0, -1, -1, -2), (0, -1, -1, -2));
        assert_eq!(sort4(-2, -1, 0, -1), (0, -1, -1, -2));
    }

    #[test]
    fn test_unpot() {
        assert_eq!(unpot(0), 0);
        assert_eq!(unpot(1), 1);
        assert_eq!(unpot(2), 1);
        assert_eq!(unpot(3), 3);
        assert_eq!(unpot(6), 3);
        assert_eq!(unpot(32), 1);
        assert_eq!(unpot(50), 25);
    }

    #[test]
    fn test_expot() {
        assert_eq!(expot(0), 0);
        assert_eq!(expot(1), 1);
        assert_eq!(expot(2), 2);
        assert_eq!(expot(3), 1);
        assert_eq!(expot(6), 2);
        assert_eq!(expot(32), 32);
        assert_eq!(expot(50), 2);
        assert_eq!(expot(28), 4);
    }

    #[test]
    fn test_uabs() {
        assert_eq!(uabs::<i32, u32>(0), 0);
        assert_eq!(uabs::<i32, u32>(1), 1);
        assert_eq!(uabs::<i32, u32>(-1), 1);
    }

    #[test]
    fn test_iabs() {
        assert_eq!(iabs::<i32, u32>(0u32), 0i32);
        assert_eq!(iabs::<i32, u32>(1u32), 1i32);
    }
}
