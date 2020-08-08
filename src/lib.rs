//! A heap allocator for Cortex-M processors.
//!
//! Note that using this as your global allocator requires nightly Rust.
//!
//! # Example
//!
//! For a usage example, see `examples/global_alloc.rs`.

#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::NonNull;

use cortex_m::interrupt::Mutex;
// use linked_list_allocator::Heap;
use tlsf::Tlsf as Heap;

pub struct CortexMHeap {
    heap: Mutex<RefCell<Heap>>,
}

impl CortexMHeap {
    /// Crate a new UNINITIALIZED heap allocator
    ///
    /// You must initialize this heap using the
    /// [`init`](struct.CortexMHeap.html#method.init) method before using the allocator.
    pub const fn empty() -> CortexMHeap {
        CortexMHeap {
            heap: Mutex::new(RefCell::new(Heap::new())),
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
    pub unsafe fn extend(&self, block: &'static mut [u8]) {
        cortex_m::interrupt::free(move |cs| {
            self.heap
                .borrow(cs)
                .borrow_mut()
                .extend(block);
        });
    }
}

unsafe impl GlobalAlloc for CortexMHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        cortex_m::interrupt::free(|cs| {
            let returned_mem = self.heap.borrow(cs).borrow_mut().alloc(layout);
            match returned_mem {
                Ok(mem_ptr) => mem_ptr.as_ptr(),
                Err(_) => 0_usize as *mut u8,
            }
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if let Some(mem_ptr) = NonNull::new(ptr) {
            cortex_m::interrupt::free(|cs| {
                self.heap.borrow(cs).borrow_mut().dealloc(mem_ptr);
            });
        }
    }
}
