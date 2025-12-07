use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt::DOUBLE_FAULT_IST_IDX, println};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt: InterruptDescriptorTable = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe { idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(DOUBLE_FAULT_IST_IDX); }
        idt.invalid_tss.set_handler_fn(stack_segment_fault_handler);
        idt
    };

}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("\n[EXCEPTION]: breakpoint\n{:?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("\n[EXCEPTION]: double fault({})\n{:?}", error_code, stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame, code: u64) {
    panic!("stack segfault")
}