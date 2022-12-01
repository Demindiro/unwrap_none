// Original source is from https://github.com/rust-lang/rust/pull/83349/files

#![no_std]

//! See docs with examples on [`UnwrapNone#required-methods`].

use core::fmt;

pub trait UnwrapNone<T> {
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
    /// use std::collections::HashMap;
    ///
    /// use unwrap_none::UnwrapNone;
    ///
    /// let mut squares = HashMap::new();
    /// for i in -10..=10 {
    ///     // This will not panic, since all keys are unique.
    ///     squares.insert(i, i * i).expect_none("duplicate key");
    /// }
    /// ```
    ///
    /// ```should_panic
    /// use std::collections::HashMap;
    ///
    /// use unwrap_none::UnwrapNone;
    ///
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
    /// use std::collections::HashMap;
    ///
    /// use unwrap_none::UnwrapNone;
    ///
    /// let mut squares = HashMap::new();
    /// for i in -10..=10 {
    ///     // This will not panic, since all keys are unique.
    ///     squares.insert(i, i * i).unwrap_none();
    /// }
    /// ```
    ///
    /// ```should_panic
    /// use std::collections::HashMap;
    ///
    /// use unwrap_none::UnwrapNone;
    ///
    /// let mut sqrts = HashMap::new();
    /// for i in -10..=10 {
    ///     // This will panic, since both negative and positive `i` will
    ///     // insert the same `i * i` key, returning the old `Some(i)`.
    ///     sqrts.insert(i * i, i).unwrap_none();
    /// }
    /// ```
    fn unwrap_none(self);

    /// Calls the supplied closure only if the instance is `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unwrap_none::UnwrapNone;
    ///
    /// let input: Option<i32> = None;
    ///
    /// // The closure is not called because `input` is `None`.
    /// input.unwrap_none_or_else(|val| panic!("Got unexpected value: {val}"));
    /// ```
    ///
    /// ```should_panic
    /// # use unwrap_none::UnwrapNone;
    /// #
    /// #
    /// let input = Some(10);
    ///
    /// // The closure is called since `input` is `Some`.
    /// input.unwrap_none_or_else(|val| panic!("Got unexpected value: {val}"));
    /// ```
    fn unwrap_none_or_else<F>(self, f: F)
    where
        F: FnOnce(T);
}

impl<T> UnwrapNone<T> for Option<T>
where
    T: fmt::Debug,
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

    #[inline]
    fn unwrap_none_or_else<F>(self, f: F)
    where
        F: FnOnce(T),
    {
        if let Some(val) = self {
            f(val)
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
