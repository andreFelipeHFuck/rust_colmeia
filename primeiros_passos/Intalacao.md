# Intalação do Rust

A intalação do Rust é feita através da ferramenta **rustup**, uma ferramentaa de linha de comando para gerenciar versões Rust e ferramentas associadas.

## Intalando rustup no Linux ou macOS

Tanto no Linux quanto no macOS, abra o terminal e digite o seguinte comando:

> $ curl https://sh.rustup.rs -sSf | sh


## Intalando rustup no Windows

No Windows, vá para  https://www.rust-lang.org/pt-BR/tools/install e siga as intruções de instalação do Rust.

## Atualização e Desistalação

 + Comando para atualizar o Rust para a versão mais recente:
 > $ rustup update

 + Comando para desintalar o Rust e o rustup:
 > $ rustup self uninstall

 ## Verificando se Rust foi instalado corretamente

Comando para verificar se Rust foi instalado corretamente:
> $ rustc --version

## Documentação Local
Com a intalação do Rust também vem incluida uma cópia offline da documetação de Rust. Para abrir a documentação execute o seguinte comando:

> rustup doc