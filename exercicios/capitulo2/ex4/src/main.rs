// Exercíco 4

use std::io;
use  std::io::prelude::*;

fn main(){
    let mut comprimento = String::new();
    let mut largura = String::new();
    let mut  altura = String::new();

    let c: f32;
    let l: f32;
    let a: f32;
    let v: f32;

    print!("Digite o comprimento da caixa: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut comprimento).unwrap();
    c = comprimento.trim().parse::<f32>().unwrap();

    

    print!("Digite a largura da caixa: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut largura).unwrap();
    l = largura.trim().parse::<f32>().unwrap();

    

    print!("Digite a altura da caixa: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut altura).unwrap();
    a = altura.trim().parse::<f32>().unwrap();

    println!();
    v = c * l * a;
    println!("O volume da caixa é {} m³", v);

    print!("Tecle <Enter> para terminar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}