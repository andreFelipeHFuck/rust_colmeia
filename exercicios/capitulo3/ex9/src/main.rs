use std::io;
use std::io::prelude::*;

fn main(){
    let mut numero = String::new();

    let n: u32;
    let m: u32;

    print!("Digite o valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut numero).unwrap();
    n = numero.trim().parse::<u32>().unwrap();

    m = 2 * n;

    if m > 30{
        println!("{}", m);
    }

    println!();
    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}