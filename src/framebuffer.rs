use core::fmt;

use lazy_static::lazy_static;
use limine::framebuffer::Framebuffer;
use spin::Mutex;

use crate::{FRAMEBUFFER};

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(5));
}

const FONT: [u64; 94] = [
    0x1818181818001800,
    0x6666660000000000,
    0x6C6CFE6CFE6C6C00,
    0x183E603C067C1800,
    0x00C6CC183066C600,
    0x386C3876DCCC7600,
    0x1818300000000000,
    0x0C18303030180C00,
    0x30180C0C0C183000,
    0x00663CFF3C660000,
    0x0018187E18180000,
    0x0000000000181830,
    0x0000007E00000000,
    0x0000000000181800,
    0x03060C183060C000,
    0x3C666E7666663C00,
    0x1838181818187E00,
    0x3C660C1830607E00,
    0x3C66061C06663C00,
    0x1C3C6CCCFE0C0C00,
    0x7E607C0606663C00,
    0x1C30607C66663C00,
    0x7E06060C18181800,
    0x3C66663C66663C00,
    0x3C66663E060C3800,
    0x0018180000181800,
    0x0018180000181830,
    0x0C18306030180C00,
    0x00007E007E000000,
    0x6030180C18306000,
    0x3C66060C18001800,
    0x7CC6DEDEDEC07C00,
    0x183C66667E666600,
    0x7C66667C66667C00,
    0x3C66606060663C00,
    0x786C6666666C7800,
    0x7E60607C60607E00,
    0x7E60607C60606000,
    0x3C66606E66663E00,
    0x6666667E66666600,
    0x7E18181818187E00,
    0x0606060606663C00,
    0xC6CCD8F0D8CCC600,
    0x6060606060607E00,
    0xC6EEFED6C6C6C600,
    0xC6E6F6DECEC6C600,
    0x3C66666666663C00,
    0x7C66667C60606000,
    0x3C666666666C3600,
    0x7C66667C6C666600,
    0x3C66603C06663C00,
    0x7E18181818181800,
    0x6666666666663C00,
    0x66666666663C1800,
    0xC6C6C6D6FEEEC600,
    0xC3663C183C66C300,
    0xC3663C1818181800,
    0x7E060C1830607E00,
    0x3C30303030303C00,
    0xC06030180C060300,
    0x3C0C0C0C0C0C3C00,
    0x10386CC600000000,
    0x00000000000000FF,
    0x180C060000000000,
    0x00003C063E663E00,
    0x60607C6666667C00,
    0x00003C6060603C00,
    0x06063E6666663E00,
    0x00003C667E603C00,
    0x1C307C3030303000,
    0x00003E66663E067C,
    0x60607C6666666600,
    0x1800381818181E00,
    0x0C000C0C0C0C0C78,
    0x6060666C786C6600,
    0x3818181818181E00,
    0x0000CCFED6D6C600,
    0x00007C6666666600,
    0x00003C6666663C00,
    0x00007C66667C6060,
    0x00003E66663E0606,
    0x00007C6660606000,
    0x00003E603C067C00,
    0x30307E3030301E00,
    0x0000666666663E00,
    0x00006666663C1800,
    0x0000C6C6D67C6C00,
    0x0000C66C386CC600,
    0x00006666663E063C,
    0x00007E0C18307E00,
    0x0E18187018180E00,
    0x1818181818181800,
    0x7018180E18187000,
    0x76DC000000000000
];

pub struct Writer {
    x_pos: u64,
    pub y_pos: u64,
    font_size: u64,
    color: Color,
    offset_add: u64
}

