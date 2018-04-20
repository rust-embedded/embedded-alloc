//! A heap allocator for Cortex-M processors
//!
//! # Example
//!
//! ```
//! // Plug in the allocator crate
//! extern crate alloc_cortex_m;
//! extern crate alloc;
//!
//! use alloc::Vec;
//! use alloc_cortex_m::CortexMHeap;
//!
//! #[global_allocator]
//! static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
//!
//! // These symbols come from a linker script
//! extern "C" {
//!     static mut _sheap: u32;
//!     static mut _eheap: u32;
//! }
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     // Initialize the heap BEFORE you use the allocator
//!     let start = unsafe { &mut _sheap as *mut u32 as usize };
//!     let end = unsafe { &mut _sheap as *mut u32 as usize };
//!     unsafe { ALLOCATOR.init(start, end - start) }
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

#![feature(const_fn)]
#![no_std]
#![feature(alloc, allocator_api)]

extern crate cortex_m;
extern crate linked_list_allocator;
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout, Opaque};
use core::ptr::NonNull;

use linked_list_allocator::Heap;
use cortex_m::interrupt::Mutex;

pub struct CortexMHeap {
    heap: Mutex<Heap>,
}

impl CortexMHeap {

    /// Crate a new UNINITIALIZED heap allocator
    ///
    /// You must initialize this heap using the
    /// [`init`](struct.CortexMHeap.html#method.init) method before using the allocator.
    pub const fn empty() -> CortexMHeap {
        CortexMHeap {
            heap: Mutex::new(Heap::empty()),
        }
    }

    /// Initializes the heap
    ///
    /// This function must be called BEFORE you run any code that makes use of the
    /// allocator.
    ///
    /// `start_addr` is the address where the heap will be located.
    ///
    /// `size` is the size of the heap in bytes.
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
    /// - `size > 0`
    pub unsafe fn init(&self, start_addr: usize, size: usize){
        self.heap.lock(|heap| heap.init(start_addr, size));
    }
}

unsafe impl GlobalAlloc for CortexMHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
        self.heap.lock(|heap| {
            heap.allocate_first_fit(layout)
        }).ok().map_or(0 as *mut Opaque, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut Opaque, layout: Layout) {
        self.heap.lock(|heap| heap.deallocate(NonNull::new_unchecked(ptr), layout));
    }
}
