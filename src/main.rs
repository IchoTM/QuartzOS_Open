#![no_std]
#![no_main]

use::core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    loop {}
}

//Panic Handling
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}