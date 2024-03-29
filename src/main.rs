#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
mod vga_buffer;
static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    //
    // for (i, &byte) in HELLO.iter().enumerate(){
    //     unsafe{
    //         *vga_buffer.offset(i as isize*2) = byte;
    //
    //         *vga_buffer.offset(i as isize*2 + 1) = 0xb;
    //     }
    // }
    //
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again\n").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    // vga_buffer::print_something();
    // println!("Hello How are you?");
    // println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();
    // panic!("In here");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
