#![no_std] // don't link the Rust standard library
#![no_main]

use core::{fmt::Write, mem::MaybeUninit, ptr::addr_of_mut, slice};

#[macro_use]
mod macros;

mod arch;
mod console;
mod logger;

#[macro_use]
extern crate log;

extern "C" {
    static kernel_end: u8;
    static kernel_start: u8;
}

/// Entry Point of the HermitCore Loader
/// (called from entry.asm or entry.S)
#[no_mangle]
pub unsafe extern "C" fn loader_main() {
    init_bss();
    arch::message_output_init();
    logger::init();

    info!(
        "Hello, Loader: [{:#x} - {:#x}]",
        &kernel_start as *const u8 as usize, &kernel_end as *const u8 as usize
    );
    loop{}
}

unsafe fn init_bss() {
    extern "C" {
        static mut bss_start: MaybeUninit<u8>;
        static mut bss_end: MaybeUninit<u8>;
    }

    let start_ptr = addr_of_mut!(bss_start);
    let end_ptr = addr_of_mut!(bss_end);
    let len = end_ptr.offset_from(start_ptr).try_into().unwrap();
    let slice = slice::from_raw_parts_mut(start_ptr, len);
    slice.fill(MaybeUninit::new(0));
}

fn _print(args: core::fmt::Arguments<'_>) {
	unsafe {
		console::CONSOLE.write_fmt(args).unwrap();
	}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    // We can't use `println!` or related macros, because `_print` unwraps a result and might panic again
    writeln!(unsafe { &mut console::CONSOLE }, "[LOADER] {info}").ok();

    loop {}
}
