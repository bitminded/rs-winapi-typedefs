//! winapi_typedefs provides definitions for types and constants that correspond
//! to those found in various winapi header files. There are 2 different versions
//! available in this crate.
//! * Ones that are a 1-to-1 translation to be used with unsafe API bindings.
//! * Ones that are to be used with safe interface API bindings. 
//!
//! The latter have been subject to significant changes in order to make the winapi
//! nicer to work with in Rust. Those changes include naming conventions, type
//! abstractions and removal of minor API features that felt outdated and/or
//! problematic in other ways.

pub mod basetsd;
pub mod intsafe;
pub mod windef;
pub mod wingdi;
pub mod winnt;
pub mod winuser;
pub mod winbase;
