// Exercício 11

use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor_a = String::new();
    let mut valor_b = String::new();

    let a: i64;
    let b: i64;

    print!("Digite o valor de A: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<i64>().unwrap();

    print!("Digite o valor de B: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<i64>().unwrap();

    println!("Soma: {} + {} = {}", a, b, a + b);
    println!("Subtração: {} - {} = {}", a, b, a - b);
    println!("Multiplicação: {} * {} = {}", a, b, a * b);
    println!("Divisão: {} / {} = {}", a, b, a / b);

    println!();
    print!("Tecle <Enter> para encerrar ..");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}