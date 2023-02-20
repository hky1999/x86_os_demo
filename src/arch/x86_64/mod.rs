// Copyright (c) 2018 Colin Finck, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod gdt;
mod interrupts;
mod memory;

use bootloader::BootInfo;

#[no_mangle]
#[link_section = ".text.start"]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    println!("x86 arch init... ");
    loop{}
    crate::drivers::serial::message_output_init();
    gdt::init();
    interrupts::init_idt();
    unsafe { crate::drivers::pic::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    use memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    crate::heap::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    crate::loader_main();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
