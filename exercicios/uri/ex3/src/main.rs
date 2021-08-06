use std::io;
use std::io::prelude::*;

fn main(){
    const PI: f64 = 3.14159;

    let mut raio = String::new();

    let r: f64;
    let a: f64;

    // Entrada
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut raio).unwrap();
    r = raio.trim().parse::<f64>().unwrap();

    a = PI * r.powf(2.);

    println!("A = {:.4}", a);
}
