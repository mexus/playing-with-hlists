use std::fmt;

use frunk::{HCons, HNil};

/// A helper trait to trick the orphans rule which basically mimics the
/// [Display]. Basically we need it to make different [Display] implementations
/// for generic [HCons<_, _>] and [HCons<_, HNil>].
///
/// While it is technically possible to implement the trait, there is no sense
/// in doing so since it is only used for the internal type machinery.
///
/// [Display]: fmt::Display
#[doc(hidden)]
pub trait MyDisplay {
    /// Like [fmt::Display::fmt].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T: ?Sized + MyDisplay> MyDisplay for &'_ T {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as MyDisplay>::fmt(self, f)
    }
}

/// A wrapper that implements [fmt::Display]. Used for providing a
/// [fmt::Display] implementation for displayable [HCons].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StdDisplayWrapper<T: ?Sized>(T);

impl<T> fmt::Display for StdDisplayWrapper<T>
where
    T: ?Sized + MyDisplay,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<Head, Tail> MyDisplay for HCons<Head, Tail>
where
    Head: fmt::Display,
    Tail: MyDisplay,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.head, StdDisplayWrapper(&self.tail))
    }
}

impl<T> MyDisplay for HCons<T, HNil>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.head.fmt(f)
    }
}

/// An extension trait for [HCons] that adds a `display` method.
pub trait HConsDisplayExt {
    /// Wraps up the provided list into a type that implements [Display], when all
    /// types of the given list ([HNil] excluded) implement [Display].
    ///
    /// [Display]: fmt::Display
    ///
    /// ```rust
    /// use playing_with_hlists::display_frunk::HConsDisplayExt;
    /// use frunk::hlist;
    ///
    /// let some_list = hlist!["abcde", 123, String::from("Wow!")];
    /// assert_eq!("abcde, 123, Wow!", some_list.display().to_string());
    ///
    /// let single = hlist!["kek"];
    /// assert_eq!("kek", single.display().to_string());
    /// ```
    fn display(self) -> StdDisplayWrapper<Self>;
}

/// # Notes on the generics constraint.
///
/// The [`MyDisplay`] trait which is mentioned in the `where` clause is a helper
/// trait which is implemented for [HCons] when all of its members implement
/// [fmt::Display].
impl<Head, Tail> HConsDisplayExt for HCons<Head, Tail>
where
    HCons<Head, Tail>: MyDisplay,
{
    fn display(self) -> StdDisplayWrapper<Self> {
        StdDisplayWrapper(self)
    }
}
