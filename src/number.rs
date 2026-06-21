//! Sealed numeric conversion traits.
//!
//! [`Number`] lets the formatting functions accept any primitive numeric type,
//! while [`Integer`] backs [`crate::ordinal`]. Both traits are sealed: they
//! cannot be implemented outside this crate, which keeps the public API stable.

mod sealed {
    pub trait Sealed {}
}

/// Any primitive number that can be humanized.
///
/// Implemented for every built-in integer and floating-point type. The trait is
/// sealed and converts the value to [`f64`] for formatting. Conversions of very
/// large 64/128-bit integers may lose low-order precision, which is harmless for
/// magnitude formatting (only the leading significant digits are shown).
pub trait Number: sealed::Sealed + Copy {
    /// Convert the value to [`f64`] for magnitude formatting.
    fn to_f64(self) -> f64;
}

/// Any primitive signed/unsigned integer, used by [`crate::ordinal`].
pub trait Integer: sealed::Sealed + Copy {
    /// Convert the value to [`i128`] (lossless for all primitive integers).
    fn to_i128(self) -> i128;
}

macro_rules! impl_number {
    ($($t:ty),* $(,)?) => {
        $(
            impl sealed::Sealed for $t {}
            impl Number for $t {
                #[inline]
                fn to_f64(self) -> f64 { self as f64 }
            }
        )*
    };
}

macro_rules! impl_integer {
    ($($t:ty),* $(,)?) => {
        $(
            impl Integer for $t {
                #[inline]
                fn to_i128(self) -> i128 { i128::from(self) }
            }
        )*
    };
}

impl_number!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
// `i128::from` only exists for fixed-width types, so the macro excludes the
// platform-dependent (`isize`/`usize`) and 128-bit types; they are handled below.
impl_integer!(i8, i16, i32, i64, u8, u16, u32, u64);

impl Integer for i128 {
    #[inline]
    fn to_i128(self) -> i128 {
        self
    }
}
impl Integer for u128 {
    #[inline]
    fn to_i128(self) -> i128 {
        // Saturating: a u128 above i128::MAX clamps to i128::MAX. Such values are
        // far outside any realistic ordinal use and never reached in practice.
        i128::try_from(self).unwrap_or(i128::MAX)
    }
}
impl Integer for isize {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
impl Integer for usize {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
