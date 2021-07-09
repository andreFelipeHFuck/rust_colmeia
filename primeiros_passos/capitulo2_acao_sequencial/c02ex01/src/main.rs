use std::io;
use std::io::prelude::*;
// use faz chamada dos bibliotecas de auxílio a linguagem

fn main(){
    println!("Programação na linguagem Rust.\n");
    print!("Tecla <Enter> para encerrar...");
    io::stdout().flush().unwrap(); 
    //io::stdout é responsavel por definir e direcionar para a saída-padrão
    //flush() é responsável por garantir que a saída seja enviada imediatamente ao periférico-padrão de saída conectado ao computador
    //unwrap faz com que a ação da intrução seja executada da forma mais rápida pissível e da maneira mais simples possível
    io::stdin().read(&mut [0u8]).unwrap(); // Efetua a leitura de apenas um caractere
    // &mut retorno de uma refêrancia mutável
    // io::stdin() indica o uso do periférico-padrão e entrada
}