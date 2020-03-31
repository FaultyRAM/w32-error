// Copyright (c) 2020 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Encapsulates Windows API error codes.

#![no_std]
#![deny(
    clippy::all,
    clippy::pedantic,
    warnings,
    future_incompatible,
    rust_2018_idioms,
    rustdoc,
    unused,
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    non_ascii_idents,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_import_braces,
    unused_lifetimes,
    unused_results
)]
#![allow(clippy::must_use_candidate)]

#[cfg(not(target_os = "windows"))]
compile_error!("w32-error only supports Windows-based targets");

use winapi::{shared::minwindef::DWORD, um::errhandlingapi::GetLastError};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[must_use = "this `W32Error` is unhandled"]
#[repr(transparent)]
/// A Windows API error.
pub struct W32Error(DWORD);

impl W32Error {
    /// Wraps an arbitrary error code.
    ///
    /// ```ignore
    /// # use w32_error::W32Error;
    /// let error = W32Error::new(0);
    /// println!("{}", error);
    /// ```
    pub const fn new(code: DWORD) -> Self {
        Self(code)
    }

    /// Wraps the error code that is currently set for the calling thread.
    ///
    /// This is equivalent to calling the Windows API function `GetLastError` and passing the return
    /// value to `W32Error::new`.
    ///
    /// ```ignore
    /// # use w32_error::W32Error;
    /// let error = W32Error::last_thread_error();
    /// println!("{}", error);
    /// ```
    pub fn last_thread_error() -> Self {
        Self::new(unsafe { GetLastError() })
    }

    /// Returns the underlying error code wrapped by a `W32Error`.
    ///
    /// ```
    /// # use w32_error::W32Error;
    /// assert_eq!(W32Error::new(0).into_inner(), 0);
    /// ```
    pub const fn into_inner(self) -> DWORD {
        self.0
    }
}
