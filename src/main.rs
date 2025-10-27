#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use::core::panic::PanicInfo;

mod vga_buffer;
mod serial;

//Escaping QEMU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

//Exit QEMU gracefully
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }

    serial_println!("Exited QEMU with code: {:?}", exit_code);
}

//Panic Handling
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop{}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    //Test code - Currently will not run even in test mode, fix later
    #[cfg(test)]{
        test_main();
    }

    #[cfg(not(test))]
    {
        println!("Non-test mode...");
        serial_println!("Running kernel in non-test build");

        //Supposed to exit QEMU - Does not seem work currently
        let exit_code = QemuExitCode::Success;
        exit_qemu(exit_code);
    }

    loop {}
}