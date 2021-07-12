// Exercício 5

use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor = String::new();

    let v_64: i64;
    let v_32: i32;

    print!("Digite um valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    v_64 = valor.trim().parse::<i64>().unwrap();
    v_32 = valor.trim().parse::<i32>().unwrap();

    println!("64 bits: {}", v_64);
    println!("32 bits elevado ao quadrado: {}", v_32.pow(2));

    println!();

    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}