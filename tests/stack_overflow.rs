#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use ansi_rgb::{ Foreground, Background, green, red, black, white };
use core::panic::PanicInfo;
use self_os::{exit_qemu, QemuExitCode, serial_println, serial_print};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[no_mangle]
pub extern  "C" fn _start() -> ! {
    serial_print!("{}...\t", "stack_overflow::stack_overflow".fg(black()).bg(white()));

    self_os::gdt::init();
    init_test_idt();

    stack_overflow();

    serial_println!("{}", "[KO]".fg(red()));
    panic!("Execution continue after stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    self_os::test_panic_handler(info);
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(self_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    serial_println!("{}", "[OK]".fg(green()));
    exit_qemu(QemuExitCode::Success);
    loop{}
}