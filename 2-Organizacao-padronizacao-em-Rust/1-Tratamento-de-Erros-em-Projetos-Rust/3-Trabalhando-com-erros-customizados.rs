// Trabalhando com Erros Customizados em Rust

/*
Em aplicações reais em Rust, é fundamental definir tipos de erro próprios (customizados) 
para que possamos representar com precisão as falhas do nosso domínio de negócio.

Para que um tipo seja considerado um erro idiomático em Rust, ele deve:
1. Implementar `std::fmt::Debug` (para logs de depuração).
2. Implementar `std::fmt::Display` (para formatação amigável e legível por humanos).
3. Implementar `std::error::Error` (a trait padrão de erro em Rust).
4. (Opcional, porém altamente recomendado) Implementar a trait `From<E>` para converter erros externos automaticamente.
*/

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

// ============================================================================
// 1. Criando um Enum de Erro Customizado para o Domínio Bancário
// ============================================================================

#[derive(Debug)]
enum ErroBancario {
    SaldoInsuficiente { saldo_atual: f64, valor_solicitado: f64 },
    ContaBloqueada(String),
    ValorInvalido(f64),
    ErroDeParsing(ParseIntError),
}

// 2. Implementação de Display (Mensagem amigável para o usuário final)
impl fmt::Display for ErroBancario {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErroBancario::SaldoInsuficiente { saldo_atual, valor_solicitado } => {
                write!(f, "Saldo insuficiente. Saldo atual: R$ {:.2}, tentativa de saque: R$ {:.2}", saldo_atual, valor_solicitado)
            }
            ErroBancario::ContaBloqueada(motivo) => {
                write!(f, "Operação negada: A conta está bloqueada pelo motivo: '{}'", motivo)
            }
            ErroBancario::ValorInvalido(val) => {
                write!(f, "O valor R$ {:.2} é inválido para esta operação", val)
            }
            ErroBancario::ErroDeParsing(err) => {
                write!(f, "Falha na conversão de dados da conta: {}", err)
            }
        }
    }
}

// 3. Implementação da Trait std::error::Error
impl Error for ErroBancario {
    // Método opcional para retornar a causa subjacente do erro (encadeamento de erros)
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErroBancario::ErroDeParsing(err) => Some(err),
            _ => None, // Outros erros não possuem uma causa raiz externa
        }
    }
}

// 4. Implementação da Trait From para conversão automática via operador '?'
impl From<ParseIntError> for ErroBancario {
    fn from(err: ParseIntError) -> Self {
        ErroBancario::ErroDeParsing(err)
    }
}

// ============================================================================
// 2. Struct de Erro Customizado (Útil para erros simples ou específicos)
// ============================================================================

#[derive(Debug)]
struct ErroValidacaoSenha {
    tamanho_minimo: usize,
    tamanho_recebido: usize,
}

impl fmt::Display for ErroValidacaoSenha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Senha muito curta: tamanho mínimo é {} caracteres, mas foram fornecidos {}", 
            self.tamanho_minimo, self.tamanho_recebido
        )
    }
}

impl Error for ErroValidacaoSenha {}

// ============================================================================
// 3. Regras de Negócio e Funções de Exemplo
// ============================================================================

struct ContaBancaria {
    titular: String,
    saldo: f64,
    ativa: bool,
}

impl ContaBancaria {
    fn sacar(&mut self, valor: f64) -> Result<f64, ErroBancario> {
        if !self.ativa {
            return Err(ErroBancario::ContaBloqueada(String::from("Suspeita de fraude")));
        }

        if valor <= 0.0 {
            return Err(ErroBancario::ValorInvalido(valor));
        }

        if valor > self.saldo {
            return Err(ErroBancario::SaldoInsuficiente {
                saldo_atual: self.saldo,
                valor_solicitado: valor,
            });
        }

        self.saldo -= valor;
        Ok(self.saldo)
    }
}

// Função que combina parse + conversão com operador '?'
fn processar_codigo_transacao(codigo_str: &str) -> Result<i32, ErroBancario> {
    // O ? converterá ParseIntError em ErroBancario::ErroDeParsing automaticamente devido ao From
    let codigo: i32 = codigo_str.trim().parse()?;
    Ok(codigo * 10)
}

fn validar_senha(senha: &str) -> Result<(), ErroValidacaoSenha> {
    if senha.len() < 8 {
        Err(ErroValidacaoSenha {
            tamanho_minimo: 8,
            tamanho_recebido: senha.len(),
        })
    } else {
        Ok(())
    }
}

fn main() {
    println!("=== 1. Testando Enum de Erro Customizado (Regras de Negócio) ===");
    let mut conta = ContaBancaria {
        titular: String::from("Henrique"),
        saldo: 500.0,
        ativa: true,
    };

    println!("Titular: {}", conta.titular);

    // Tentativa 1: Saque com sucesso
    match conta.sacar(150.0) {
        Ok(novo_saldo) => println!("Saque realizado! Novo saldo: R$ {:.2}", novo_saldo),
        Err(e) => println!("Erro no saque: {}", e),
    }

    // Tentativa 2: Saque com valor inválido
    match conta.sacar(-50.0) {
        Ok(novo_saldo) => println!("Saque realizado! Novo saldo: R$ {:.2}", novo_saldo),
        Err(e) => println!("Erro no saque: {}", e),
    }

    // Tentativa 3: Saque com saldo insuficiente
    match conta.sacar(1000.0) {
        Ok(novo_saldo) => println!("Saque realizado! Novo saldo: R$ {:.2}", novo_saldo),
        Err(e) => println!("Erro no saque: {}", e),
    }

    println!("\n=== 2. Conversão Automática de Erro de Biblioteca para Erro Customizado ===");
    match processar_codigo_transacao(" 1234 ") {
        Ok(cod) => println!("Código de transação gerado: {}", cod),
        Err(e) => println!("Erro no código: {}", e),
    }

    match processar_codigo_transacao("CODIGO_INVALIDO") {
        Ok(cod) => println!("Código de transação gerado: {}", cod),
        Err(e) => {
            println!("Erro no código: {}", e);
            if let Some(causa) = e.source() {
                println!("Causa raiz identificada: {}", causa);
            }
        }
    }

    println!("\n=== 3. Struct de Erro Customizado ===");
    match validar_senha("12345") {
        Ok(_) => println!("Senha válida!"),
        Err(e) => println!("Erro de validação: {}", e),
    }
}

/*
Resumo sobre Erros Customizados:
1. Permite criar tipos fortemente tipados para representar falhas da sua aplicação.
2. Exige a implementação de `Debug`, `Display` e `std::error::Error`.
3. Implementar `From<OutroErro>` facilita o uso do operador `?` para converter erros de terceiros.
4. Em projetos maiores da comunidade Rust, crates como `thiserror` (para bibliotecas) 
   e `anyhow` (para aplicações/CLI) são amplamente utilizadas para reduzir o código boilerplate.
*/
