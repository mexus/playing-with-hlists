//! Experiments with heterogenous lists (and [frunk]).
//!
//! Currently the following examples are implemented in the respective modules:
//!
//! * `display`: [Display] implementation for a heterogenous list.
//! * `display_frunk`: Ad-hoc [Display] implementation for a heterogenous list
//!   from [frunk].
//!
//! [frunk]: https://github.com/lloydmeta/frunk
//!
//! [Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html

#![deny(missing_docs)]
#![deny(rustdoc::all)]

/// [Display][std::fmt::Display] implementation for heterogenous lists.
pub mod display;

/// [Display][std::fmt::Display] implementation for heterogenous lists from
/// [frunk](https://github.com/lloydmeta/frunk).
pub mod display_frunk;

/// A recursively-defined non-empty heterogenous list.
///
/// Use a [`hlist!`] macro to create an object of this type with multiple objects.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeterogenousList<Head, Tail> {
    /// First element of the list.
    pub head: Head,

    /// Rest elements of the list.
    pub tail: Tail,
}

/// Terminating entry of a heterogenous list.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListTerminator;

/// Constructs a heterogenous list with at least one element.
#[macro_export]
macro_rules! hlist {
    ( $head:expr $(,)? ) => {
        $crate::HeterogenousList {
            head: $head,
            tail: $crate::ListTerminator,
        }
    };

    ( $head:expr $(, $other:expr)+ $(,)? ) => {
        $crate::HeterogenousList {
            head: $head,
            tail: $crate::hlist!( $($other),* ),
        }
    };
}
