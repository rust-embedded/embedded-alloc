#![no_std]
#![no_main]

extern crate alloc;

use cortex_m as _;
use cortex_m_rt::entry;
use defmt_semihosting as _;

use core::panic::PanicInfo;
// Linked-List First Fit Heap allocator (feature = "llff")
use embedded_alloc::LlffHeap as Heap;
// Two-Level Segregated Fit Heap allocator (feature = "tlsf")
// use embedded_alloc::TlsfHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe {
        embedded_alloc::init!(HEAP, 1024);
    }

    let vec = alloc::vec![1];

    defmt::info!("Allocated vector: {:?}", vec.as_slice());

    let string = alloc::string::String::from("Hello, world!");

    defmt::info!("Allocated string: {:?}", string.as_str());

    semihosting::process::exit(0);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    defmt::error!("{}", info);
    semihosting::process::exit(0);
}
