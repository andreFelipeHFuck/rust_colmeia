use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor_a = String::new();
    let mut valor_b = String::new();
    let mut valor_c = String::new();

    let a: u32;
    let b: u32;
    let c: u32;

    // Valor A
    print!("Entre com o valor de <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<u32>().unwrap();

    // Valor B
    print!("Entre com o valor de <B>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<u32>().unwrap();

    // Valor C
    print!("Entre com o valor de <C>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_c).unwrap();
    c = valor_c.trim().parse::<u32>().unwrap();

    // Maior valor
    print!("Valores na ordem decrescente: ");
    if a > b && a > c {
        if b > c{
            println!("{}, {}, {}", a, b, c);
        }else {
            println!("{}, {}, {}", a, c, b);
        }
    }if b > a && b > c {
        if a > c{
            println!("{}, {}, {}", b, a, c);
        }else {
            println!("{}, {}, {}", b, c, a);
        }
    }if c > a && c > b {
        if a > b {
            println!("{}, {}, {}", c, a, b);
        }else {
            println!("{}, {}, {}", c, b, a);
        }
    }

    println!();
    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}