/* 
* Exercício 1
* Ler temperatura em graus Celsius de 64 bit e apresentá-la
* convertida em graus Fahreheit
*/

use std::io;
use std::io::prelude::*;

fn main(){
    let mut celsius = String::new();

    let c: f64; // Graus Celsius
    let f: f64; // Graus Fahrenheit

    print!("Digite uma temperatura em graus Ceslcius: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut celsius).unwrap();
    c = celsius.trim().parse::<f64>().unwrap();

    // Formula de conversão f = (9 * c + 160)/5

    f = (9. * c + 160.)/5.;

    println!();
    println!("{:8.2}° Celsius é equivalente a {:8.2}° Fahrenheit", c, f);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
