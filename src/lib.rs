#![no_std]

use core::panic::PanicInfo;
use core::ffi::*;

#[no_mangle]
pub extern "C" fn rfunc_add(left: u32, right: u32) -> u32 {
    left + right * 90000
}

// panic handling

extern "C" {
    pub fn write(fd: c_int, buf: *const u8, len: usize);
    pub fn exit(num: c_int) -> !;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let msg = b"it broke\n";
    unsafe { write(1, msg.as_ptr(), msg.len()) }
    unsafe { exit(1) }
}
