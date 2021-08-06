use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor_a = String::new();
    let mut valor_b = String::new();
    let mut escolha = String::new();

    let a: f32;
    let b: f32;
    let mut r: f32 = 0.;
    let opcao: u8;

    print!("Entre o valor <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<f32>().unwrap();

    print!("Entre o valor <B>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_a.trim().parse::<f32>().unwrap();

    println!();
    println!("[1] - Adição");
    println!("[2] - Subtração");
    println!("[3] - Multiplicação");
    println!("[4] - Divisão");

    println!();

    print!("Escolha uma opção: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut escolha).unwrap();
    opcao = escolha.trim().parse::<u8>().unwrap();

    if opcao == 1{
        r = a + b;
    }

    if opcao == 2{
        r = a - b;
    }

    if opcao == 3 {
        r = a * b;
    }

    if opcao == 4 {
        if b == 0. {
            r = 0.;
        }else {
            r = a/b;
        }
    }

    if opcao >= 1 && opcao <= 4{
        print!("\nO resultado da operação equivalente a: {:8.2}", r);
    }else {
        print!("\nOpção inválida.");
    }

    println!();
    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}