#![no_std]
#![no_main]
#![feature(asm)]

use::core::panic::PanicInfo;

mod vga_buffer;

//Panic Handling
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Intentional Crash");
    loop {}
}