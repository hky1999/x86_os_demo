use buddy_system_allocator::LockedHeap;

const KERNEL_HEAP_SIZE: usize = 8 * 1024 * 1024; // 8 MB

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

pub fn init_heap() {
    const MACHINE_ALIGN: usize = core::mem::size_of::<usize>();
    const HEAP_BLOCK: usize = KERNEL_HEAP_SIZE / MACHINE_ALIGN;
    static mut HEAP: [usize; HEAP_BLOCK] = [0; HEAP_BLOCK];
    unsafe {
        println!(
            "heap init at {:x} of size 0x{:x}",
            HEAP.as_ptr() as usize,
            KERNEL_HEAP_SIZE
        );
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, HEAP_BLOCK * MACHINE_ALIGN);
    }
}