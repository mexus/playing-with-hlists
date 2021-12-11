use std::fmt;

use crate::{HeterogenousList, ListTerminator};

/// ```rust
/// use playing_with_hlists::hlist;
///
/// let some_list = hlist!["abcde", 123, String::from("Wow!")];
/// assert_eq!("abcde, 123, Wow!", some_list.to_string());
/// ```
impl<Head, Tail> fmt::Display for HeterogenousList<Head, Tail>
where
    Head: fmt::Display,
    Tail: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.head, self.tail)
    }
}

impl<T> fmt::Display for HeterogenousList<T, ListTerminator>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.head.fmt(f)
    }
}
