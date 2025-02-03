#![feature(allocator_api)]
#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use embedded_alloc::LlffHeap as Heap;

// This is not used, but as of 2023-10-29 allocator_api cannot be used without
// a global heap
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    const HEAP_SIZE: usize = 16;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    let heap: Heap = Heap::empty();
    unsafe { heap.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }

    let mut xs = Vec::new_in(heap);
    xs.push(1);

    #[allow(clippy::empty_loop)]
    loop { /* .. */ }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
