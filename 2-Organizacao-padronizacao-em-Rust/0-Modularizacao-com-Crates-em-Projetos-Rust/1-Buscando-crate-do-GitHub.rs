//! ============================================================================
//! 🦀 BUSCANDO E ADICIONANDO CRATES DO GITHUB NO CARGO (RUST)
//! ============================================================================
//! 
//! Este arquivo explica como importar e utilizar dependências (crates) hospedadas
//! diretamente em repositórios remotos do GitHub/Git no ecossistema Rust.
//!
//! ----------------------------------------------------------------------------
//! 📚 CONCEITO: DEPENDÊNCIAS GIT NO CARGO
//! ----------------------------------------------------------------------------
//! Além de publicar e consumir crates do registro oficial (crates.io), o Cargo
//! permite declarar dependências diretamente de repositórios Git (como o GitHub).
//!
//! Isso é útil em cenários como:
//! 1. Testar uma versão recente/unreleased (em desenvolvimento) da biblioteca.
//! 2. Utilizar um fork privado ou personalizado mantido pela sua equipe.
//! 3. Utilizar bibliotecas internas corporativas hospedadas no GitHub Enterprise.
//! ============================================================================

/// ----------------------------------------------------------------------------
/// 📝 1. CONFIGURAÇÕES NO ARQUIVO Cargo.toml (EXEMPLOS PRÁTICOS)
/// ----------------------------------------------------------------------------
///
/// Abaixo estão as diferentes formas de declarar uma crate do GitHub no seu `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// # 1. Importando diretamente pelo repositório (aponta para a branch padrão main/master)
/// regex_git = { git = "https://github.com/rust-lang/regex" }
///
/// # 2. Especificando uma Branch específica
/// rand_git = { git = "https://github.com/rust-random/rand", branch = "master" }
///
/// # 3. Especificando uma Tag de versão
/// serde_git = { git = "https://github.com/serde-rs/serde", tag = "v1.0.195" }
///
/// # 4. Especificando um Commit exacto (SHA hash)
/// tokio_git = { git = "https://github.com/tokio-rs/tokio", rev = "a1b2c3d4e5f" }
///
/// # 5. Importando um subpacote de um Monorepo/Workspace no GitHub
/// meu_crate_interno = { git = "https://github.com/empresa/monorepo", package = "sub_crate" }
/// ```

/// ----------------------------------------------------------------------------
/// 🛠️ 2. ESTRUTURA DE EXEMPLO DE CÓDIGO RUST UTILIZANDO CRATE DO GITHUB
/// ----------------------------------------------------------------------------
pub mod exemplos_uso {
    /// Simulação de integração com uma Crate importada via GitHub
    pub fn processar_dados_com_crate_git(entrada: &str) -> String {
        println!("[GIT DEPENDENCY]: Processando entrada '{}'...", entrada);
        format!("Processado com sucesso: {}", entrada.to_uppercase())
    }

    /// Exemplo demonstrativo de geração de dados aleatórios (como faria a crate `rand` do GitHub)
    pub fn gerar_token_simulado() -> u32 {
        println!("[GIT DEPENDENCY]: Gerando token via algoritmo aleatório da Crate...");
        42195 // Valor simulado demonstrativo
    }
}

/// ----------------------------------------------------------------------------
/// 🚀 3. PONTO DE ENTRADA (MAIN) DEMONSTRANDO O USO DA CRATE DO GITHUB
/// ----------------------------------------------------------------------------
fn main() {
    println!("=================================================================");
    println!("     IMPORTANDO E UTILIZANDO CRATES DO GITHUB EM RUST            ");
    println!("=================================================================\n");

    let dados = "exemplo de texto para analise";
    let resultado = exemplos_uso::processar_dados_com_crate_git(dados);
    println!("--> Resultado: {}\n", resultado);

    let token = exemplos_uso::gerar_token_simulado();
    println!("--> Token Gerado: {}\n", token);

    println!("=================================================================");
    println!("📌 PASSO A PASSO PARA ADICIONAR UMA CRATE DO GITHUB:");
    println!("-----------------------------------------------------------------");
    println!("1. Abra o arquivo Cargo.toml do seu projeto.");
    println!("2. Na seção [dependencies], adicione a URL do GitHub:");
    println!("   nome_crate = {{ git = \"https://github.com/usuario/repositorio\" }}");
    println!("3. Execute o comando de compilação ou checagem:");
    println!("   $ cargo check   (O Cargo clonará automaticamente o repositório)");
    println!("4. Importe no seu código normalmente:");
    println!("   use nome_crate::minha_funcao;");
    println!("=================================================================");
}
