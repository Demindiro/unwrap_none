// Original source is from https://github.com/rust-lang/rust/pull/83349/files

use core::fmt;

pub trait UnwrapNone {
    /// Consumes `self` while expecting [`None`] and returning nothing.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Some`], with a panic message including the
    /// passed message, and the content of the [`Some`].
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(option_expect_none)]
    ///
    /// use std::collections::HashMap;
    /// let mut squares = HashMap::new();
    /// for i in -10..=10 {
    ///     // This will not panic, since all keys are unique.
    ///     squares.insert(i, i * i).expect_none("duplicate key");
    /// }
    /// ```
    ///
    /// ```should_panic
    /// #![feature(option_expect_none)]
    ///
    /// use std::collections::HashMap;
    /// let mut sqrts = HashMap::new();
    /// for i in -10..=10 {
    ///     // This will panic, since both negative and positive `i` will
    ///     // insert the same `i * i` key, returning the old `Some(i)`.
    ///     sqrts.insert(i * i, i).expect_none("duplicate key");
    /// }
    /// ```
	fn expect_none(self, msg: &str);

    /// Consumes `self` while expecting [`None`] and returning nothing.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Some`], with a custom panic message provided
    /// by the [`Some`]'s value.
    ///
    /// [`Some(v)`]: Some
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(option_unwrap_none)]
    ///
    /// use std::collections::HashMap;
    /// let mut squares = HashMap::new();
    /// for i in -10..=10 {
    ///     // This will not panic, since all keys are unique.
    ///     squares.insert(i, i * i).unwrap_none();
    /// }
    /// ```
    ///
    /// ```should_panic
    /// #![feature(option_unwrap_none)]
    ///
    /// use std::collections::HashMap;
    /// let mut sqrts = HashMap::new();
    /// for i in -10..=10 {
    ///     // This will panic, since both negative and positive `i` will
    ///     // insert the same `i * i` key, returning the old `Some(i)`.
    ///     sqrts.insert(i * i, i).unwrap_none();
    /// }
    /// ```
	fn unwrap_none(self);
}

impl<T> UnwrapNone for Option<T>
where
	T: fmt::Debug
{
    #[inline]
    #[track_caller]
    fn expect_none(self, msg: &str) {
        if let Some(val) = self {
            expect_none_failed(msg, &val);
        }
    }

    #[inline]
    #[track_caller]
    fn unwrap_none(self) {
        if let Some(val) = self {
            expect_none_failed("called `Option::unwrap_none()` on a `Some` value", &val);
        }
    }
}

// This is a separate function to reduce the code size of .expect_none() itself.
#[inline(never)]
#[cold]
#[track_caller]
fn expect_none_failed(msg: &str, value: &dyn fmt::Debug) -> ! {
    panic!("{}: {:?}", msg, value)
}
