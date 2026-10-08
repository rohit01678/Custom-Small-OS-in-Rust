#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]//for running tests
#![test_runner(crate::test_runner)]//for running tests
#![reexport_test_harness_main = "test_main"]//for generating a main function for tests

mod vga_buffer;
use core::panic::PanicInfo;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success = 0x10,
    Failed = 0x11,
}
pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;
    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{:?}", _info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
  #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]){
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}
#[test_case] // to see test was called or not.
fn trivial_assertion(){
    println!("trivial assertion");
    assert_eq!(1,1);
    println!("[ok]");
}