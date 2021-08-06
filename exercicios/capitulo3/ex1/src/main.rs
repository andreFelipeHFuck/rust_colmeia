// Exercício 1

use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor_a = String::new();
    let mut valor_b = String::new();

    // Usando o let Ok não precisa declarar as variáveis
    let r: i64;

    print!("Entre com o valor <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    
    if let Ok(a) = valor_a.trim().parse::<i64>() {

        print!("Entre com valor <B>: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor_b).unwrap();

        if let Ok(b) = valor_b.trim().parse::<i64>() {
            
            if a > b {
                r = a - b;
                println!("{} - {} = {}", a, b, r);
            }else {
                r = b - a;
                println!("{} - {} = {}", b, a, r);
            }


        }else {
            println!("Não entrou valor inteiro em <B>");
        }
    }else {
        println!("Não entrou valor inteiro em <A>");
    }
}
