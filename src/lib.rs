#![feature(abi_x86_interrupt)]
#![no_std]

pub mod vga_buffer;
pub mod interrupts;

pub fn init() {
    interrupts::init_idt();
}
