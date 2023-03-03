[![crates.io](https://img.shields.io/crates/d/embedded-alloc.svg)](https://crates.io/crates/embedded-alloc)
[![crates.io](https://img.shields.io/crates/v/embedded-alloc.svg)](https://crates.io/crates/embedded-alloc)

# `embedded-alloc`

> A heap allocator for embedded systems.

Note that using this as your global allocator requires Rust 1.68.
(With earlier versions, you need the unstable feature `#![feature(default_alloc_error_handler)]`)

This project is developed and maintained by the [Cortex-M team][team].

## Example

For a usage example, see `examples/global_alloc.rs`.

## [Documentation](https://docs.rs/embedded-alloc)

## [Change log](CHANGELOG.md)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
