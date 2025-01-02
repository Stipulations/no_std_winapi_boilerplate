#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use core::panic::PanicInfo;
use core::ptr;
use winapi::shared::minwindef::{DWORD, LPCVOID};
use winapi::um::consoleapi::WriteConsoleW;
use winapi::um::processenv::GetStdHandle;
use winapi::um::processthreadsapi::ExitProcess;
use winapi::um::winbase::STD_OUTPUT_HANDLE;

#[panic_handler]
fn panic_handler(_: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    let message = "hello from a cat in a console\n";
    const BUFFER_SIZE: usize = 256;
    let mut wide_message: [u16; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let message_len = str_to_utf16(message, &mut wide_message);

    unsafe {
        let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut chars_written: DWORD = 0;
        WriteConsoleW(
            stdout_handle,
            wide_message.as_ptr() as LPCVOID,
            message_len as DWORD,
            &mut chars_written,
            ptr::null_mut(),
        );
    }

    unsafe {
        ExitProcess(0);
        loop {}
    }
}

#[no_mangle]
pub extern "C" fn memset(s: *mut u8, c: u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *s.add(i) = c;
        }
    }
    s
}

fn str_to_utf16(input: &str, buffer: &mut [u16]) -> usize {
    let mut len = 0;
    for (i, c) in input.encode_utf16().enumerate() {
        if i >= buffer.len() {
            break;
        }
        buffer[i] = c;
        len += 1;
    }
    len
}