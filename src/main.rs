#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(snail::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use snail::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    snail::init();

    #[cfg(test)]
    test_main();

    snail::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    
    snail::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    snail::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
