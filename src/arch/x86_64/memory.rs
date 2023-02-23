use x86_64::{
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

pub const MEMORY_OFFSET: usize = 0;
pub const KERNEL_OFFSET: usize = 0xffffff00_00000000;
pub const KSEG2_OFFSET: usize = 0xfffffe80_00000000;
pub const PHYSICAL_MEMORY_OFFSET: usize = 0xffff8000_00000000;
pub const KERNEL_HEAP_SIZE: usize = 8 * 1024 * 1024; // 8 MB

pub const KERNEL_PM4: usize = (KERNEL_OFFSET >> 39) & 0o777;
pub const KSEG2_PM4: usize = (KSEG2_OFFSET >> 39) & 0o777;
pub const PHYSICAL_MEMORY_PM4: usize = (PHYSICAL_MEMORY_OFFSET >> 39) & 0o777;

pub const USER_STACK_OFFSET: usize = 0x00008000_00000000 - USER_STACK_SIZE;
pub const USER_STACK_SIZE: usize = 8 * 1024 * 1024; // 8 MB, the default config of Linux
pub const KSEG2_START: usize = 0xffff_fe80_0000_0000;

pub const ARCH: &'static str = "x86_64";

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// A FrameAllocator that always returns `None`.
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

