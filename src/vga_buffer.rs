use core::fmt::{self, Result, Write};
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[allow(unused)]
#[allow(unused_imports)]

use volatile::Volatile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magneta = 5,
    Brown = 6,
    LightGray = 8,
    LightBlue = 9,
    LightCyan = 11,
    LightGreen = 10,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White  = 15,

}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);


impl ColorCode {
    fn new(foreground: Color, background: Color, ) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

const BUFFER_WIDTH:usize = 80;
const BUFFER_HEIGHT:usize = 25;


#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 6,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
        },
    });
}


#[allow(unused)]
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code.0;
                self.buffer.chars[row][col].write(ScreenChar { ascii_character: byte, color_code });
                self.column_position += 1;
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }   

    fn new_line(&mut self) {
       self.column_position = 0;
       if self.row_position < BUFFER_HEIGHT -1 {
        self.row_position +=1;
       } else {
        self.row_position = 0; 
       }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code.0,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

#[allow(unused)]
pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        row_position: 4,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe{ &mut *(0xb8000 as *mut Buffer)},
    };

    
    writeln!(writer, "This is from write macro and the numbers are {}, {}", 42,3.14);

}

#[allow(unused)]
pub fn print_byte(text: u8, position: usize) {
    let mut byte_writer = Writer {
        column_position:position,
        row_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    };

    byte_writer.write_byte(text);
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    } 
}
#[allow(unused)]
pub fn print_string(text: &'static str, position: usize) {

    let mut string_writer = Writer {
        column_position: position,
        row_position: 1,
       color_code: ColorCode::new(Color::Yellow, Color::Black),
       buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},

    };

    for byte in text.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => string_writer.write_byte(byte),
                _ => string_writer.write_byte(0xfe),
            }
    }

}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
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

