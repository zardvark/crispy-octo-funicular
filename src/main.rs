#![no_std] //don't include std library to create "freestanding" binary, that can be run without an
           //OS
#![no_main] //to not use normal entry point. (regularly execution starts in C runtime lib "crt0"
            //which then invokes the entry point of the Rust runtime, marked by "start". Then the
            //"main" function is called.
#![feature(custom_test_frameworks)] //allows for use of [test_case] and [test_runner]
#![test_runner(blog_os::test_runner)] //all [test_case]'s get passed here
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;

#[no_mangle] //use the name "_start" so linker knows entry point (compiler usually generates unique
             //name like _ZN3blog_os4_start
pub extern "C" fn _start() -> ! { //"C" tells compiler to use C calling convention for the function
    println!("Hello {}", "Jeffrey");
    
    #[cfg(test)]
    test_main();
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


#[cfg(test)]
#[panic_handler] //need to define panic_handler ourselves when using no_std
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
