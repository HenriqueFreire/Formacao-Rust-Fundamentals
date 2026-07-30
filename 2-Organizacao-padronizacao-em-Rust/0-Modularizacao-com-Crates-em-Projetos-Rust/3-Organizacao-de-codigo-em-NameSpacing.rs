//! ============================================================================
//! 🦀 ORGANIZAÇÃO DE CÓDIGO COM NAMESPACING EM RUST
//! ============================================================================
//! 
//! Este arquivo explica como organizar código em Namespaces utilizando Módulos (`mod`),
//! caminhos absolutos e relativos (`crate`, `super`, `self`), re-exportações (`pub use`)
//! e aliases de tipos para evitar colisões de nomes.
//!
//! ----------------------------------------------------------------------------
//! 📚 O QUE É NAMESPACING EM RUST?
//! ----------------------------------------------------------------------------
//! Diferente de linguagens como C++ ou C#, Rust não possui a palavra-chave `namespace`.
//! Em Rust, a própria estrutura de **Módulos (`mod`)** funciona como o mecanismo de
//! Namespacing nativo da linguagem.
//!
//! VANTAGENS DO NAMESPACING:
//! 1. Evita conflitos de nomes (ex: duas structs chamadas `Conexao` em locais diferentes).
//! 2. Agrupa funções, structs, enums e constantes por contexto/domínio.
//! 3. Proporciona controle refinado sobre o que é público (`pub`) ou privado.
//! ============================================================================

/// ----------------------------------------------------------------------------
/// 📁 1. DEFINIÇÃO DE NAMESPACES HIERÁRQUICOS (MÓDULOS ANINHADOS)
/// ----------------------------------------------------------------------------
pub mod banco_de_dados {
    // Namespace interno: PostgreSQL
    pub mod postgres {
        pub struct Conexao {
            pub string_conexao: String,
        }

        impl Conexao {
            pub fn conectar(url: &str) -> Self {
                println!("[PostgreSQL Namespace]: Conectando em {}...", url);
                Conexao {
                    string_conexao: url.to_string(),
                }
            }
        }
    }

    // Namespace interno: SQLite
    pub mod sqlite {
        pub struct Conexao {
            pub caminho_arquivo: String,
        }

        impl Conexao {
            pub fn conectar(caminho: &str) -> Self {
                println!("[SQLite Namespace]: Abrindo banco local em {}...", caminho);
                Conexao {
                    caminho_arquivo: caminho.to_string(),
                }
            }
        }
    }
}

/// ----------------------------------------------------------------------------
/// 🔄 2. RE-EXPORTAÇÃO DE NAMESPACES (`pub use` - PATTERN FAÇADE)
/// ----------------------------------------------------------------------------
/// A re-exportação permite oferecer uma API externa limpa e simplificada,
/// escondendo a complexidade de submódulos aninhados.
pub mod api_facil {
    // Re-exporta os componentes principais para o nível raiz do namespace `api_facil`
    pub use super::banco_de_dados::postgres::Conexao as ConexaoPostgres;
    pub use super::banco_de_dados::sqlite::Conexao as ConexaoSqlite;
}

/// ----------------------------------------------------------------------------
/// 🚀 3. PONTO DE ENTRADA (MAIN) DEMONSTRANDO O USO DE NAMESPACES
/// ----------------------------------------------------------------------------
fn main() {
    println!("=================================================================");
    println!("         DEMONSTRAÇÃO DE NAMESPACING EM RUST (`mod`)             ");
    println!("=================================================================\n");

    // 1. Resolução de nomes via caminhos explícitos (Evitando Conflito de Nomes)
    println!("--- 1. Usando Namespaces Explícitos ---");
    let conn_pg = banco_de_dados::postgres::Conexao::conectar("postgres://localhost:5432/meudb");
    println!("Conexão Postgres criada: {}\n", conn_pg.string_conexao);

    let conn_lite = banco_de_dados::sqlite::Conexao::conectar("/tmp/dados.db");
    println!("Conexão SQLite criada: {}\n", conn_lite.caminho_arquivo);

    // 2. Resolução de Conflitos usando Aliasing (`use ... as ...`)
    println!("--- 2. Importação com Aliases para Evitar Colisão de Nomes ---");
    use banco_de_dados::postgres::Conexao as PgConexao;
    use banco_de_dados::sqlite::Conexao as LiteConexao;

    let _pg = PgConexao::conectar("postgres://prod:5432/app");
    let _lite = LiteConexao::conectar("./local.db");

    // 3. Utilizando o Namespace Re-exportado (Façade API)
    println!("\n--- 3. Usando API Re-exportada (Façade Pattern) ---");
    let _facade_pg = api_facil::ConexaoPostgres::conectar("postgres://facade:5432/db");

    println!("\n=================================================================");
    println!("📌 RESUMO DE BOAS PRÁTICAS EM NAMESPACING RUST:");
    println!("-----------------------------------------------------------------");
    println!("1. Use `mod nome_dominio` para criar um namespace lógico.");
    println!("2. Use `use caminho::item as Alias` para desambiguar tipos iguais.");
    println!("3. Use `pub use` para criar APIs externas limpas e organizadas.");
    println!("4. Prefira `crate::...` para caminhos absolutos dentro da mesma crate.");
    println!("5. Use `super::...` para acessar o namespace pai imediato.");
    println!("=================================================================");
}
