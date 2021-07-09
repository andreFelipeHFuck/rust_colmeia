use std::io;
use std::io::prelude::*;

fn main() {
    let mut nome = String::new(); // variável mutável
    // new(): estabelece em memória uma instância de trabalho para a variável nome

    print!("Informe seu nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nome).expect("Entrada incorreta"); // Efetua a entrada de caracteres informados no teclado junto com à variável NOME
    // A leitura dos dados é feita pelo método read_line()

    if let Some('\n') = nome.chars().next_back(){
        nome.pop();
    }

    if let Some('\r') = nome.chars().next_back(){
        nome.pop();
    }

    println!("Olá, {}\n", nome);
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
