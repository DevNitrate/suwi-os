#![no_std]
#![no_main]

mod framebuffer;

use core::{arch::asm, panic::PanicInfo};

use limine::{BaseRevision, framebuffer::Framebuffer, request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker}};

use crate::framebuffer::{Color, render_char, render_rect, render_text, write_pixel};

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
            let col: Color = Color::new(&framebuffer, ((x as f64 / framebuffer.width() as f64) * 255.0) as u8, ((y as f64 / framebuffer.height() as f64) * 255.0) as u8, 0);
            write_pixel(&framebuffer, x, y, &col);
        }
    }

    let color: Color = Color::new(&framebuffer, 255, 255, 255);

    render_text(&framebuffer, "hello, world", 100, 100, 10, &color);

    hcf()    
}

fn hcf() -> ! {
    loop {
        unsafe { asm!("hlt"); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hcf()
}