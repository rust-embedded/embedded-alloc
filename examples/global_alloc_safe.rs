#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use embedded_alloc::Heap;
use spin;
const HEAP_SIZE: usize = 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();

static START: spin::Once = spin::Once::new();

fn init_heap() {
    START.call_once(|| {
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    });
}

#[entry]
fn main() -> ! {
    // Initialize safely the allocator BEFORE you use it
    init_heap();
    let mut xs = Vec::new();
    xs.push(1);

    loop { /* .. */ }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
