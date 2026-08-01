// Erros Customizados Envelopados e Encadeamento de Causas (Error Chaining / `source()`)

/*
Quando desenvolvemos módulos e aplicações complexas em Rust, frequentemente capturamos
erros de baixo nível (como I/O de arquivo ou conversão de texto) e os encapsulamos em um
erro de alto nível que faz sentido para o nosso domínio (ex: `ErroConfiguracao`, `ErroConexao`).

Para manter a rastreabilidade da falha sem perder o motivo original, utilizamos a funcionalidade
de **fonte de erro** (*Error Source* ou *Error Chaining*).

Isso é feito implementando o método `source()` da trait `std::error::Error`:

fn source(&self) -> Option<&(dyn Error + 'static)> {
    // Retorna uma referência para o erro subjacente (causa raiz), se existir
}
*/

use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

// ============================================================================
// 1. Definindo o Erro de Alto Nível (Dominio: Sistema de Configurações)
// ============================================================================

#[derive(Debug)]
enum ErroConfiguracao {
    // Envelopa o erro de I/O de arquivo
    FalhaLeituraArquivo(io::Error),
    
    // Envelopa o erro de parse de inteiros
    FalhaParsePorta(ParseIntError),
    
    // Envelopa qualquer outro erro dinamicamente (usando Box<dyn Error>)
    ErroGenerico {
        mensagem: String,
        fonte: Box<dyn Error + 'static>,
    },
    
    // Um erro próprio de validação lógica que NÃO tem fonte externa
    PortaInvalida(u16),
}

// 2. Implementação de Display para o Erro Customizado
impl fmt::Display for ErroConfiguracao {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErroConfiguracao::FalhaLeituraArquivo(_) => {
                write!(f, "Erro de Configuração: Não foi possível ler o arquivo de configuração")
            }
            ErroConfiguracao::FalhaParsePorta(_) => {
                write!(f, "Erro de Configuração: A porta especificada no arquivo é inválida")
            }
            ErroConfiguracao::ErroGenerico { mensagem, .. } => {
                write!(f, "Erro de Configuração Geral: {}", mensagem)
            }
            ErroConfiguracao::PortaInvalida(porta) => {
                write!(f, "Erro de Configuração: A porta {} está fora do intervalo permitido (1024-65535)", porta)
            }
        }
    }
}

// 3. Implementação da Trait std::error::Error RETORNANDO O ERRO FONTE (source)
impl Error for ErroConfiguracao {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // Retornamos a referência para o erro original encapsulado
            ErroConfiguracao::FalhaLeituraArquivo(err_io) => Some(err_io),
            ErroConfiguracao::FalhaParsePorta(err_parse) => Some(err_parse),
            
            // Para o Box<dyn Error>, desreferenciamos e convertemos a referência
            ErroConfiguracao::ErroGenerico { fonte, .. } => Some(fonte.as_ref()),
            
            // Erros puramente lógicos não possuem fonte subjacente
            ErroConfiguracao::PortaInvalida(_) => None,
        }
    }
}

// 4. Trait From para automação com o operador '?'
impl From<io::Error> for ErroConfiguracao {
    fn from(err: io::Error) -> Self {
        ErroConfiguracao::FalhaLeituraArquivo(err)
    }
}

impl From<ParseIntError> for ErroConfiguracao {
    fn from(err: ParseIntError) -> Self {
        ErroConfiguracao::FalhaParsePorta(err)
    }
}

// ============================================================================
// 2. Funções de Simulação que Geram Erros Envelopados
// ============================================================================

// Simula a leitura de uma porta a partir do conteúdo de um arquivo
fn carregar_porta_servidor(conteudo_simulado: &str) -> Result<u16, ErroConfiguracao> {
    // Se o conteúdo estiver vazio, simula um erro de I/O
    if conteudo_simulado.is_empty() {
        let err_io = io::Error::new(io::ErrorKind::NotFound, "Arquivo 'config.txt' não encontrado");
        return Err(ErroConfiguracao::FalhaLeituraArquivo(err_io));
    }

    // O '?' usará From<ParseIntError> para envelopar em ErroConfiguracao::FalhaParsePorta
    let porta: u16 = conteudo_simulado.trim().parse()?;

    if porta < 1024 {
        return Err(ErroConfiguracao::PortaInvalida(porta));
    }

    Ok(porta)
}

// ============================================================================
// 3. Função Auxiliar para Inspecionar e Imprimir a Cadeia Completa de Erros
// ============================================================================

fn imprimir_cadeia_de_erros(erro: &(dyn Error + 'static)) {
    println!(" [ERRO PRINCIPAL]: {}", erro);
    
    let mut causa_atual = erro.source();
    let mut nivel = 1;

    while let Some(causa) = causa_atual {
        println!("   └── [CAUSA RAIZ {}]: {}", nivel, causa);
        causa_atual = causa.source(); // Caminha para a próxima causa se houver
        nivel += 1;
    }
}

fn main() {
    println!("=== 1. Teste de Sucesso ===");
    match carregar_porta_servidor("8080") {
        Ok(porta) => println!("Servidor configurado na porta: {}", porta),
        Err(e) => imprimir_cadeia_de_erros(&e),
    }

    println!("\n=== 2. Teste: Erro de Parsing com Fonte Original (ParseIntError) ===");
    match carregar_porta_servidor("porta_8080_invalida") {
        Ok(porta) => println!("Servidor configurado na porta: {}", porta),
        Err(e) => imprimir_cadeia_de_erros(&e),
    }

    println!("\n=== 3. Teste: Erro de I/O com Fonte Original (io::Error) ===");
    match carregar_porta_servidor("") {
        Ok(porta) => println!("Servidor configurado na porta: {}", porta),
        Err(e) => imprimir_cadeia_de_erros(&e),
    }

    println!("\n=== 4. Teste: Erro Lógico Interno (Sem fonte subjacente) ===");
    match carregar_porta_servidor("80") {
        Ok(porta) => println!("Servidor configurado na porta: {}", porta),
        Err(e) => imprimir_cadeia_de_erros(&e),
    }
}

/*
Resumo sobre Fonte de Erros (Error Chaining):
1. O método `source()` da trait `std::error::Error` permite expor o erro subjacente que causou a falha atual.
2. Permite encapsular erros técnicos/de baixo nível em tipos de erro de alto nível/domínio sem perder a causa raiz.
3. Permite percorrer toda a árvore de falhas (*cause chain*) em loops `while let Some(causa) = erro.source()`.
4. É a prática recomendada em Rust para manter rastreabilidade e depuração eficientes em sistemas complexos.
*/
