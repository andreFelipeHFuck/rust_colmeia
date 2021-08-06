use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor_a = String::new();
    let mut  valor_b = String::new();

    let a: i16;
    let b: i16;
    let x: i16;

    // Valor A
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<i16>().unwrap();

    // Valor B
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<i16>().unwrap();

    x = a + b;

    println!("X = {}", x);
}
