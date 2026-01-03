//! Example which shows behavior on pool exhaustion. It simply panics.
#![no_std]
#![no_main]

extern crate alloc;

use cortex_m as _;
use cortex_m_rt::entry;
use defmt::Debug2Format;
use defmt_semihosting as _;

use core::panic::PanicInfo;
use embedded_alloc::TlsfHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe {
        embedded_alloc::init!(HEAP, 16);
    }

    let _vec = alloc::vec![0; 16];

    defmt::error!("unexpected vector allocation success");

    // Panic is expected here.
    semihosting::process::exit(-1);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    defmt::warn!("received expected heap exhaustion panic");
    defmt::warn!("{}: {}", info, Debug2Format(&info.message()));
    semihosting::process::exit(0);
}
