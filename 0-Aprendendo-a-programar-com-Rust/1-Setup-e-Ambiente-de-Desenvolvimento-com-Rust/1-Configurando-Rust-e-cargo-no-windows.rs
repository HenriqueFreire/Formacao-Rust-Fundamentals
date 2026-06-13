// Configurando Rust e Cargo no Windows

/*
A instalação do Rust no Windows é simplificada pelo uso do rustup, mas requer a atenção a alguns pré-requisitos do sistema, como as ferramentas de compilação do C++.
*/

// 1. Pré-requisitos: Build Tools para Visual Studio
// O Rust no Windows geralmente utiliza o linker do C++. 
// Você deve instalar o "Visual Studio Community" e selecionar a carga de trabalho:
// "Desenvolvimento para desktop com C++" (Desktop development with C++).

// 2. Instalando via Rustup
// Acesse https://rustup.rs e baixe o instalador 'rustup-init.exe'.
/*
Ao executar o instalador no PowerShell ou CMD:
- Selecione a opção (1) "Proceed with installation (default)".
- Isso instalará a versão estável (stable) do Rust, o compilador rustc e o gerenciador Cargo.
*/

// 3. Verificando as Variáveis de Ambiente (PATH)
// O instalador adiciona automaticamente o diretório binário do Cargo ao seu PATH:
// %USERPROFILE%\.cargo\bin
/*
Exemplo para verificar no PowerShell:
$ env:Path -split ';' | Select-String "cargo"
*/

// 4. Testando a Instalação
// Abra um novo terminal (PowerShell ou CMD) e execute:
/*
$ rustc --version
$ cargo --version

Se ambos retornarem a versão (ex: rustc 1.x.y), a configuração foi bem-sucedida.
*/

// 5. Exemplo de Criação de Projeto no Windows
/*
No PowerShell:
$ cd ~/Documents
$ cargo new hello_windows
$ cd hello_windows
$ cargo run

Saída esperada:
   Compiling hello_windows v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in ...s
     Running `target\debug\hello_windows.exe`
Hello, world!
*/

// Dica: Se preferir usar o WSL (Windows Subsystem for Linux), 
// siga os mesmos passos da instalação para Linux dentro do terminal da sua distro.

fn main() {
    println!("Configuração concluída! Agora você pode usar Rust nativamente no Windows.");
}
