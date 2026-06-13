// Configurando Rust e Cargo no macOS

/*
A instalação do Rust no macOS é muito semelhante à do Linux, utilizando o terminal. O principal requisito é ter as ferramentas de linha de comando do Xcode instaladas.
*/

// 1. Pré-requisitos: Xcode Command Line Tools
// O macOS precisa do linker e de outras ferramentas de compilação fornecidas pela Apple.
/*
Abra o Terminal e execute:
$ xcode-select --install

Uma janela aparecerá solicitando a confirmação da instalação.
*/

// 2. Instalando via Rustup
// Utilize o comando curl oficial para baixar o script de instalação.
/*
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Escolha a opção (1) "Proceed with installation (default)".
*/

// 3. Configurando o Ambiente
// O instalador modificará seu arquivo ~/.zshrc (padrão no macOS moderno) ou ~/.bash_profile.
// Para ativar o Rust imediatamente no terminal atual:
/*
$ source $HOME/.cargo/env
*/

// 4. Verificando a Instalação
/*
$ rustc --version
$ cargo --version

Exemplo de saída esperada:
rustc 1.x.y (hash data)
cargo 1.x.y (hash data)
*/

// 5. Exemplo de Uso: Criando um App no Mac
/*
$ cargo new hello_mac
$ cd hello_mac
$ cargo run

Isso compilará o código para a arquitetura do seu Mac (Intel ou Apple Silicon/M1/M2/M3).
*/

// 6. Homebrew (Opcional)
// Embora o rustup seja o recomendado, você também pode instalar via Homebrew, 
// mas o rustup oferece melhor controle sobre as versões do compilador.
/*
$ brew install rust
*/

fn main() {
    println!("Ambiente configurado com sucesso! Bem-vindo ao Rust no macOS.");
}
