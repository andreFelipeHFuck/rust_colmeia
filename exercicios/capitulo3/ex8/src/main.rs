use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor = String::new();

    let v: i64;

    print!("Digite o valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    v = valor.trim().parse::<i64>().unwrap();

    if !(v > 3){
        println!("{}", v)
    }

    println!();
    print!("Tecle <Enter> para encerrar ..");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}