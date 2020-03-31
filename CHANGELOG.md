# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2020-03-31
### Added
- `W32Error` type
    - `W32Error::new`
    - `W32Error::from_io_error` (requires `std`)
    - `W32Error::last_thread_error`
    - `W32Error::into_inner`
    - `W32Error` implements the following traits:
        - `Clone`
        - `Copy`
        - `Display`
        - `Debug`
        - `Eq`
        - `Error` (requires `std`)
        - `From<DWORD>`
        - `Hash`
        - `Ord`
        - `PartialEq`
        - `PartialOrd`
        - `TryFrom<io::Error>` (requires `std`)
    - `io::Error` implements `From<W32Error>` (requires `std`)
- `TryFromIoError` type (requires `std`)
    - `TryFromIoError` is the error type for `W32Error`'s `TryFrom<io::Error>` impl
    - `TryFromIoError` implements the following traits:
        - `Clone`
        - `Copy`
        - `Display`
        - `Debug`
        - `Eq`
        - `Error`
        - `Hash`
        - `Ord`
        - `PartialEq`
        - `PartialOrd`

[Unreleased]: https://github.com/FaultyRAM/w32-error/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/FaultyRAM/w32-error/compare/releases/tag/v1.0.0
