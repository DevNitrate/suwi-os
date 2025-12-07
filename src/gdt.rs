use core::ptr::addr_of;

use lazy_static::lazy_static;
use x86_64::{VirtAddr, instructions::tables::load_tss, registers::segmentation::{CS, DS, SS, Segment}, structures::{gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector}, tss::TaskStateSegment}};

pub const DOUBLE_FAULT_IST_IDX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss: TaskStateSegment = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_IDX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(addr_of!(STACK));
            let stack_end = stack_start.align_up(16u64) + STACK_SIZE as u64;
            stack_end
        };
        tss
    };
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt: GlobalDescriptorTable = GlobalDescriptorTable::new();
        let code_selector: SegmentSelector = gdt.append(Descriptor::kernel_code_segment());
        let data_selector: SegmentSelector = gdt.append(Descriptor::kernel_data_segment());
        let tss_selector: SegmentSelector = gdt.append(Descriptor::tss_segment(&TSS));

        (gdt, Selectors { code_selector, data_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector
}

pub fn init_gdt() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        DS::set_reg(GDT.1.data_selector);
        SS::set_reg(GDT.1.data_selector);
        load_tss(GDT.1.tss_selector);
    }
}