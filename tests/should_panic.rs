#![no_std]
#![no_main]

use ansi_rgb::{ Foreground, Background, green, red, black, white };
use core::panic::PanicInfo;
use self_os::{QemuExitCode, exit_qemu, serial_println, serial_print};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("{}", "[test did not panic]".fg(red()));
    exit_qemu(QemuExitCode::Failed);

    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", "[ok]".fg(green()));
    exit_qemu(QemuExitCode::Success);
    loop{}
}

fn should_fail() {
    serial_print!("{}...\t", "should_panic::should_fail".fg(black()).bg(white()));
    assert_eq!(0,1);
}