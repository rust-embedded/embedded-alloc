//! This is a very basic smoke test that runs in QEMU
//! Reference the QEMU section of the [Embedded Rust Book] for more information
//!
//! This only tests integration of the allocator on an embedded target.
//! Comprehensive allocator tests are located in the allocator dependency.
//!
//! After toolchain installation this test can be run with:
//!
//! ```bash
//! cargo +nightly run --target thumbv7m-none-eabi --example tlsf_integration_test --all-features
//! ```
//!
//! [Embedded Rust Book]: https://docs.rust-embedded.org/book/intro/index.html

#![feature(allocator_api)]
#![no_main]
#![no_std]

extern crate alloc;
extern crate panic_semihosting;

use alloc::collections::LinkedList;
use core::mem::MaybeUninit;
use core::ptr::addr_of_mut;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use embedded_alloc::TlsfHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 30 * 1024;

fn test_global_heap() {
    const ELEMS: usize = 250;

    let mut allocated = LinkedList::new();
    for _ in 0..ELEMS {
        allocated.push_back(0);
    }
    for i in 0..ELEMS {
        allocated.push_back(i as i32);
    }

    assert_eq!(allocated.len(), 2 * ELEMS);

    for _ in 0..ELEMS {
        allocated.pop_front();
    }

    for i in 0..ELEMS {
        assert_eq!(allocated.pop_front().unwrap(), i as i32);
    }
}

fn test_allocator_api() {
    // small local heap
    const HEAP_SIZE: usize = 256;
    let heap_mem: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    let local_heap: Heap = Heap::empty();
    unsafe { local_heap.init(heap_mem.as_ptr() as usize, HEAP_SIZE) }

    const ELEMS: usize = 2;

    let mut allocated = LinkedList::new_in(local_heap);
    for _ in 0..ELEMS {
        allocated.push_back(0);
    }
    for i in 0..ELEMS {
        allocated.push_back(i as i32);
    }

    assert_eq!(allocated.len(), 2 * ELEMS);

    for _ in 0..ELEMS {
        allocated.pop_front();
    }

    for i in 0..ELEMS {
        assert_eq!(allocated.pop_front().unwrap(), i as i32);
    }
}

#[entry]
fn main() -> ! {
    {
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
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
