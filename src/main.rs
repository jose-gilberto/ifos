#![no_std]          // faz com que o compilador não linke os arquivos da biblioteca padrão do Rust
#![no_main]         // desabilita todos os Rust-level entry-points
#![feature(custom_test_frameworks)]
#![test_runner(if_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rlibc;

use core::panic::PanicInfo;
use if_os::println;

#[no_mangle] // faz com que o nome da função não seja alterado ao compilar
pub extern "C" fn _start() -> ! {
    // essa função será o ponto de entrada, uma vez que o linker procura por uma
    // função com nome _start por padrão
    println!("Hello world{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

// Essa função é chamada em caso de pânico
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Função de pânico para testes
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}