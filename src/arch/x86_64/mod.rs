// Copyright (c) 2018 Colin Finck, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod cpu;
mod gdt;
mod interrupts;
// mod memory;

use rboot::BootInfo;

#[no_mangle]
#[link_section = ".text.start"]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    crate::drivers::serial::message_output_init();

    let cpu_id = cpu::id();
    println!("Hello world! from CPU {}!", cpu_id);

    gdt::init();
    interrupts::init_idt();
    unsafe { crate::drivers::pic::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    crate::heap::init_heap();

    // check BootInfo from bootloader
    // println!("{:#x?}", boot_info);
    const PHYSICAL_MEMORY_OFFSET: usize = 0xffff8000_00000000;
    assert_eq!(
        boot_info.physical_memory_offset as usize,
        PHYSICAL_MEMORY_OFFSET
    );

    println!("memory_map: size {}", boot_info.memory_map.len());

    use rboot::MemoryType;
    for region in boot_info.memory_map.iter() {
        println!(
            "type {:#?} p 0x{} v 0x{} c {}",
            region.ty, region.phys_start, region.virt_start, region.page_count
        );
    }

    crate::loader_main();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
