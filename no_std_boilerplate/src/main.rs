#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use core::ptr;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    loop {}
}