impl Writer {
    pub fn new(font_size: u64) -> Self {
        Self {
            x_pos: 5,
            y_pos: 5,
            font_size,
            color: Color::new(255, 255, 255),
            offset_add: (font_size * 8) + 2
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        let fb: &Framebuffer = get_framebuffer();

        match byte {
            b'\n' => self.new_line(),
            b'\0' => return,
            b' ' => self.x_pos += self.offset_add,
            b'\t' => self.x_pos += self.offset_add * 4,
            _ => {
                if (self.x_pos + self.offset_add) >= fb.width() {
                    self.new_line();
                }

                render_char(byte as char, self.x_pos, self.y_pos, &self.color, self.font_size);
                self.x_pos += self.offset_add;
            }
        }
    }

    pub fn new_line(&mut self) {
        let fb: &Framebuffer = get_framebuffer();
        self.x_pos = 5;
        self.y_pos += self.offset_add;
        if (self.y_pos + self.offset_add) >= fb.height() {
            self.y_pos = 5;
            clear_screen();
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7E => self.write_byte(byte),
                0x8..=0xA => self.write_byte(byte),
                _ => self.write_byte(b'#'),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn clear_screen() {
    let framebuffer: &Framebuffer = get_framebuffer();

    let col: Color = Color::new(13, 13, 13);
    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            write_pixel(x, y, &col);
        }
    }
}

pub struct Color {
    value: u32
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        let framebuffer: &Framebuffer = get_framebuffer();
        let value: u32 = ((r as u32) << framebuffer.red_mask_shift()) | ((g as u32) << framebuffer.green_mask_shift()) | ((b as u32) << framebuffer.blue_mask_shift());

        Self {
            value
        }
    }

    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        let framebuffer: &Framebuffer = get_framebuffer();
        let value: u32 = ((r as u32) << framebuffer.red_mask_shift()) | ((g as u32) << framebuffer.green_mask_shift()) | ((b as u32) << framebuffer.blue_mask_shift());

        self.value = value;
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

pub fn get_framebuffer() -> &'static Framebuffer<'static> {
    &FRAMEBUFFER
}

pub fn write_pixel(x: u64, y: u64, color: &Color) {
    let framebuffer: &Framebuffer = get_framebuffer();
    let pixel_offset = y * framebuffer.pitch() + x * 4;

    unsafe {
        framebuffer
            .addr()
            .add(pixel_offset as usize)
            .cast::<u32>()
            .write(color.value())
    };
}

pub fn render_rect(x: u64, y: u64, w: u64, h: u64, color: &Color) {
    for x in x..x+w {
        for y in y..y+h {
            write_pixel(x, y, color);
        }
    }
}

pub fn render_char(c: char, x: u64, y: u64, color: &Color, font_size: u64) {
    let glyph = &FONT[((c as u8) - 33) as usize];

    for row in 0..8 {
        let row_bits = (glyph >> ((7 - row) * 8)) & 0xFF; 
        for bit in 0..8 {
            render_rect(x + (bit * font_size), y + (row as u64 * font_size), font_size, font_size, &Color::new(13, 13, 13));
            if (row_bits >> (7 - bit)) & 1 == 1 {
                render_rect(x + (bit * font_size), y + (row as u64 * font_size), font_size, font_size, color);
            }
        }
    }
}

pub fn render_text<T: AsRef<[u8]>>(string: T, x: u64, y: u64, font_size: u64, color: &Color) {
    let offset_add: u64 = (font_size * 8) + 2;

    let mut x_offset: u64 = 0;
    let mut y_offset: u64 = 0;

    let bytes: &[u8] = string.as_ref();

    for b in bytes {
        let c: char = *b as char;
        if (c as u8) > 32 && (c as u8) < 127 {
            render_char(c, x + x_offset, y + y_offset, color, font_size);
            x_offset += offset_add;
        } else if c == '\n' {
            y_offset += offset_add;
            x_offset = 0;
        } else if c == ' ' {
            x_offset += offset_add;
        } else if c == '\t' {
            x_offset += offset_add * 4;
        } else if c == '\0' {
            
        } else {
            let col: Color = Color::new(255, 0, 0);
            // render_text(u8_to_hex(c as u8), x + x_offset, y + y_offset, font_size, &col);
            panic!()
        }
    }
}