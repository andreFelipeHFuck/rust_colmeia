use std::io;
use std::io::prelude::*;

fn main() {
    let mut base = String::new();
    let mut indice = String::new();

    let bas: i64;
    let ind: u32;

    print!("Entre o valor da base .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base).unwrap();
    bas = base.trim().parse::<i64>().unwrap();

    print!("Entre o valor do índice ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut indice).unwrap();
    ind = indice.trim().parse::<u32>().unwrap();

    println!();
    println!("Exponenciação = {:8.2}", bas.pow(ind));
    // Para calcular potências de valores inteiros positivos e negativos de 32 bits de uam base real de 64 bits, será necessário uase a função powi()

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
