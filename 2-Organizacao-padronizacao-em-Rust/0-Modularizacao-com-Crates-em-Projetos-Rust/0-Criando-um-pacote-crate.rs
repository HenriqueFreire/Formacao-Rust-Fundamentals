//! ============================================================================
//! 🦀 MODULARIZAÇÃO COM CRATES EM RUST: CRIANDO UM PACOTE (CRATE)
//! ============================================================================
//! 
//! Este arquivo explica os conceitos de Pacotes (Packages), Crates e Módulos em
//! Rust com exemplos práticos compiláveis.
//!
//! ----------------------------------------------------------------------------
//! 📚 CONCEITOS FUNDAMENTAIS:
//! ----------------------------------------------------------------------------
//! 1. **Packages (Pacotes)**:
//!    - É a unidade do Cargo que contém um arquivo `Cargo.toml` descrevendo
//!      como construir uma ou mais Crates.
//!    - Pode conter no máximo UMA Library Crate (`src/lib.rs`) e N QUANTIDADES
//!      de Binary Crates (`src/main.rs` ou `src/bin/*.rs`).
//!
//! 2. **Crates**:
//!    - É a menor unidade de compilação em Rust.
//!    - Pode ser compilada para gerar um executável (**Binary Crate**) ou uma
//!      reutilizável biblioteca de código (**Library Crate**).
//!
//! 3. **Módulos (`mod`)**:
//!    - Organização hierárquica do código dentro de uma Crate, controlando o
//!      escopo e a visibilidade dos itens (funções, structs, enums, etc).
//! ============================================================================

/// ----------------------------------------------------------------------------
/// 🛠️ 1. EXEMPLO DE MÓDULO INTERNO (MODULARIZAÇÃO)
/// ----------------------------------------------------------------------------
pub mod calculadora {
    /// Função pública: Pode ser acessada fora do módulo `calculadora`.
    pub fn somar(a: i32, b: i32) -> i32 {
        log_operacao("Soma", a, b);
        a + b
    }

    pub fn subtrair(a: i32, b: i32) -> i32 {
        log_operacao("Subtração", a, b);
        a - b
    }

    pub fn multiplicar(a: i32, b: i32) -> i32 {
        log_operacao("Multiplicação", a, b);
        a * b
    }

    pub fn dividir(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Erro: Divisão por zero não é permitida!"))
        } else {
            Ok(a / b)
        }
    }

    /// Função privada: Apenas acessível dentro do módulo `calculadora`.
    fn log_operacao(op: &str, a: i32, b: i32) {
        println!("[LOG INTERNO]: Executando {} entre {} e {}", op, a, b);
    }
}

/// ----------------------------------------------------------------------------
/// 📦 2. EXEMPLO DE STRUCTS E STRUCT FIELDS COM VISIBILIDADE (`pub`)
/// ----------------------------------------------------------------------------
pub mod gestao {
    /// Em Rust, os campos de uma struct são PRIVADOS por padrão.
    /// Para torná-los acessíveis externamente, adicione `pub` antes de cada campo.
    pub struct PacoteInfo {
        pub nome: String,
        pub versao: String,
        pub autor: String,
        is_ativo: bool, // Campo privado (só pode ser modificado via métodos internos)
    }

    impl PacoteInfo {
        /// Construtor público (método associado)
        pub fn novo(nome: &str, versao: &str, autor: &str) -> Self {
            PacoteInfo {
                nome: nome.to_string(),
                versao: versao.to_string(),
                autor: autor.to_string(),
                is_ativo: true,
            }
        }

        pub fn exibir_resumo(&self) {
            println!(
                "📦 Pacote: {} | Versão: {} | Autor: {} | Ativo: {}",
                self.nome, self.versao, self.autor, self.is_ativo
            );
        }
    }
}

/// ----------------------------------------------------------------------------
/// 🚀 3. PONTO DE ENTRADA (MAIN) DEMONSTRANDO O USO DA CRATE
/// ----------------------------------------------------------------------------
fn main() {
    println!("=================================================================");
    println!("    DEMONSTRAÇÃO DE PACOTES, CRATES E MÓDULOS EM RUST            ");
    println!("=================================================================\n");

    // 1. Invocando funções do módulo `calculadora`
    let res_soma = calculadora::somar(10, 20);
    println!("resultado da Soma: {}\n", res_soma);

    let res_sub = calculadora::subtrair(50, 15);
    println!("Resultado da Subtração: {}\n", res_sub);

    let res_mult = calculadora::multiplicar(4, 5);
    println!("Resultado da Multiplicação: {}\n", res_mult);

    match calculadora::dividir(10.0, 2.0) {
        Ok(val) => println!("Resultado da Divisão: {}\n", val),
        Err(e) => println!("Error: {}\n", e),
    }

    // 2. Utilizando o módulo `gestao` e instanciando Struct com visibilidade pub
    let pacote = gestao::PacoteInfo::novo(
        "Formacao-Rust-Crate",
        "1.0.0",
        "Henrique Freire",
    );

    pacote.exibir_resumo();

    println!("\n=================================================================");
    println!("📌 GUIA RÁPIDO PARA CRIAR SEU PRÓPRIO PACOTE VIA CARGO:");
    println!("-----------------------------------------------------------------");
    println!("1. Para criar uma Binary Crate (Aplicação Executável):");
    println!("   $ cargo new meu_projeto_binario");
    println!();
    println!("2. Para criar uma Library Crate (Biblioteca Reutilizável):");
    println!("   $ cargo new --lib meu_pacote_lib");
    println!();
    println!("3. Estrutura do arquivo Cargo.toml:");
    println!("   [package]");
    println!("   name = \"meu_pacote_lib\"");
    println!("   version = \"0.1.0\"");
    println!("   edition = \"2024\"");
    println!("=================================================================");
}
