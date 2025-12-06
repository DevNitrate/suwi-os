#![no_std]
#![no_main]

mod framebuffer;
mod keyboard;

use core::{arch::asm, panic::PanicInfo};

use limine::{BaseRevision, request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker}};

use crate::{framebuffer::{Color, render_text, write_pixel}, keyboard::read_key};

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

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    let framebuffer_response = FRAMEBUFFER_REQUEST.get_response().unwrap();
    let framebuffer = framebuffer_response.framebuffers().next().unwrap();
    for x in 0..framebuffer.width() {
        for y in 0..framebuffer.height() {
            let col: Color = Color::new(&framebuffer, 13, 13, 13);
            write_pixel(&framebuffer, x, y, &col);
        }
    }

    let color: Color = Color::new(&framebuffer, 255, 255, 255);
    let mut keys: [u8; 256] = [0; 256];
    let mut k_idx: usize = 0;
    
    loop {
        keys[k_idx] = read_key();
        // render_text(&framebuffer, u8_to_hex(key[0]), 0, 0, 5, &color);
        render_text(&framebuffer, &keys[0..k_idx], 0, 0, 5, &color);
        k_idx += 1;
    }

    hcf()    
}

pub fn u8_to_hex(n: u8) -> [u8; 4] {
    const LUT: &[u8; 16] = b"0123456789ABCDEF";

    let high = LUT[(n >> 4) as usize];
    let low  = LUT[(n & 0xF) as usize];

    [b'0', b'x', high, low]
}


fn hcf() -> ! {
    loop {
        unsafe { asm!("hlt"); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let framebuffer_response = FRAMEBUFFER_REQUEST.get_response().unwrap();
    let framebuffer = framebuffer_response.framebuffers().next().unwrap();

    render_text(&framebuffer, "panic occured", 200, 200, 10, &Color::new(&framebuffer, 255, 0, 0));

    hcf()
}