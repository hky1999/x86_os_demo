#![no_std] // don't link the Rust standard library
#![no_main]
#![feature(abi_x86_interrupt)]

use core::fmt::Write;

#[macro_use]
mod macros;

mod arch;
mod console;
mod logger;
mod drivers;

#[macro_use]
extern crate log;

use core::panic::PanicInfo;

fn loader_main() -> ! {
    logger::init();

    info!("hello x86");

    arch::hlt_loop();
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
