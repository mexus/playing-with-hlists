//! Experiments with heterogenous lists and [frunk].
//!
//! Currently the following examples are implemented:
//!
//! * [display_list][display::display_list]: [Display][std::fmt::Display]
//!   implementation for a heterogenous list.
//!
//! [frunk]: https://github.com/lloydmeta/frunk

#![deny(missing_docs)]
#![deny(rustdoc::all)]

/// Display heterogenous list.
pub mod display;
