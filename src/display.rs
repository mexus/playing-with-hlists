use std::fmt;

use frunk::{HCons, HNil};

/// A helper trait to trick the orphans rule which basically mimics the
/// [fmt::Display].
///
/// While it is technically possible to implement the trait, there is no sense
/// in doing so: simply go with the [fmt::Display] instead.
#[doc(hidden)]
pub trait MyDisplay {
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

/// Wraps up the provided list into a type that implements [fmt::Display], when
/// all types of the given list ([HNil] excluded) implement [fmt::Display].
///
/// The [`MyDisplay`] trait which is mentioned in the `where` clause is a helper
/// trait which is implemented for all the types which implement [fmt::Display].
///
/// ```rust
/// use playing_with_hlists::display::display_list;
/// use frunk::hlist;
///
/// let some_list = hlist!["abcde", 123, String::from("Wow!")];
/// assert_eq!("abcde, 123, Wow!", display_list(some_list).to_string());
/// ```
pub fn display_list<Head, Tail>(list: HCons<Head, Tail>) -> StdDisplayWrapper<HCons<Head, Tail>>
where
    Head: fmt::Display,
    Tail: MyDisplay,
{
    StdDisplayWrapper(list)
}
