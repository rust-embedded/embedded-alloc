#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[repr(align(4))]
struct Aligned<T>(T);

#[entry]
fn main() -> ! {
    static mut M: Aligned<[u8; tlsf::MAX_BLOCK_SIZE as usize]> =
        Aligned([0; tlsf::MAX_BLOCK_SIZE as usize]);

    ALLOCATOR.extend(&mut M.0);

    let mut xs = Vec::new();
    xs.push(1);

    loop { /* .. */ }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
