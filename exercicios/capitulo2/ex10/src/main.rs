// Exercíco 10

use std::io;
use std::io::prelude::*;

fn main(){
    const pi: f64 = 3.14159265;

    let mut raio = String::new();

    let r: f64;
    let vol: f64;

    print!("Digite o valor do raio: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut raio).unwrap();
    r = raio.trim().parse::<f64>().unwrap();

    vol = pi * r.powf(2.);

    println!("A área de uma circunferência de raio {:.2} é de {:.2}", r, vol);

    println!();
    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}