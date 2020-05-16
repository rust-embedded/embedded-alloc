//! A heap allocator for Cortex-M processors.
//!
//! Note that using this as your global allocator requires nightly Rust.
//!
//! # Example
//!
//! For a usage example, see `examples/global_alloc.rs`.

#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

use cortex_m::interrupt::Mutex;
use linked_list_allocator::Heap;

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
    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        self.heap.lock(|heap| heap.init(start_addr, size));
    }
}

unsafe impl GlobalAlloc for CortexMHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.heap
            .lock(|heap| heap.allocate_first_fit(layout))
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.heap
            .lock(|heap| heap.deallocate(NonNull::new_unchecked(ptr), layout));
    }
}
