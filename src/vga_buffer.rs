use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
  Black       = 0,
  Blue        = 1,
  Green       = 2,
  Cyan        = 3,
  Red         = 4,
  Magenta     = 5,
  Brown       = 6,
  LightGray   = 7,
  DarkGray    = 8, 
  LightBlue   = 9,
  LightGreen  = 10,
  LightCyan   = 11,
  LightRed    = 12,
  Pink        = 13,
  Yellow      = 14,
  White       = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
  ascii_character: u8,
  color_code: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
  // O tipo Volatile é genérico e pode envolver quase qualquer tipo.
  // Isso garante que não possamos escrever nele acidentalmente por meio de uma gravação normal.
  chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
  column_position: usize,
  color_code: ColorCode,
  buffer: &'static mut Buffer,
}

impl Writer {
  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        // caracteres ASCII que podem ser printados ou uma nova linha
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // caracteres fora do range printável dos ASCII
        _ => self.write_byte(0xfe),
      }
    }
  }
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let color_code = self.color_code;
        // O método write garante que o compilador não ira otimizar essa gravação
        self.buffer.chars[row][col].write(ScreenChar {
          ascii_character: byte,
          color_code,
        });
        self.column_position += 1;
      }
    }
  }
  fn new_line(&mut self) {
    // Iterando sobre os caracteres que estão na tela
    for row in 1..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH {
        // Cada caractere na tela é movido para a linha de cima
        let character = self.buffer.chars[row][col].read();
        self.buffer.chars[row - 1][col].write(character);
      }
    }
    self.clean_row(BUFFER_HEIGHT - 1);
    self.column_position = 0;
  }

  fn clean_row(&mut self, row: usize) {
    let blank = ScreenChar {
      ascii_character: b' ',
      color_code: self.color_code,
    };
    // Limpa uma linha substituindo todos os seus caracteres por um b' ' (espaço em bytes)
    for col in 0..BUFFER_WIDTH {
      self.buffer.chars[row][col].write(blank);
    }
  }
}

use core::fmt;

impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}

// O lazy static permite que nós utilizemos os ponteiros em ma referência estática do Write
lazy_static! {
  // O mutex é utilizado para obter mutabilidade interna sincronizada.
  // Baseado no spinlock
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    // buffer utilizado pelo hardware do VGA
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
  });
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