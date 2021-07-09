use std::io;
use std::io::prelude::*;

fn main() {
    let mut valor = String::new();
    // A linguagem Rust só aceita entrada de dados que sejam do tipo caractere alfanúmerico
    let vlr:i64;

    print!("Informe um valor inteiro: ");
    io::stdout().flush().unwrap(); // Entrada da variável valor
    io::stdin().read_line(&mut valor).unwrap();

    vlr = valor.trim().parse::<i64>().unwrap(); // Convertendo de String para Inteiro
    // trim(): retira qualquer espaço em branco que tenha sido eventualmente fornecido na entrada
    // parse: converção do tipo


    println!("Resultado = {}", vlr);

    println!();
    print!("Tecla <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
