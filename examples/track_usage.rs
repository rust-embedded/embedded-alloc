#![no_std]
#![no_main]

extern crate alloc;

use cortex_m as _;
use cortex_m_rt::entry;
use defmt::Debug2Format;
use defmt_semihosting as _;

use core::{mem::MaybeUninit, panic::PanicInfo};
use embedded_alloc::TlsfHeap as Heap;
//use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    const HEAP_SIZE: usize = 4096;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }

    let mut alloc_vecs = alloc::vec::Vec::new();
    let mut free_memory = HEAP_SIZE;
    // Keep allocating until we are getting low on memory. It doesn't have to end in a panic.
    while free_memory > 512 {
        defmt::info!(
            "{} of {} heap memory allocated so far...",
            HEAP_SIZE - free_memory,
            HEAP_SIZE
        );
        let new_vec = alloc::vec![1_u8; 64];
        alloc_vecs.push(new_vec);
        free_memory = HEAP.free();
    }

    drop(alloc_vecs);

    defmt::info!(
        "{} of {} heap memory are allocated after drop",
        HEAP_SIZE - HEAP.free(),
        HEAP_SIZE
    );

    semihosting::process::exit(0);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    defmt::error!("{}: {}", info, Debug2Format(&info.message()));
    semihosting::process::exit(-1);
}
