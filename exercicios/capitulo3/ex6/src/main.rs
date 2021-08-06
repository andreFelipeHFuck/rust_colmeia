use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor_a = String::new();
    let mut valor_b = String::new();
    let mut valor_c = String::new();
    let mut valor_d = String::new();

    let a: i64;
    let b: i64;
    let c: i64;
    let d: i64;

    print!("Entre com o valor <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<i64>().unwrap();

    print!("Entre com o valor <B>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<i64>().unwrap();

    print!("Entre com o valor <C>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_c).unwrap();
    c = valor_c.trim().parse::<i64>().unwrap();

    print!("Entre com o valor <D>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_d).unwrap();
    d = valor_d.trim().parse::<i64>().unwrap();

    println!();

    println!("Os valores divisiveis por 2 ou 3 são: ");

    if a % 2 == 0 || a % 3 == 0{
        println!("{}", a);
    }

    if b % 2 == 0 || b % 3 == 0 {
        println!("{}", b);
    }

    if c % 2 == 0 || c % 3 == 0{
        println!("{}", c);
    }

    if d % 2 == 0 || d % 3 == 0{
        println!("{}", d);
    }

    println!();
    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
    
}