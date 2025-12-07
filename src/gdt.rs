// use lazy_static::lazy_static;
// use x86_64::{instructions::tables::load_tss, registers::segmentation::{CS, DS, SS, Segment}, structures::{gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector}, tss::TaskStateSegment}};

// lazy_static! {
//     static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();
//     static mut TSS: TaskStateSegment = TaskStateSegment::new();
// }

// pub fn load_gdt() {
//     unsafe {
//         let code_selector: SegmentSelector = GDT.append(Descriptor::kernel_code_segment());
//         let data_selector: SegmentSelector = GDT.append(Descriptor::kernel_data_segment());
//         let tss_selector: SegmentSelector = GDT.append(Descriptor::tss_segment(&TSS));

//         GDT.load();

//         CS::set_reg(code_selector);
//         DS::set_reg(data_selector);
//         SS::set_reg(data_selector);
//         load_tss(tss_selector);
//     }
// }