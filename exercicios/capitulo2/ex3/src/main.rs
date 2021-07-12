// Exercíco 3

use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor_a = String::new();
    let mut valor_b = String::new();

    let mut a: i32;
    let mut b: i32;
    let troca: i32;

    print!("Digite um número inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<i32>().unwrap();

    println!();

    print!("Digite outro número inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<i32>().unwrap();

    troca = a;
    a = b;
    b = troca;

    println!("A: {}", a);
    println!("B: {}", b);

    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
