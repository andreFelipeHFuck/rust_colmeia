use std::io;
use std::io::prelude::*;

fn main() {
    let valor = 10;

    println!("Bínario ......: {:b}", valor);
    println!("Octal ........: {:o}", valor);
    println!("Hexadecimal ..: {:X}", valor);

    println!();
    println!("ou");
    println!();
    println!("BIN: {:b} OCT: {:o} HEX: {:X}", valor, valor, valor);

    // Também pode ser usado a máscaras {:e} e {:E} para anotação científica e {:p} para ponteiros

    println!();
    print!("Tecla <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
