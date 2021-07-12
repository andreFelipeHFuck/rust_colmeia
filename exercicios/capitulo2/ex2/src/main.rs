
/* 
* Exercíco 2
* Ler uma temperatura em graus Fahrenheit de 64 bits 
* e apresentá-la convertida em graus Fahrenheit
*/

use std::io;
use std::io::prelude::*;

fn main(){
    let mut fahrenheit = String::new();

    let f: f64;
    let c: f64;

    print!("Digite uma temperatura em Graus Fahrenheit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut fahrenheit).unwrap();
    f = fahrenheit.trim().parse::<f64>().unwrap();

// Formula de conversão c = ((f - 32) * 5)/9

c = ((f - 32.) * 5.) / 9.;

println!();
println!("{:8.2}° Fahrenheit são equivalentes a {:8.2}° Celsius", f, c);

println!();
print!("Tecle <Enter> para encerrar...");
io::stdout().flush().unwrap();
io::stdin().read(&mut [0u8]).unwrap();

}