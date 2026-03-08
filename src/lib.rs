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
    let s = min2(two_part(a), two_part(b));
    (a, b) = sort2(odd(a), odd(b));
    while b > U::one() {
        (a, b) = sort2(odd(a - b), b)
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
    let s = min3(two_part(a), two_part(b), two_part(c));
    (a, b, c) = sort3(odd(a), odd(b), odd(c));
    while c > U::one() {
        (a, b, c) = sort3(odd(a - b), odd(b - c), c)
    }
    if c == U::one() {
        b = U::one()
    }
    while b > U::one() {
        (a, b) = sort2(odd(a - b), b)
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
    let s = min4(two_part(a), two_part(b), two_part(c), two_part(d));
    (a, b, c, d) = sort4(odd(a), odd(b), odd(c), odd(d));
    while d > U::one() {
        (a, b, c, d) = sort4(odd(a - b), odd(b - c), odd(c - d), d);
    }
    if d == U::one() {
        c = U::one()
    }
    while c > U::one() {
        (a, b, c) = sort3(odd(a - b), odd(b - c), c);
    }
    if c == U::one() {
        b = U::one()
    }
    while b > U::one() {
        (a, b) = sort2(odd(a - b), b);
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
    match vec.len() {
        0 => return U::one(),
        1 => return uabs(vec[0]),
        2 => return gcd2(vec[0], vec[1]),
        3 => return gcd3(vec[0], vec[1], vec[2]),
        4 => return gcd4(vec[0], vec[1], vec[2], vec[3]),
        _ => {}
    }
    let mut or = vec[0];
    for x in vec.iter() {
        if *x == T::one() {
            return U::one();
        }
        or = or | *x;
    }
    let s = two_part(uabs(or));
    for x in vec.iter_mut() {
        *x = iabs(odd(uabs(*x)));
        if *x == T::one() {
            return s;
        }
    }
    vec.sort_by(|a, b| b.cmp(a));
    for k in (2..=vec.len()).rev() {
        // SAFETY: k is in 2..=vec.len(), so k-1 and k-2 are valid indices.
        // prev and i are in 0..k, so within bounds.
        unsafe {
            loop {
                if uabs(*vec.get_unchecked(k - 1)) <= U::one() {
                    break;
                }
                let mut prev = 0;
                for i in 1..k {
                    let x = vec.get_unchecked(prev).wrapping_sub(vec.get_unchecked(i));
                    let x = x.wrapping_shr(x.trailing_zeros());
                    *vec.get_unchecked_mut(prev) = x;
                    prev = i;
                }
                vec[..k].sort_unstable_by(|a, b| b.cmp(a));
            }
            if *vec.get_unchecked(k - 1) == T::one() {
                *vec.get_unchecked_mut(k - 2) = T::one();
            }
        }
    }
    uabs(*unsafe { vec.get_unchecked(0) }) * s
}

/// LCM of 2 arguments.
pub fn lcm2<T, U>(a: T, b: T) -> U
where
    T: UAbs<U>,
    U: PrimInt + WrappingShr + WrappingSub,
{
    let mut a = uabs(a);
    let mut b = uabs(b);
    let s = max2(two_part(a), two_part(b));
    (a, b) = sort2(odd(a), odd(b));
    let lcm = a * b * s;
    while b > U::one() {
        (a, b) = sort2(odd(a - b), b);
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
    lcm2(iabs(lcm2(a, b)), c)
}

/// LCM of 4 arguments.
pub fn lcm4<T, U>(a: T, b: T, c: T, d: T) -> U
where
    T: UAbs<U>,
    U: PrimInt + WrappingShr + WrappingSub,
{
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
