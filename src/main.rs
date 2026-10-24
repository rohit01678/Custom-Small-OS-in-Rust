#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    rust_os::init();

     fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }


    […] // test_main(), println(…), and loop {}
    #[cfg(test)]
    test_main();
    println!("It did not crash!");
     // trigger a stack overflow
    stack_overflow();
    loop {}
}