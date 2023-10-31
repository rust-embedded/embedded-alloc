//! This is a very basic smoke test that runs in QEMU
//! Reference the QEMU section of the [Embedded Rust Book] for more information
//!
//! This only tests integration of the allocator on an embedded target.
//! Comprehensive allocator tests are located in the allocator dependency.
//!
//! After toolchain installation this test can be run with:
//!
//! ```bash
//! cargo +nightly run --target thumbv7m-none-eabi --example llff_integration_test --all-features
//! ```
//!
//! [Embedded Rust Book]: https://docs.rust-embedded.org/book/intro/index.html

#![feature(allocator_api)]
#![no_main]
#![no_std]

extern crate alloc;
extern crate panic_semihosting;

use alloc::vec::Vec;
use core::mem::{size_of, MaybeUninit};
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn test_global_heap() {
    assert_eq!(HEAP.used(), 0);

    let mut xs: Vec<i32> = alloc::vec![1];
    xs.push(2);
    xs.extend(&[3, 4]);

    // do not optimize xs
    core::hint::black_box(&mut xs);

    assert_eq!(xs.as_slice(), &[1, 2, 3, 4]);
    assert_eq!(HEAP.used(), size_of::<i32>() * xs.len());
}

fn test_allocator_api() {
    // small local heap
    const HEAP_SIZE: usize = 16;
    let heap_mem: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    let local_heap: Heap = Heap::empty();
    unsafe { local_heap.init(heap_mem.as_ptr() as usize, HEAP_SIZE) }

    assert_eq!(local_heap.used(), 0);

    let mut v: Vec<u16, Heap> = Vec::new_in(local_heap);
    v.push(0xCAFE);
    v.extend(&[0xDEAD, 0xFEED]);

    // do not optimize v
    core::hint::black_box(&mut v);

    assert_eq!(v.as_slice(), &[0xCAFE, 0xDEAD, 0xFEED]);
}

#[entry]
fn main() -> ! {
    {
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    #[allow(clippy::type_complexity)]
    let tests: &[(fn() -> (), &'static str)] = &[
        (test_global_heap, "test_global_heap"),
        (test_allocator_api, "test_allocator_api"),
    ];

    for (test_fn, test_name) in tests {
        hprintln!("{}: start", test_name);
        test_fn();
        hprintln!("{}: pass", test_name);
    }

    // exit QEMU with a success status
    debug::exit(debug::EXIT_SUCCESS);
    #[allow(clippy::empty_loop)]
    loop {}
}
