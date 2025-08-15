use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr;
use critical_section::Mutex;

/// The simplest possible heap.
///
/// # Safety
///
/// This heap does **NOT** free allocated memory.
pub struct Heap {
    heap: Mutex<RefCell<SimplestHeap>>,
}

impl Heap {
    /// Create a new UNINITIALIZED heap allocator
    ///
    /// You must initialize this heap using the
    /// [`init`](Self::init) method before using the allocator.
    pub const fn empty() -> Heap {
        Heap {
            heap: Mutex::new(RefCell::new(SimplestHeap::empty())),
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
    /// # Safety
    ///
    /// Obey these or Bad Stuff will happen.
    ///
    /// - This function must be called exactly ONCE.
    /// - `size > 0`
    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        critical_section::with(|cs| {
            self.heap
                .borrow(cs)
                .borrow_mut()
                .init(start_addr as *mut u8, size);
        });
    }

    /// Returns an estimate of the amount of bytes in use.
    pub fn used(&self) -> usize {
        critical_section::with(|cs| self.heap.borrow(cs).borrow().used())
    }

    /// Returns an estimate of the amount of bytes available.
    pub fn free(&self) -> usize {
        critical_section::with(|cs| self.heap.borrow(cs).borrow().free())
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        critical_section::with(|cs| self.heap.borrow(cs).borrow_mut().alloc(layout))
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[cfg(feature = "allocator_api")]
mod allocator_api {
    use super::*;
    use core::alloc::{AllocError, Allocator};
    use core::ptr::NonNull;

    unsafe impl Allocator for Heap {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            match layout.size() {
                0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
                size => critical_section::with(|cs| {
                    let rst = NonNull::new(self.heap.borrow(cs).borrow_mut().alloc(layout))
                        .ok_or(AllocError)?;
                    Ok(NonNull::slice_from_raw_parts(rst, size))
                }),
            }
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
}

struct SimplestHeap {
    arena: *mut u8,
    remaining: usize,
    size: usize,
}

unsafe impl Send for SimplestHeap {}

impl SimplestHeap {
    const fn empty() -> Self {
        Self {
            arena: ptr::null_mut(),
            remaining: 0,
            size: 0,
        }
    }

    fn init(&mut self, start_addr: *mut u8, size: usize) -> Self {
        Self {
            arena: start_addr,
            remaining: size,
            size,
        }
    }

    fn free(&self) -> usize {
        self.remaining
    }

    fn used(&self) -> usize {
        self.size - self.remaining
    }

    fn alloc(&mut self, layout: Layout) -> *mut u8 {
        if layout.size() > self.remaining {
            return ptr::null_mut();
        }

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask_to_round_down = !(layout.align() - 1);

        self.remaining -= layout.size();
        self.remaining &= align_mask_to_round_down;
        self.arena.wrapping_add(self.remaining)
    }
}
