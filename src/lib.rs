//! A heap allocator for Cortex-M processors
//!
//! # Example
//!
//! ```
//! // Plug in the allocator crate
//! extern crate alloc_cortex_m;
//! extern crate collections;
//!
//! use collections::Vec;
//!
//! // These symbols come from a linker script
//! extern "C" {
//!     static mut _heap_start: usize;
//!     static mut _heap_end: usize;
//! }
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     // Initialize the heap BEFORE you use the allocator
//!     unsafe { alloc_cortex_m::init(&mut _heap_start, &mut _heap_end) }
//!
//!     let mut xs = Vec::new();
//!     xs.push(1);
//!     // ...
//! }
//! ```
//!
//! And in your linker script, you might have something like:
//!
//! ``` text
//! /* space reserved for the stack */
//! _stack_size = 0x1000;
//!
//! /* `.` is right after the .bss and .data sections */
//! _heap_start = .;
//! _heap_end = ORIGIN(SRAM) + LENGTH(SRAM) - _stack_size;
//! ```

#![allocator]
#![feature(allocator)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate linked_list_allocator;

use core::{cmp, ptr};

use linked_list_allocator::Heap;
use cortex_m::interrupt::Mutex;

/// A global UNINITIALIZED heap allocator
///
/// You must initialize this heap using the
/// [`init`](struct.Heap.html#method.init) method before using the allocator.
static HEAP: Mutex<Heap> = Mutex::new(Heap::empty());

/// Initializes the heap
///
/// This function must be called BEFORE you run any code that makes use of the
/// allocator.
///
/// `start_addr` is the address where the heap will be located.
///
/// `end_addr` points to the end of the heap.
///
/// Note that:
///
/// - The heap grows "upwards", towards larger addresses. Thus `end_addr` must
///   be larger than `start_addr`
///
/// - The size of the heap is `(end_addr as usize) - (start_addr as usize)`. The
///   allocator won't use the byte at `end_addr`.
///
/// # Unsafety
///
/// Obey these or Bad Stuff will happen.
///
/// - This function must be called exactly ONCE.
/// - `end_addr` > `start_addr`
pub unsafe fn init(start_addr: *mut usize, end_addr: *mut usize) {
    let start = start_addr as usize;
    let end = end_addr as usize;
    let size = end - start;
    HEAP.lock(|heap| heap.init(start, size));
}

// Rust allocator interface

#[doc(hidden)]
#[no_mangle]
/// Rust allocation function (c.f. malloc)
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    HEAP.lock(|heap| {
        heap.allocate_first_fit(size, align).expect("out of memory")
    })
}

#[doc(hidden)]
#[no_mangle]
pub extern fn __rust_allocate_zeroed(size: usize, align: usize) -> *mut u8 {
    let ptr = __rust_allocate(size, align);
    if !ptr.is_null() {
        ptr::write_bytes(ptr, 0, size);
    }
    ptr
}

/// Rust de-allocation function (c.f. free)
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize) {
    HEAP.lock(|heap| unsafe { heap.deallocate(ptr, size, align) });
}

/// Rust re-allocation function (c.f. realloc)
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __rust_reallocate(ptr: *mut u8,
                                    size: usize,
                                    new_size: usize,
                                    align: usize)
                                    -> *mut u8 {

    // from: https://github.com/rust-lang/rust/blob/
    //     c66d2380a810c9a2b3dbb4f93a830b101ee49cc2/
    //     src/liballoc_system/lib.rs#L98-L101

    let new_ptr = __rust_allocate(new_size, align);
    unsafe { ptr::copy(ptr, new_ptr, cmp::min(size, new_size)) };
    __rust_deallocate(ptr, size, align);
    new_ptr
}

/// Rust re-allocation function which guarantees not to move the data
/// somewhere else.
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(_ptr: *mut u8,
                                            size: usize,
                                            _new_size: usize,
                                            _align: usize)
                                            -> usize {
    size
}

/// Some allocators (pool allocators generally) over-allocate. This checks how
/// much space there is at a location. Our allocator doesn't over allocate so
/// this just returns `size`
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
