#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use core::ptr;
use core::panic::PanicInfo;
use winapi::um::{processthreadsapi::ExitProcess, winuser::{MessageBoxA, MB_OK}};

#[panic_handler]
fn panic_handler(_: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    let message = b"an amazing message from a cat eating a good ol taco\0";
    let title = b"a title of such a distinguished little message box\0";
    unsafe {
        MessageBoxA(ptr::null_mut(), message.as_ptr() as *const i8, title.as_ptr() as *const i8, MB_OK);
    }
    unsafe { ExitProcess(0) };
    loop {}
}
