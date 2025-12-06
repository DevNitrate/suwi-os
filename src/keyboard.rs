use core::arch::asm;

fn inb(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!(
            "in al, dx",
            out("al") value,
            in("dx") port,
            options(nomem, nostack)
        )
    }

    value
}

static mut SHIFT: bool = false;

pub fn read_key() -> u8 {
    loop {
        let status = inb(0x64);

        if status & 1 != 0 {
            let c: u8 = inb(0x60);
            if c == 0x2A {
                unsafe { SHIFT = true; }
                continue;
            } else if c == 0xAA {
                unsafe { SHIFT = false; }
                continue;
            }
            let mut res =  match c {
                0x10 => b'a',
                0x11 => b'z',
                0x12 => b'e',
                0x13 => b'r',
                0x14 => b't',
                0x15 => b'y',
                0x16 => b'u',
                0x17 => b'i',
                0x18 => b'o',
                0x19 => b'p',
                0x1E => b'q',
                0x1F => b's',
                0x20 => b'd',
                0x21 => b'f',
                0x22 => b'g',
                0x23 => b'h',
                0x24 => b'j',
                0x25 => b'k',
                0x26 => b'l',
                0x27 => b'm',
                0x2C => b'w',
                0x2D => b'x',
                0x2E => b'c',
                0x2F => b'v',
                0x30 => b'b',
                0x31 => b'n',
                0x32 => b',',
                0x33 => b';',
                0x34 => b':',
                0x35 => b'!',
                _ => b'\0'
            };

            if unsafe { SHIFT == true } && (res >= b'a' &&  res <= b'z') {
                res -= 32;
            }

            return res;

            return c;
        }
    }
}