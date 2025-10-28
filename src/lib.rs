//! An algorithmically faster implementation of variadic GCD.

#![no_std]
#![doc(html_root_url = "https://docs.rs/gcdn")]
#![crate_name = "gcdn"]
#![warn(
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_import_braces,
    clippy::shadow_unrelated
)]
#![deny(missing_docs, unsafe_op_in_unsafe_fn)]

/// Some traits needed for generic implementation.
pub mod util;
use util::*;

/// GCD of 2 arguments.
pub fn gcd2<T, U>(a: T, b: T) -> U
where
    T: UAbs<U>,
    U: PrimInt + WrappingShr + WrappingSub,
{
    let mut a = uabs(a);
    let mut b = uabs(b);
    let s = min2(expot(a), expot(b));
    (a, b) = sort2(unpot(a), unpot(b));
    while b > U::one() {
        (a, b) = sort2(unpot(a - b), b)
    }
    if b == U::one() {
        a = U::one()
    }
    a * s
}

/// GCD of 3 arguments.
pub fn gcd3<T, U>(a: T, b: T, c: T) -> U
where
    T: UAbs<U>,
    U: PrimInt + WrappingShr + WrappingSub,
{
    let mut a = uabs(a);
    let mut b = uabs(b);
    let mut c = uabs(c);
    let s = min3(expot(a), expot(b), expot(c));
    (a, b, c) = sort3(unpot(a), unpot(b), unpot(c));
    while c > U::one() {
        (a, b, c) = sort3(unpot(a - b), unpot(b - c), c)
    }
    if c == U::one() {
        b = U::one()
    }
    while b > U::one() {
        (a, b) = sort2(unpot(a - b), b)
    }
    if b == U::one() {
        a = U::one()
    }
    a * s
}

/// GCD of 4 arguments.
pub fn gcd4<T, U>(a: T, b: T, c: T, d: T) -> U
where
    T: UAbs<U>,
    U: PrimInt + WrappingShr + WrappingSub,
{
    let mut a = uabs(a);
    let mut b = uabs(b);
    let mut c = uabs(c);
    let mut d = uabs(d);
    let s = min4(expot(a), expot(b), expot(c), expot(d));
    (a, b, c, d) = sort4(unpot(a), unpot(b), unpot(c), unpot(d));
    while d > U::one() {
        (a, b, c, d) = sort4(unpot(a - b), unpot(b - c), unpot(c - d), d);
    }
    if d == U::one() {
        c = U::one()
    }
    while c > U::one() {
        (a, b, c) = sort3(unpot(a - b), unpot(b - c), c);
    }
    if c == U::one() {
        b = U::one()
    }
    while b > U::one() {
        (a, b) = sort2(unpot(a - b), b);
    }
    if b == U::one() {
        a = U::one()
    }
    a * s
}

/// GCD of N arguments.
pub fn gcdn<T, U>(vec: &mut [T]) -> U
where
    T: PrimInt + WrappingSub + WrappingShr + UAbs<U>,
    U: PrimInt + WrappingSub + WrappingShr,
{
    if vec.is_empty() {
        return U::one();
    }
    if vec.len() == 1 {
        return uabs(vec[0]);
    }
    let mut or = vec[0];
    for x in vec.iter() {
        // gcd is 1 if any number is 1
        if *x == T::one() {
            return U::one();
        }
        // two's complement allows this to work with negative numbers
        or = or | *x;
    }
    let s = expot(uabs(or));
    for x in vec.iter_mut() {
        *x = iabs(unpot(uabs(*x)));
        // x has only factors of two, so the gcd must be a power of two
        // 1 << s is the largest power of two it can be
        if *x == T::one() {
            return s;
        }
    }
    loop {
        vec.sort_by(|a, b| b.cmp(a));
        let mut prev = 0;
        for i in 1..vec.len() {
            let x = vec[i];
            if x == T::zero() {
                return uabs(vec[prev]) * s;
            }
            let x = vec[prev].wrapping_sub(&x);
            let x = x.wrapping_shr(x.trailing_zeros());
            vec[prev] = x;
            prev = i;
        }
    }
}

/// LCM of 2 arguments.
pub fn lcm2<T, U>(a: T, b: T) -> U
where
    T: UAbs<U>,
    U: PrimInt + WrappingShr + WrappingSub,
{
    let mut a = uabs(a);
    let mut b = uabs(b);
    let s = max2(expot(a), expot(b));
    (a, b) = sort2(unpot(a), unpot(b));
    let lcm = a * b * s;
    while b > U::one() {
        (a, b) = sort2(unpot(a - b), b);
    }
    if b == U::one() || a <= U::one() {
        lcm
    } else {
        lcm / a
    }
}

/// LCM of 3 arguments.
pub fn lcm3<T, U>(a: T, b: T, c: T) -> U
where
    T: UAbs<U>,
    U: PrimInt + WrappingShr + WrappingSub,
{
    // TODO: better implementation
    lcm2(iabs(lcm2(a, b)), c)
}

