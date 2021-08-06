use std::io;
use std::io::prelude::*;

fn main(){
    let mut tabuada = String::new();

    let n: u8;
    let mut i: u8;
    let mut r: u8;

    print!("Entre o valor da tabuada: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut tabuada).unwrap();
    n = tabuada.trim().parse::<u8>().unwrap();

    println!();

    i = 1;
    while !(i > 10) {
        r = n * i;
        println!("{:2} x {:2} = {:3}",n, i, r);
        i += 1;
    }

    println!();
    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}