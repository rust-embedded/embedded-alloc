[![crates.io](https://img.shields.io/crates/d/embedded-alloc.svg)](https://crates.io/crates/embedded-alloc)
[![crates.io](https://img.shields.io/crates/v/embedded-alloc.svg)](https://crates.io/crates/embedded-alloc)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.68+-blue.svg) -
 [Documentation](https://docs.rs/embedded-alloc) - [Change log](https://github.com/rust-embedded/embedded-alloc/blob/master/CHANGELOG.md)

# `embedded-alloc`

> A heap allocator for embedded systems.

Note that using this as your global allocator requires Rust 1.68 or later.
(With earlier versions, you need the unstable feature `#![feature(default_alloc_error_handler)]`)

This project is developed and maintained by the [libs team][team].

## Example

Starting with Rust 1.68, this crate can be used as a global allocator on stable Rust:

```rust
#![no_std]
#![no_main]

extern crate alloc;

use cortex_m_rt::entry;
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe {
        embedded_alloc::init!(HEAP, 1024);
    }
    // Alternatively, you can write the code directly to meet specific requirements.
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    // now the allocator is ready types like Box, Vec can be used.

    loop { /* .. */ }
}
```

For a full usage example, see [`examples/global_alloc.rs`](https://github.com/rust-embedded/embedded-alloc/blob/master/examples/global_alloc.rs).

For this to work, an implementation of [`critical-section`](https://github.com/rust-embedded/critical-section) must be provided.

For simple use cases with Cortex-M CPUs you may enable the `critical-section-single-core` feature in the [cortex-m](https://github.com/rust-embedded/cortex-m) crate.
Please refer to the documentation of [`critical-section`](https://docs.rs/critical-section) for further guidance.

## Features

There are two heaps available to use:

* `llff`: Provides `LlffHeap`, a Linked List First Fit heap.
* `tlsf`: Provides `TlsfHeap`, a Two-Level Segregated Fit heap.

The best heap to use will depend on your application, see [#78](https://github.com/rust-embedded/embedded-alloc/pull/78) for more discussion.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [libs team][team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-libs-team
