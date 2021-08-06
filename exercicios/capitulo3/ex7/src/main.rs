use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor_a = String::new();
    let mut valor_b = String::new();
    let mut valor_c = String::new();
    let mut valor_d = String::new();
    let mut valor_e = String::new();

    let a: i32;
    let b: i32;
    let c: i32;
    let d: i32;
    let e: i32;

    let mut maior: i32;
    let mut menor: i32;

    print!("Entre com o valor de <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<i32>().unwrap();

    print!("Entre com o valor de <B>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<i32>().unwrap();

    print!("Entre com o valor de <C>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_c).unwrap();
    c = valor_c.trim().parse::<i32>().unwrap();

    print!("Entre com o valor de <D>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_d).unwrap();
    d = valor_d.trim().parse::<i32>().unwrap();

    print!("Entre com o valor de <E>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_e).unwrap();
    e = valor_e.trim().parse::<i32>().unwrap();


    maior = a;
    menor = a;

    // Maior

    if a > maior{
        maior = a;
    }if b > maior {
        maior = b;
    }if c > maior {
        maior = c;
    }if d > maior {
        maior = d;
    }if e > maior {
        maior = e;
    }

    // Menor

    if a < menor{
        menor = a;
    }if b < menor {
        menor = b;
    }if c < menor{
        menor = c;
    }if d < menor {
        menor = d;
    }if e < menor {
        menor = e;
    }

    println!();
    println!("O maior valor é {}", maior);
    println!("O menor valor é {}", menor);

    println!();
    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}