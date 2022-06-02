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
use core::ptr::{self, NonNull};

#[cfg(not(feature = "nrf-softdevice"))]
use cortex_m::interrupt::{
    Mutex,
    CriticalSection,
};
#[cfg(feature = "nrf-softdevice")]
use {
    critical_section::CriticalSection,
    embassy::blocking_mutex::CriticalSectionMutex as Mutex,
};
use linked_list_allocator::Heap;

#[cfg(not(feature = "nrf-softdevice"))]
#[inline]
fn block_interrupts<F, R>(f: F) -> R
    where F: FnOnce(&CriticalSection) -> R
{
    cortex_m::interrupt::free(f)
}

#[cfg(feature = "nrf-softdevice")]
#[inline]
fn block_interrupts<F, R>(f: F) -> R
    where F: FnOnce(CriticalSection) -> R
{
    critical_section::with(f)
}

pub struct CortexMHeap {
    heap: Mutex<RefCell<Heap>>,
}

impl CortexMHeap {
    /// Crate a new UNINITIALIZED heap allocator
    ///
    /// You must initialize this heap using the
    /// [`init`](struct.CortexMHeap.html#method.init) method before using the allocator.
    #[cfg(not(feature = "nrf-softdevice"))]
    pub const fn empty() -> CortexMHeap {
        CortexMHeap {
            heap: Mutex::new(RefCell::new(Heap::empty())),
        }
    }

    #[cfg(feature = "nrf-softdevice")]
    pub fn empty() -> CortexMHeap {
        CortexMHeap {
            heap: Mutex::new(RefCell::new(Heap::empty())),
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
    /// # Safety
    ///
    /// Obey these or Bad Stuff will happen.
    ///
    /// - This function must be called exactly ONCE.
    /// - `size > 0`
    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        block_interrupts(|cs| {
            self.heap.borrow(cs).borrow_mut().init(start_addr, size);
        });
    }

    /// Returns an estimate of the amount of bytes in use.
    pub fn used(&self) -> usize {
        block_interrupts(|cs| self.heap.borrow(cs).borrow_mut().used())
    }

    /// Returns an estimate of the amount of bytes available.
    pub fn free(&self) -> usize {
        block_interrupts(|cs| self.heap.borrow(cs).borrow_mut().free())
    }
}

unsafe impl GlobalAlloc for CortexMHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        block_interrupts(|cs| {
            self.heap
                .borrow(cs)
                .borrow_mut()
                .allocate_first_fit(layout)
                .ok()
                .map_or(ptr::null_mut(), |allocation| allocation.as_ptr())
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        block_interrupts(|cs| {
            self.heap
                .borrow(cs)
                .borrow_mut()
                .deallocate(NonNull::new_unchecked(ptr), layout)
        });
    }
}
