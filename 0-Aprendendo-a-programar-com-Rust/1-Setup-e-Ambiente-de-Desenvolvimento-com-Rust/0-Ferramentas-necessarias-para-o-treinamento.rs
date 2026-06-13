// Ferramentas necessárias para o treinamento em Rust

/*
Para programar em Rust de forma eficiente, você precisará de algumas ferramentas essenciais que compõem o ecossistema da linguagem.
Abaixo estão as principais, suas funções e exemplos de uso.
*/

// 1. Rustup
// O instalador oficial e gerenciador de versões do Rust. 
// Ele permite instalar, atualizar e alternar entre diferentes versões do compilador (stable, beta, nightly).
/*
Exemplo de comando no terminal:
$ rustup update  // Atualiza o Rust para a versão mais recente
$ rustup toolchain install nightly // Instala a versão experimental
*/

// 2. Rustc (The Rust Compiler)
// É o compilador que transforma seu código .rs em um executável ou biblioteca.
// Embora o Cargo seja mais usado no dia a dia, entender o rustc é fundamental.
/*
Exemplo de uso:
Suponha um arquivo chamado 'ola.rs' com o conteúdo:
fn main() {
    println!("Olá, Rust!");
}

Comando para compilar:
$ rustc ola.rs

Isso gera um executável 'ola' (ou 'ola.exe' no Windows).
$ ./ola
Saída: Olá, Rust!
*/

// 3. Cargo
// É o "canivete suíço" do Rust: gerenciador de pacotes, sistema de compilação e ferramenta de testes.
// Com ele, você gerencia dependências (crates) e automatiza o fluxo de desenvolvimento.
/*
Exemplos de comandos frequentes:
$ cargo new meu_projeto // Cria um novo projeto estruturado
$ cargo build           // Compila o projeto
$ cargo run             // Compila e executa em um só passo
$ cargo check           // Verifica se o código compila sem gerar o executável (muito rápido)
$ cargo test            // Executa os testes automatizados
*/

// 4. Rust-analyzer
// É o servidor de linguagem (LSP) que fornece recursos avançados para editores de código (como VS Code).
// Ele oferece: auto-complete, verificação de erros em tempo real, definições de tipos e refatoração.

fn main() {
    println!("Ferramentas prontas para o treinamento!");
    
    // Exemplo de como o Cargo facilitaria a execução deste código:
    // Se estivéssemos em um projeto Cargo, bastaria um 'cargo run'.
}
