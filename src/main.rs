#![no_std]
#![no_main]

use core::panic::PanicInfo;

// VGA buffer constants
const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// VGA colors
#[allow(dead_code)]
#[repr(u8)]
enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

// Terminal state
struct Terminal {
    row: usize,
    column: usize,
    color: u8,
}

impl Terminal {
    // Create a new terminal and clear the screen
    fn new() -> Terminal {
        let term = Terminal {
            row: 0,
            column: 0,
            color: vga_entry_color(VgaColor::LightGrey, VgaColor::Black),
        };
        term.clear();
        term
    }

    // Clear the screen
    fn clear(&self) {
        let blank = vga_entry(' ' as u8, self.color);
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let offset = y * VGA_WIDTH + x;
                unsafe {
                    *VGA_BUFFER.add(offset) = blank;
                }
            }
        }
    }

    // Write a single character
    fn put_char(&mut self, c: char) {
        match c {
            '\n' => {
                self.column = 0;
                self.row += 1;
                if self.row >= VGA_HEIGHT {
                    self.row = 0;
                }
            }
            _ => {
                let index = self.row * VGA_WIDTH + self.column;
                unsafe {
                    *VGA_BUFFER.add(index) = vga_entry(c as u8, self.color);
                }
                self.column += 1;
                if self.column >= VGA_WIDTH {
                    self.column = 0;
                    self.row += 1;
                    if self.row >= VGA_HEIGHT {
                        self.row = 0;
                    }
                }
            }
        }
    }

    // Write a string
    fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            self.put_char(c);
        }
    }
}

// Helper functions for VGA entries
fn vga_entry_color(fg: VgaColor, bg: VgaColor) -> u8 {
    (bg as u8) << 4 | (fg as u8)
}

fn vga_entry(c: u8, color: u8) -> u16 {
    (color as u16) << 8 | (c as u16)
}

// Entry point - this uses the bootloader crate's macros
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut terminal = Terminal::new();
    terminal.write_string("Finally, we can see something in Rust!\n");

    loop {}
}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
