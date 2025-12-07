#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod framebuffer;
mod keyboard;
mod paging;
mod gdt;
mod idt;

use core::{arch::asm, panic::PanicInfo, fmt::Write};

use limine::{BaseRevision, framebuffer::Framebuffer, request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker}};
use lazy_static::lazy_static;

use crate::{framebuffer::{Color, WRITER, Writer, clear_screen, get_framebuffer, render_text, write_pixel}, idt::load_idt, keyboard::read_key};

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

lazy_static! {
    pub static ref FRAMEBUFFER: Framebuffer<'static> = FRAMEBUFFER_REQUEST.get_response().unwrap().framebuffers().next().unwrap();
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    clear_screen();

    load_idt();
    // let color: Color = Color::new(255, 0, 0);
    
    println!("test: {}", 1234);
    println!("test: {}", 5678);

    loop {
        let key: u8 = read_key();
        // render_text(&framebuffer, u8_to_hex(keys[0]), 0, 0, 5, &color);
        WRITER.lock().write_byte(key);
        if key == b'p' {
            panic!("voluntary");
        }
    }
    
}

fn hcf() -> ! {
    loop {
        unsafe { asm!("hlt"); }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n{}", info);
    hcf()
}