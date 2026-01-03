//! This examples requires nightly for the allocator API.
#![feature(allocator_api)]
#![no_std]
#![no_main]

extern crate alloc;

use core::{mem::MaybeUninit, panic::PanicInfo};
use cortex_m as _;
use cortex_m_rt::entry;
use defmt_semihosting as _;
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

    let mut vec = alloc::vec::Vec::new_in(heap);
    vec.push(1);

    defmt::info!("Allocated vector: {:?}", vec.as_slice());

    semihosting::process::exit(0);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    defmt::error!("{}", info);
    semihosting::process::exit(-1);
}
