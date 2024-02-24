use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::{self, NonNull};

use const_default::ConstDefault;
use critical_section::Mutex;
use rlsf::Tlsf;

type TlsfHeap = Tlsf<'static, usize, usize, { usize::BITS as usize }, { usize::BITS as usize }>;

/// A two-Level segregated fit heap.
pub struct Heap {
    heap: Mutex<RefCell<TlsfHeap>>,
}

impl Heap {
    /// Create a new UNINITIALIZED heap allocator
    ///
    /// You must initialize this heap using the
    /// [`init`](Self::init) method before using the allocator.
    pub const fn empty() -> Heap {
        Heap {
            heap: Mutex::new(RefCell::new(ConstDefault::DEFAULT)),
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
    /// - The heap grows "upwards", towards larger addresses. Thus `start_addr` will
    ///   be the smallest address used.
    ///
    /// - The largest address used is `start_addr + size - 1`, so if `start_addr` is
    ///   `0x1000` and `size` is `0x30000` then the allocator won't use memory at
    ///   addresses `0x31000` and larger.
    ///
    /// # Safety
    ///
    /// Obey these or Bad Stuff will happen.
    ///
    /// - This function must be called exactly ONCE.
    /// - `size > 0`
    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        critical_section::with(|cs| {
            let block: &[u8] = core::slice::from_raw_parts(start_addr as *const u8, size);
            self.heap
                .borrow(cs)
                .borrow_mut()
                .insert_free_block_ptr(block.into());
        });
    }

    fn alloc(&self, layout: Layout) -> Option<NonNull<u8>> {
        critical_section::with(|cs| self.heap.borrow(cs).borrow_mut().allocate(layout))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        critical_section::with(|cs| {
            self.heap
                .borrow(cs)
                .borrow_mut()
                .deallocate(NonNull::new_unchecked(ptr), layout.align())
        })
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc(layout)
            .map_or(ptr::null_mut(), |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.dealloc(ptr, layout)
    }
}

#[cfg(feature = "allocator_api")]
mod allocator_api {
    use super::*;
    use core::alloc::{AllocError, Allocator};

    unsafe impl Allocator for Heap {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            match layout.size() {
                0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
                size => self.alloc(layout).map_or(Err(AllocError), |allocation| {
                    Ok(NonNull::slice_from_raw_parts(allocation, size))
                }),
            }
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            if layout.size() != 0 {
                self.dealloc(ptr.as_ptr(), layout);
            }
        }
    }
}
