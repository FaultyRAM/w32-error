// Copyright (c) 2020 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Encapsulates Windows API error codes.

#![cfg_attr(not(feature = "std"), no_std)]
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

#[cfg(not(feature = "std"))]
use core as std_crate;
#[cfg(feature = "std")]
use std as std_crate;

use std_crate::{
    char,
    fmt::{self, Display, Formatter, Write},
    hint, mem, ptr,
};
#[cfg(feature = "std")]
use std_crate::{convert::TryFrom, error::Error, io};
use winapi::{
    shared::minwindef::DWORD,
    um::{
        errhandlingapi::GetLastError,
        winbase::{
            FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
            FORMAT_MESSAGE_MAX_WIDTH_MASK,
        },
        winnt::WCHAR,
    },
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[must_use = "this `W32Error` is unhandled"]
#[repr(transparent)]
/// A Windows API error.
pub struct W32Error(DWORD);

#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[must_use = "this `TryFromIoError` is unhandled"]
/// A failure to convert an `io::Error` into a `W32Error`.
pub struct TryFromIoError;

impl W32Error {
    /// Wraps an arbitrary error code.
    ///
    /// ```
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
    /// ```
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

impl Display for W32Error {
    #[allow(clippy::cast_possible_truncation)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const MAX_CHARACTERS: usize = 1024;
        // According to the MSDN documentation for `FormatMessage`, `wide_buffer` cannot be larger
        // than 64KB.
        debug_assert!(mem::size_of::<[WCHAR; MAX_CHARACTERS]>() <= 65536);
        let mut wide_buffer = [WCHAR::default(); MAX_CHARACTERS];
        let len = unsafe {
            FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS
                    | FORMAT_MESSAGE_MAX_WIDTH_MASK,
                ptr::null(),
                self.0,
                0,
                wide_buffer.as_mut_ptr(),
                MAX_CHARACTERS as DWORD,
                ptr::null_mut(),
            ) as usize
        };
        if len == 0 {
            // `FormatMessage` failed. Write out the error code itself as a last resort.
            f.write_fmt(format_args!("{:#08X}", self.0))
        } else {
            // Strip leading and trailing whitespace from the error message.
            // If `FormatMessage` is instructed to strip inserts and manual line breaks from the
            // message, they may be replaced with whitespace.
            let mut char_buffer = [char::default(); MAX_CHARACTERS];
            let char_msg = &mut char_buffer[..len];
            let wide_msg = &wide_buffer[..len];
            char::decode_utf16(wide_msg.iter().copied())
                .map(|res| res.unwrap_or(char::REPLACEMENT_CHARACTER))
                .zip(char_msg.iter_mut())
                .for_each(|(src, dst)| *dst = src);
            if let Some(a) = char_msg.iter().position(|c| !c.is_whitespace()) {
                let b = char_msg
                    .iter()
                    .rposition(|c| !c.is_whitespace())
                    .unwrap_or_else(|| unsafe { hint::unreachable_unchecked() });
                for &c in &char_msg[a..=b] {
                    f.write_char(c)?;
                }
            }
            Ok(())
        }
    }
}

#[cfg(feature = "std")]
impl Error for W32Error {}

#[cfg(feature = "std")]
impl From<W32Error> for io::Error {
    #[allow(clippy::cast_possible_wrap)]
    fn from(other: W32Error) -> Self {
        io::Error::from_raw_os_error(other.into_inner() as i32)
    }
}

impl From<DWORD> for W32Error {
    fn from(other: DWORD) -> Self {
        Self::new(other)
    }
}

impl From<W32Error> for DWORD {
    fn from(other: W32Error) -> Self {
        other.into_inner()
    }
}

#[cfg(feature = "std")]
#[allow(clippy::cast_sign_loss)]
impl TryFrom<io::Error> for W32Error {
    type Error = TryFromIoError;

    fn try_from(other: io::Error) -> Result<Self, Self::Error> {
        if let Some(code) = other.raw_os_error() {
            Ok(W32Error::new(code as DWORD))
        } else {
            Err(TryFromIoError)
        }
    }
}

#[cfg(feature = "std")]
impl Display for TryFromIoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("the given `io::Error` did not contain a Windows API error code")
    }
}

#[cfg(feature = "std")]
impl Error for TryFromIoError {}
