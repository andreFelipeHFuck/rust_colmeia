// Exercício 8

use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor_a = String::new();
    let mut valor_b = String::new();
    let mut valor_c = String::new();
    
    let a: i32;
    let b: i32;
    let c: i32;
    let soma: i32;

    print!("Digite o valor de A: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<i32>().unwrap();

    print!("Digite o valor de B: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<i32>().unwrap();

    print!("Digite o valor de C: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_c).unwrap();
    c = valor_c.trim().parse::<i32>().unwrap();

    soma = a.pow(2) + b.pow(2) + c.pow(2);

    println!("A somas dos quadrados de {}, {} e {} é igual a {}", a, b, c, soma);

    println!();
    print!("Tecle <Enter> para encerrar .. .");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}