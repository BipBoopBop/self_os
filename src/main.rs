#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(self_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use self_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}", "!");
    self_os::init();

    fn stack_overflow() {
        stack_overflow();
    }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("it did not crash");
    loop {
        use self_os::print;
        print!("-");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);   
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    self_os::test_panic_handler(info);
}