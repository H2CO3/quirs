//! Various utility functions and types.

use std::usize;
use std::mem::size_of;
use libc::{ c_int, INT_MAX };
use error::{ Error, Result };

/// Attempts to convert a `usize` to an `int` without overflow.
#[cfg_attr(feature = "cargo-clippy", allow(if_same_then_else, cast_possible_truncation, cast_possible_wrap))]
pub fn usize_to_int(n: usize) -> Result<c_int> {
    if size_of::<usize>() < size_of::<c_int>() {
        Ok(n as c_int)
    } else if n <= INT_MAX as usize {
        Ok(n as c_int)
    } else {
        Err(Error::IntOverflow)
    }
}

/// Attempts to convert an `int` to a `usize` without under- or overflow.
#[cfg_attr(feature = "cargo-clippy", allow(if_same_then_else, cast_possible_truncation, cast_possible_wrap))]
pub fn int_to_usize(n: c_int) -> Result<usize> {
    if n < 0 {
        Err(Error::IntOverflow)
    } else if size_of::<c_int>() <= size_of::<usize>() {
        Ok(n as usize)
    } else if n <= usize::MAX as c_int {
        Ok(n as usize)
    } else {
        Err(Error::IntOverflow)
    }
}
