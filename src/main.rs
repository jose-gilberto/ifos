#![no_std]          // faz com que o compilador não linke os arquivos da biblioteca padrão do Rust
#![no_main]         // desabilita todos os Rust-level entry-points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // faz com que o nome da função não seja alterado ao compilar
pub extern "C" fn _start() -> ! {
    // essa função será o ponto de entrada, uma vez que o linker procura por uma
    // função com nome _start por padrão
    loop {}
}

