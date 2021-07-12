// Exercíco 13 :V

use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor = String::new();

    let v: i32;

    print!("Digite um número inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    v = valor.trim().parse::<i32>().unwrap();

    println!("O sucessor de {} é {}", v, v + 1);
    println!("O antecessor de {} é {}", v, v - 1);

    println!();
    print!("Tecle <Enter> para encerrar ..");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}