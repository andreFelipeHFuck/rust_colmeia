// Exercício 2

use std::io;
use std::io::prelude::*;

fn main(){
    let mut nota1 = String::new();
    let mut nota2 = String::new();
    let mut nota3 = String::new();
    let mut nota4 = String::new();

    let n1: f32;
    let n2: f32;
    let n3: f32;
    let n4: f32;

    let md: f32;

    // Nota 1
    print!("Entre com <Nota1>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota1).unwrap();
    n1 = nota1.trim().parse::<f32>().unwrap();

    // Nota 2
    print!("Entre com <Nota2>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota2).unwrap();
    n2 = nota2.trim().parse::<f32>().unwrap();

    // Nota 3
    print!("Entre com <Nota1>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota3).unwrap();
    n3 = nota3.trim().parse::<f32>().unwrap();

    // Nota 4
    print!("Entre com <Nota1>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota4).unwrap();
    n4 = nota4.trim().parse::<f32>().unwrap();

    md = (n1 + n2 + n3 + n4) / 4.;

    if md > 5. {
        println!("Aprovado com média {}", md);
    }else {
        println!("Reprovado com média {}", md);
    }

    println!();
    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}