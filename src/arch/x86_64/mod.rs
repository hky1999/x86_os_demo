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

    dump_boot_info_memory_layout(boot_info);

    crate::loader_main();
}

pub fn dump_boot_info_memory_layout(boot_info: &'static BootInfo) {
    println!("Dump boot_info memory layout:");
    for (idx, m) in boot_info.memory_map.into_iter().enumerate() {
        println!(
            "[{:>2}] [{:#x}-{:#x}] {:?} {:#x} {}",
            idx,
            m.phys_start,
            m.phys_start + m.page_count * 0x1000,
            m.ty,
            m.page_count * 0x1000,
            m.page_count
        );
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
