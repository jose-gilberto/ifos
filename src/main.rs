#![no_std]          // faz com que o compilador não linke os arquivos da biblioteca padrão do Rust
#![no_main]         // desabilita todos os Rust-level entry-points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // faz com que o nome da função não seja alterado ao compilar
pub extern "C" fn _start() -> ! {
    // essa função será o ponto de entrada, uma vez que o linker procura por uma
    // função com nome _start por padrão
    
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

extern crate rlibc;