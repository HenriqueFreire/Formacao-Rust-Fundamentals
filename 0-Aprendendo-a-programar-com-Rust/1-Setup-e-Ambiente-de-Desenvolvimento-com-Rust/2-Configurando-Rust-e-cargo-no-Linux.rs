// Configurando Rust e Cargo no Linux

/*
Instalar o Rust no Linux é um processo direto através do terminal. A maioria das distribuições Linux requer apenas que você tenha um compilador C (como gcc ou clang) instalado para que o Rust funcione corretamente.
*/

// 1. Pré-requisitos: Build Tools
// Antes de começar, certifique-se de ter as ferramentas de compilação instaladas.
/*
No Ubuntu/Debian:
$ sudo apt update
$ sudo apt install build-essential

No Fedora:
$ sudo dnf groupinstall "Development Tools"

No Arch Linux:
$ sudo pacman -S base-devel
*/

// 2. Instalando via Rustup (Método Recomendado)
// O comando oficial baixa e executa o script de instalação do rustup.
/*
No terminal, execute:
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Siga as instruções na tela e escolha a opção (1) "Proceed with installation (default)".
*/

// 3. Configurando o PATH no Shell
// O instalador geralmente adiciona o Rust ao seu PATH automaticamente no arquivo ~/.profile ou ~/.bashrc.
// Para aplicar as mudanças na sessão atual, você pode rodar:
/*
$ source $HOME/.cargo/env
*/

// 4. Verificando a Instalação
/*
$ rustc --version
$ cargo --version

Exemplo de saída:
rustc 1.70.0 (90c541806 2023-05-31)
cargo 1.70.0 (ec8a8a0ca 2023-05-25)
*/

// 5. Exemplo: Criando um Projeto no Linux
/*
$ cargo new projeto_linux
$ cd projeto_linux
$ cargo run

Isso irá baixar as dependências (se houver), compilar e mostrar:
Hello, world!
*/

// 6. Mantendo o Rust atualizado
/*
Para atualizar para a versão mais recente a qualquer momento:
$ rustup update
*/

fn main() {
    println!("Parabéns! O ambiente Rust está configurado e pronto para uso no Linux.");
}
