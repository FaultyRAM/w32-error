# w32-error

[![Travis CI](https://api.travis-ci.com/FaultyRAM/w32-error.svg)](https://travis-ci.com/FaultyRAM/w32-error)
[![Crates.io](https://img.shields.io/crates/v/w32-error.svg)](https://crates.io/crates/w32-error)
[![Docs.rs](https://docs.rs/w32-error/badge.svg)](https://docs.rs/w32-error)

w32-error is a Rust crate for encapsulating Windows API error codes. It provides the `W32Error`
type, a thin wrapper over a `DWORD` with trait implementations for error handling.

## Features

* `#![no_std]`-friendly - almost all of w32-error is available to `#![no_std]` crates. Parts that
  require libstd (`Error` trait impl, conversion to/from `io::Error`) are kept behind an opt-in
  feature gate.
* Zero overhead - `W32Error` is guaranteed to have the same layout as a `DWORD`. `Display::fmt`
  doesn't access the heap; instead, it uses a small buffer on the stack to receive error messages.

## Usage

To use w32-error, simply add it to your `Cargo.toml`.

By default, w32-error is configured for a `#![no_std]` environment:

```toml
[dependencies]
w32-error = "^0.1.0"
```

Alternatively, features that require libstd can be manually enabled:

```toml
[dependencies]
w32-error = { version = "^0.1.0", features = ["std"] }
```

For more details, see the [API documentation](https://docs.rs/w32-error).

## License

Licensed under either of

* Apache License, Version 2.0,
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