/// LCM of 4 arguments.
pub fn lcm4<T, U>(a: T, b: T, c: T, d: T) -> U
where
    T: UAbs<U>,
    U: PrimInt + WrappingShr + WrappingSub,
{
    // TODO: better implementation
    lcm2(iabs(lcm2(iabs(lcm2(a, b)), c)), d)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_gcd2() {
        assert_eq!(gcd2(5, 3), 1u32);
        assert_eq!(gcd2(3, 3), 3u32);
        assert_eq!(gcd2(5, 25), 5u32);
        assert_eq!(gcd2(4, 2), 2u32);
        assert_eq!(gcd2(4, 8), 4u32);
        assert_eq!(gcd2(0, 25), 25u32);
        assert_eq!(gcd2(4, 0), 4u32);
    }

    #[test]
    fn test_gcd3() {
        assert_eq!(gcd3(5, 4, 3), 1u32);
        assert_eq!(gcd3(5, 3, 3), 1u32);
        assert_eq!(gcd3(7, 14, 28), 7u32);
        assert_eq!(gcd3(3, 3, 3), 3u32);
        assert_eq!(gcd3(120, 5, 25), 5u32);
        assert_eq!(gcd3(4, 4, 2), 2u32);
        assert_eq!(gcd3(16, 4, 8), 4u32);
        assert_eq!(gcd3(0, 5, 25), 5u32);
        assert_eq!(gcd3(4, 0, 2), 2u32);
        assert_eq!(gcd3(16, 4, 0), 4u32);
    }

    #[test]
    fn test_gcd4() {
        assert_eq!(gcd4(4, 5, 4, 3), 1u32);
        assert_eq!(gcd4(5, 5, 3, 3), 1u32);
        assert_eq!(gcd4(21, 7, 14, 28), 7u32);
        assert_eq!(gcd4(3, 3, 3, 3), 3u32);
        assert_eq!(gcd4(15, 120, 30, 25), 5u32);
        assert_eq!(gcd4(4, 4, 4, 2), 2u32);
        assert_eq!(gcd4(4, 16, 4, 8), 4u32);
        assert_eq!(gcd4(0, 3, 3, 3), 3u32);
        assert_eq!(gcd4(15, 0, 30, 25), 5u32);
        assert_eq!(gcd4(4, 4, 0, 2), 2u32);
        assert_eq!(gcd4(4, 16, 4, 0), 4u32);
    }

    #[test]
    fn test_gcdn() {
        assert_eq!(gcdn(&mut [0]), 0u32);
        assert_eq!(gcdn(&mut [3]), 3u32);
        assert_eq!(gcdn(&mut [4, 3]), 1u32);
        assert_eq!(gcdn(&mut [5, 4, 3]), 1u32);
        assert_eq!(gcdn(&mut [4, 5, 4, 3]), 1u32);
        assert_eq!(gcdn(&mut [5, 5, 3, 3]), 1u32);
        assert_eq!(gcdn(&mut [21, 7, 14, 28]), 7u32);
        assert_eq!(gcdn(&mut [3, 3, 3, 3]), 3u32);
        assert_eq!(gcdn(&mut [15, 120, 30, 25]), 5u32);
        assert_eq!(gcdn(&mut [4, 4, 4, 2]), 2u32);
        assert_eq!(gcdn(&mut [4, 16, 4, 8]), 4u32);
        assert_eq!(gcdn(&mut [4, 16, 4, 8, 0]), 4u32);
        assert_eq!(gcdn(&mut [4, 16, 4, 0, 8]), 4u32);
        assert_eq!(gcdn(&mut [4, 16, 0, 4, 8]), 4u32);
        assert_eq!(gcdn(&mut [4, 0, 16, 4, 8]), 4u32);
        assert_eq!(gcdn(&mut [0, 4, 16, 4, 8]), 4u32);
    }

    #[test]
    fn test_lcm2() {
        assert_eq!(lcm2(5, 3), 15u32);
        assert_eq!(lcm2(3, 3), 3u32);
        assert_eq!(lcm2(5, 25), 25u32);
        assert_eq!(lcm2(4, 2), 4u32);
        assert_eq!(lcm2(6, 8), 24u32);
    }

    #[test]
    fn test_lcm3() {
        assert_eq!(lcm3(5, 4, 3), 60u32);
        assert_eq!(lcm3(5, 3, 3), 15u32);
        assert_eq!(lcm3(7, 14, 28), 28u32);
        assert_eq!(lcm3(3, 3, 3), 3u32);
        assert_eq!(lcm3(120, 5, 25), 600u32);
        assert_eq!(lcm3(4, 4, 2), 4u32);
        assert_eq!(lcm3(16, 4, 8), 16u32);
    }

    #[test]
    fn test_lcm4() {
        assert_eq!(lcm4(4, 5, 4, 3), 60u32);
        assert_eq!(lcm4(5, 5, 3, 3), 15u32);
        assert_eq!(lcm4(21, 7, 14, 28), 84u32);
        assert_eq!(lcm4(3, 3, 3, 3), 3u32);
        assert_eq!(lcm4(15, 120, 30, 25), 600u32);
        assert_eq!(lcm4(4, 4, 4, 2), 4u32);
        assert_eq!(lcm4(4, 16, 4, 8), 16u32);
    }
}
