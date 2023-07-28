#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {

    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {

    }
}
