// Exercíco 6

use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor_a = String::new();
    let mut valor_b = String::new();

    let a: i32;
    let b: i32;
    let quadrado_da_diferenca: i32;

    print!("Digite o primeiro valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<i32>().unwrap();

    print!("Digite o segundo valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<i32>().unwrap();

    quadrado_da_diferenca = (a - b).pow(2);

    print!("O quadrado da diferença ente {} e {} é {}", a, b, quadrado_da_diferenca);

    println!();
    print!("Tecle <Enter> para finalizar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}