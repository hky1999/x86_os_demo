#![no_std] // don't link the Rust standard library
#![no_main]
#![feature(abi_x86_interrupt)]

use core::fmt::Write;

#[macro_use]
mod macros;

mod arch;
mod console;
mod logger;

#[macro_use]
extern crate log;

use core::panic::PanicInfo;

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 因为编译器会寻找一个名为 `_start` 的函数，所以这个函数就是入口点
    // 默认命名为 `_start`
    arch::init();
    logger::init();

    info!("hello x86");
    
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn _print(args: core::fmt::Arguments<'_>) {
    unsafe {
        console::CONSOLE.write_fmt(args).unwrap();
    }
}
