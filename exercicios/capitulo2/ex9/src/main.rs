// Exercício 9

use std::io;
use std::io::prelude::*;

fn main(){
    let mut salario = String::new();
    let mut percentual = String::new();

    let sa: f32;
    let pr: f32;
    let ns: f32;

    print!("Digite o salário atual R$: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut salario).unwrap();
    sa = salario.trim().parse::<f32>().unwrap();

    print!("Digite o valor percentual de reajuste: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut percentual).unwrap();
    pr = percentual.trim().parse::<f32>().unwrap();

    ns = sa + (sa * (pr/ 100.));

    println!("O valor do novo salário será de R${:.2}", ns);

    println!();
    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}