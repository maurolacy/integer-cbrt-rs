//!
//! This module contains the single trait [`IntegerCubeRoot`] and implements it for primitive
//! integer types.
//!
//! # Example
//!
//! ```
//! extern crate integer_cbrt;
//! // `use` trait to get functionality
//! use integer_cbrt::IntegerCubeRoot;
//!
//! # fn main() {
//! assert_eq!(8u8.integer_cbrt(), 2);
//! # }
//! ```
//!
//! [`IntegerCubeRoot`]: ./trait.IntegerCubeRoot.html
#![no_std]

/// A trait implementing integer cube root.
pub trait IntegerCubeRoot {
    /// Find the integer cube root.
    ///
    /// # Panics
    ///
    /// For negative numbers (`i` family) this function will panic on negative input
    fn integer_cbrt(&self) -> Self
    where
        Self: Sized,
    {
        self.integer_cbrt_checked()
            .expect("cannot calculate cube root of negative number")
    }

    /// Find the integer cube root, returning `None` if the number is negative (this can never
    /// happen for unsigned types).
    fn integer_cbrt_checked(&self) -> Option<Self>
    where
        Self: Sized;
}

impl<T: num_traits::PrimInt> IntegerCubeRoot for T {
    fn integer_cbrt_checked(&self) -> Option<Self> {
        use core::cmp::Ordering;
        match self.cmp(&T::zero()) {
            // Hopefully this will be stripped for unsigned numbers (impossible condition)
            Ordering::Less => return None,
            Ordering::Equal => return Some(T::zero()),
            _ => {}
        }

        // Taken from: https://gist.github.com/anonymous/729557, and generalized to all
        // integer primitive types.
        let one = T::one();
        let three = one + one + one;

        let num_bits = T::zero().leading_zeros();
        let mut x = *self;
        let mut result = T::zero();
        for s in (0..num_bits).step_by(3).rev() {
            result = result + result;
            let b = three * result * (result + one) + one;
            if (x >> s as usize) >= b {
                x = x - (b << s as usize);
                result = result + one;
            }
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::IntegerCubeRoot;
    use core::{i8, u16, u64, u8};

    macro_rules! gen_tests {
        ($($type:ty => $fn_name:ident),*) => {
            $(
                #[test]
                fn $fn_name() {
                    // https://en.wikipedia.org/wiki/Cube_root#Numerical_methods
                    let newton_raphson = |val, cube| 1./3. * (2. * val + (cube / (val as $type * val as $type)) as f64);
                    let max_cbrt = {
                        let cube = <$type>::max_value();
                        let mut value = (cube as f64).cbrt();
                        for _ in 0..2 {
                            value = newton_raphson(value, cube);
                        }
                        let mut value = value as $type;
                        // make sure we are below the max value (this is how integer cube
                        // root works)
                        if value.checked_mul(value*value).is_none() {
                            value -= 1;
                        }
                        value
                    };
                    let tests: [($type, $type); 10] = [
                        (0, 0),
                        (1, 1),
                        (2, 1),
                        (3, 1),
                        (4, 1),
                        (8, 2),
                        (64, 4),
                        (63, 3),
                        (<$type>::max_value(), max_cbrt),
                        (<$type>::max_value() - 1, max_cbrt),
                    ];
                    for &(in_, out) in tests.iter() {
                        assert_eq!(in_.integer_cbrt(), out, "in {}", in_);
                    }
                }
            )*
        };
    }

    gen_tests! {
        i8 => i8_test,
        u8 => u8_test,
        i16 => i16_test,
        u16 => u16_test,
        i32 => i32_test,
        u32 => u32_test,
        i64 => i64_test,
        u64 => u64_test,
        u128 => u128_test,
        isize => isize_test,
        usize => usize_test
    }

    #[test]
    fn i128_test() {
        let tests: [(i128, i128); 10] = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (64, 4),
            (63, 3),
            (23_985_346_875, 2_883),
            (24_958_973_498_745, 29_224),
            (i128::max_value(), 5_541_191_377_756),
        ];
        for &(in_, out) in tests.iter() {
            assert_eq!(in_.integer_cbrt(), out, "in {}", in_);
        }
    }
}
