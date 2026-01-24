#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "allocator_api", feature(allocator_api))]
#![warn(missing_docs)]

#[cfg(feature = "llff")]
mod llff;
#[cfg(feature = "tlsf")]
mod tlsf;

#[cfg(feature = "llff")]
pub use llff::Heap as LlffHeap;
#[cfg(feature = "tlsf")]
pub use tlsf::Heap as TlsfHeap;

/// Initialize the global heap.
///
/// This macro creates a static, uninitialized memory buffer of the specified size and
/// initializes the heap instance with that buffer.
///
/// # Parameters
///
/// - `$heap:ident`: The identifier of the global heap instance to initialize.
/// - `$size:expr`: An expression evaluating to a `usize` that specifies the size of the
///   static memory buffer in bytes. It must be **greater than zero**.
///
/// # Safety
///
/// This macro must be called first, before any operations on the heap, and **only once**.
/// It internally calls `Heap::init(...)` on the heap,
/// so `Heap::init(...)` should not be called directly if this macro is used.
///
/// # Panics
///
/// This macro will panic if either of the following are true:
///
/// - this function is called more than ONCE.
/// - `size == 0`.
///
/// # Example
///
/// ```rust
/// use cortex_m_rt::entry;
/// use embedded_alloc::LlffHeap as Heap;
///
/// #[global_allocator]
/// static HEAP: Heap = Heap::empty();
///
/// #[entry]
/// fn main() -> ! {
///     // Initialize the allocator BEFORE you use it
///     unsafe {
///         embedded_alloc::init!(HEAP, 1024);
///     }
///     let mut xs = Vec::new();
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! init {
    ($heap:ident, $size:expr) => {
        static mut HEAP_MEM: [::core::mem::MaybeUninit<u8>; $size] =
            [::core::mem::MaybeUninit::uninit(); $size];
        $heap.init(&raw mut HEAP_MEM as usize, $size)
    };
}
