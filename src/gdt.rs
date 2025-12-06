use core::{arch::asm, ptr::addr_of};

use limine::framebuffer::Framebuffer;

use crate::{framebuffer::{Color, render_text}, u64_to_hex};

#[repr(C, packed)]
struct Gdt {
    null: u64,
    code: u64,
    data: u64,
    tss_low: u64,
    tss_high: u64
}

#[repr(C, packed)]
struct Gdtr {
    limit: u16,
    base: u64
}

#[repr(C, packed)]
struct TS {
    reserved0: u32,
    rsp0: u64,
    rsp1: u64,
    rsp2: u64,
    reserved1: u64,
    ist1: u64,
    ist2: u64,
    ist3: u64,
    ist4: u64,
    ist5: u64,
    ist6: u64,
    ist7: u64,
    reserved2: u64,
    reserved3: u16,
    iomap_offset: u16,
}

#[unsafe(no_mangle)]
static mut GDT: Gdt = Gdt {
    null: 0,
    code: (1u64 << 43) | (1u64 << 44) | (1u64 << 47) | (1u64 << 53),
    data: 0,
    tss_low: 0,
    tss_high: 0
};

#[unsafe(no_mangle)]
static mut TSS: TS = TS {
    reserved0: 0,
    rsp0: 0,
    rsp1: 0,
    rsp2: 0,
    reserved1: 0,
    ist1: 0,
    ist2: 0,
    ist3: 0,
    ist4: 0,
    ist5: 0,
    ist6: 0,
    ist7: 0,
    reserved2: 0,
    reserved3: 0,
    iomap_offset: 0,
};

const STACK_LEN: usize = 16 * 1024;
#[unsafe(no_mangle)]
static mut I_KERNEL_STACK: [u8; STACK_LEN] = [0; STACK_LEN];

pub fn load_gdt_tss() {
    unsafe { TSS.rsp0 = (addr_of!(I_KERNEL_STACK) as u64) + STACK_LEN as u64 };

    let tss_addr: u64 = addr_of!(TSS) as u64;
    let tss_size: u64 = size_of::<TS>() as u64 - 1;

    unsafe {
        GDT.tss_low = (tss_size & 0xFFFF) | ((tss_addr & 0xFFFFFF) << 16) | (0x9 << 40) | (1 << 47) | ((tss_addr & 0xFF000000) << 32);
        GDT.tss_high = tss_addr >> 32;
    }

    let gdtr: Gdtr = Gdtr {
        limit: (size_of::<Gdt>() -1) as u16,
        base: addr_of!(GDT) as u64
    };

    unsafe {
        asm!(
            "lgdt [{0}]",
            in(reg) &gdtr,
            options(nostack)
        );

        let tss_selector: u16 = 3 * 8;
        asm!("ltr {0:x}", in(reg) tss_selector, options(nostack));
    }
}