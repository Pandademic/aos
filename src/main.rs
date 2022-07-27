#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
mod ribbit;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("info: main::_start() inited.");
    ribbit::croak();
    loop{}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
