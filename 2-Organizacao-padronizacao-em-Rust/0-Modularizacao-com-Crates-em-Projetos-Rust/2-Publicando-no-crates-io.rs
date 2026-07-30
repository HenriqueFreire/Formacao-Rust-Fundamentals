//! ============================================================================
//! 🦀 PUBLICANDO CRATES NO REGISTRO OFICIAL CRATES.IO (RUST)
//! ============================================================================
//! 
//! Este arquivo explica o passo a passo completo, metadados necessários e boas
//! práticas para publicar sua biblioteca de código no registro oficial do Rust: `crates.io`.
//!
//! ----------------------------------------------------------------------------
//! 📚 O QUE É O CRATES.IO?
//! ----------------------------------------------------------------------------
//! O `crates.io` é o registro central da comunidade de código aberto do Rust,
//! onde desenvolvedores compartilham pacotes reutilizáveis (crates).
//!
//! IMPORTANTES REGRAS DO CRATES.IO:
//! 1. **Imutabilidade**: Uma versão publicada NUNCA pode ser alterada ou apagada
//!    (para garantir reproduzibilidade de builds no mundo todo).
//! 2. **Nome Único**: O nome da crate no `Cargo.toml` não pode já existir no site.
//! 3. **Licença e Descrição**: São obrigatórias para publicação.
//! ============================================================================

/// ----------------------------------------------------------------------------
/// 📝 1. CONFIGURAÇÃO DE METADADOS OBRIGATÓRIOS NO Cargo.toml
/// ----------------------------------------------------------------------------
///
/// Para que o `cargo publish` aceite publicar sua Crate, seu `Cargo.toml` deve
/// conter metadados completos:
///
/// ```toml
/// [package]
/// name = "minha_biblioteca_exemplo_rust"  # Deve ser um nome ÚNICO no crates.io
/// version = "0.1.0"                        # Deve seguir o padrão SemVer (MAJOR.MINOR.PATCH)
/// edition = "2024"
/// authors = ["Henrique Freire <henrique@exemplo.com>"]
/// description = "Uma biblioteca utilitária em Rust desenvolvida na Formação Rust Fundamentals."
/// license = "MIT OR Apache-2.0"            # Especificação de licença válida (SPDX)
/// readme = "README.md"                     # Arquivo README que será exibido no crates.io
/// repository = "https://github.com/HenriqueFreire/Formacao-Rust-Fundamentals"
/// homepage = "https://github.com/HenriqueFreire/Formacao-Rust-Fundamentals"
/// documentation = "https://docs.rs/minha_biblioteca_exemplo_rust"
/// keywords = ["utilitarios", "formacao", "dio", "console"]
/// categories = ["command-line-utilities", "development-tools"]
/// ```

/// ----------------------------------------------------------------------------
/// 🛠️ 2. EXEMPLO DE BIBLIOTECA REUTILIZÁVEL PRONTA PARA PUBLICAÇÃO (src/lib.rs)
/// ----------------------------------------------------------------------------
pub mod formatadores {
    /// Formata um valor numérico para a moeda brasileira (BRL).
    ///
    /// # Exemplos
    /// ```
    /// let preco = 1250.5;
    /// let formatado = formatadores::formatar_moeda(preco);
    /// assert_eq!(formatado, "R$ 1250.50");
    /// ```
    pub fn formatar_moeda(valor: f64) -> String {
        format!("R$ {:.2}", valor)
    }

    /// Limpa e formata uma string removendo espaços extras.
    pub fn limpar_texto(texto: &str) -> String {
        texto.trim().to_string()
    }
}

/// ----------------------------------------------------------------------------
/// 🚀 3. PONTO DE ENTRADA (MAIN) DEMONSTRANDO O PASSO A PASSO DA PUBLICAÇÃO
/// ----------------------------------------------------------------------------
fn main() {
    println!("=================================================================");
    println!("        GUIA PASSO A PASSO PARA PUBLICAR NO CRATES.IO            ");
    println!("=================================================================\n");

    // Demonstração da função exportada pela biblioteca
    let preco_formatado = formatadores::formatar_moeda(99.9);
    println!("--> Exemplo de código da Crate: Preço = {}\n", preco_formatado);

    println!("=================================================================");
    println!("📋 ETAPAS DE PUBLICAÇÃO VIA LINHA DE COMANDO (TERMINAL):");
    println!("-----------------------------------------------------------------");
    println!("PASSO 1: Criar uma conta em https://crates.io (via GitHub)");
    println!("PASSO 2: Gerar um API Token no seu perfil do crates.io");
    println!("PASSO 3: Autenticar a ferramenta Cargo com seu Token:");
    println!("   $ cargo login cio_api_token_seu_codigo_aqui");
    println!();
    println!("PASSO 4: Testar o empacotamento localmente (Dry Run):");
    println!("   $ cargo package");
    println!("   (Gera o arquivo .crate compactado e valida se falta algum metadado)");
    println!();
    println!("PASSO 5: Publicar oficialmente no crates.io:");
    println!("   $ cargo publish");
    println!();
    println!("PASSO 6: Em caso de necessidade de descontinuar uma versão (Yank):");
    println!("   $ cargo yank --vers 0.1.0");
    println!("=================================================================");
}
