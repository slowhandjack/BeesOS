use core::fmt;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // Store each colour as a single byte, not the default size 
pub enum Colour{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8); // ColourCode contains the full colour byte, which is made up of a foreground and a background colour.

impl ColourCode {
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;


#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}


impl Writer {
    pub fn write_byte(& mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte  => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let colour_code = self.colour_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    colour_code,
                });

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH{
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };

        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline character
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
}


impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result{
        self.write_string(s);
        Ok(())
    }
}


pub fn print_something(){
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    

    write!(writer, "The numbers are {} and {}", 42, 1.8/3.5).unwrap();
}


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
    buffer:unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


// Add this impl block **below** your existing Writer impls:

impl Writer {
    /// Write `s` starting at the given (row, col).
    /// Does not move `column_position`.
    pub fn write_at(&mut self, row: usize, col: usize, s: &str) {
        let mut x = col;
        for &b in s.as_bytes() {
            if x >= BUFFER_WIDTH { break; }
            let colour_code = self.colour_code;
            self.buffer.chars[row][x].write(ScreenChar {
                ascii_character: b,
                colour_code,
            });
            x += 1;
        }
    }

    /// Clear a rectangular region by writing spaces
    pub fn clear_region(&mut self, row_start: usize, rows: usize, col_start: usize, cols: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };
        for r in row_start..row_start + rows {
            for c in col_start..col_start + cols {
                if r < BUFFER_HEIGHT && c < BUFFER_WIDTH {
                    self.buffer.chars[r][c].write(blank);
                }
            }
        }
    }
}