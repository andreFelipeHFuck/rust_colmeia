use std::io;
use std::io::prelude::*;

fn main(){
    let mut nota1 = String::new();
    let mut nota2 = String::new();
    let mut nota3 = String::new();
    let mut nota4 = String::new();
    let mut nota_exame = String::new();

    let n1: f32;
    let n2: f32;
    let n3: f32;
    let n4: f32;

    let md1: f32;
    let md2: f32;
    let ne: f32;

    // Nota 1
    print!("Entre com a <Nota 1>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota1).unwrap();
    n1 = nota1.trim().parse::<f32>().unwrap();

    // Nota 2
    print!("Entre com a <Nota 2>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota2).unwrap();
    n2 = nota2.trim().parse::<f32>().unwrap();

    // Nota 3
    print!("Entre com a <Nota 3>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota3).unwrap();
    n3 = nota3.trim().parse::<f32>().unwrap();

    // Nota 4
    print!("Entre com a <Nota 4>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nota4).unwrap();
    n4 = nota4.trim().parse::<f32>().unwrap();

    md1 = (n1 + n2 + n3 + n4)/4.;

    if md1 >= 7. {
        println!("Aprovado com a média {:.2}", md1);
    }else {
        println!("Você ficou em exame.");
        print!("Entre com a nota de exame: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nota_exame).unwrap();
        ne = nota_exame.trim().parse::<f32>().unwrap();

        md2 = (md1 + ne)/2.;

        if md2 >= 5. {
            println!("Aprovado com exame, média: {:.2}", md2);
        }else {
            print!("Reprovado com exame, média: {:.2}", md2);
        }

    }

    println!();
    print!("Tecle <Enter> para encerrar ...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}