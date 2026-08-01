// O Operador de Interrogação (?) e Propagação de Erros em Rust

/*
O operador `?` (Question Mark Operator) é uma das funcionalidades mais idiomáticas e poderosas de Rust.
Ele permite propagar erros (`Result`) ou ausência de valores (`Option`) para a função chamadora sem a necessidade de
escrever blocos `match` repetitivos.

Mecanismo de Funcionamento:
Ao aplicar `expressao?`:
- Se a expressão avaliar para `Ok(valor)` (ou `Some(valor)`), o operador desembala e atribui `valor`.
- Se a expressão avaliar para `Err(erro)` (ou `None`), a função atual interrompe sua execução IMEDIATAMENTE
  e faz `return Err(erro)` (ou `return None`), enviando o erro/ausência para quem chamou a função.
*/

use std::num::ParseIntError;
use std::error::Error;
use std::fmt;

// Definindo um Erro Customizado para o nosso domínio
#[derive(Debug)]
enum ErroProcessamento {
    EntradaInvalida(String),
    ValorForaDoLimite(i32),
    ErroFormato(ParseIntError),
}

// Implementação de Display para o Erro Customizado
impl fmt::Display for ErroProcessamento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErroProcessamento::EntradaInvalida(msg) => write!(f, "Entrada inválida: {}", msg),
            ErroProcessamento::ValorForaDoLimite(val) => write!(f, "Valor {} fora dos limites (deve ser entre 1 e 100)", val),
            ErroProcessamento::ErroFormato(err) => write!(f, "Erro de parse numérico: {}", err),
        }
    }
}

// Implementando Error trait
impl Error for ErroProcessamento {}

// Implementando conversão automática via trait From
impl From<ParseIntError> for ErroProcessamento {
    fn from(err: ParseIntError) -> Self {
        ErroProcessamento::ErroFormato(err)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== 1. Comparação: Sem '?' vs Com '?' ===");
    
    let entrada = "45";
    println!("Processando sem '?': {:?}", processar_sem_interrogacao(entrada));
    println!("Processando com '?': {:?}", processar_com_interrogacao(entrada));

    println!("\n=== 2. Propagação de Erros com Conversão Automática (From trait) ===");
    match ler_e_multiplicar(" 25 ") {
        Ok(res) => println!("Resultado da multiplicação: {}", res),
        Err(e) => println!("Erro retornado: {}", e),
    }

    match ler_e_multiplicar("abc") {
        Ok(res) => println!("Resultado da multiplicação: {}", res),
        Err(e) => println!("Erro retornado: {}", e),
    }

    match ler_e_multiplicar("150") {
        Ok(res) => println!("Resultado da multiplicação: {}", res),
        Err(e) => println!("Erro retornado: {}", e),
    }

    println!("\n=== 3. Utilizando o Operador '?' com Option<T> ===");
    let texto = "Rust é incrível";
    println!("Primeira palavra da string: {:?}", primeira_palavra(texto));
    println!("Primeira palavra de string vazia: {:?}", primeira_palavra(""));

    println!("\n=== 4. Uso do '?' na função main ===");
    let valor_parseado: i32 = "100".parse()?;
    println!("Parse dentro da função main com sucesso: {}", valor_parseado);

    Ok(())
}

// --- Abordagem tradicional SEM o operador '?' (verbosa) ---
fn processar_sem_interrogacao(texto: &str) -> Result<i32, String> {
    let numero = match texto.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return Err(String::from("Falha ao converter o número")),
    };

    if numero < 0 {
        return Err(String::from("Número deve ser positivo"));
    }

    Ok(numero * 2)
}

// --- Abordagem moderna COM o operador '?' (concisa e clara) ---
fn processar_com_interrogacao(texto: &str) -> Result<i32, String> {
    // O 'map_err' converte o erro original em String, e '?' faz o return Err antecipado se falhar
    let numero = texto.parse::<i32>().map_err(|_| "Falha ao converter o número")?;

    if numero < 0 {
        return Err(String::from("Número deve ser positivo"));
    }

    Ok(numero * 2)
}

// --- Propagação com conversão automática de Erros usando Trait From ---
fn ler_e_multiplicar(entrada: &str) -> Result<i32, ErroProcessamento> {
    // Aqui, `.trim().parse::<i32>()?` retorna Result<i32, ParseIntError>.
    // Como implementamos `From<ParseIntError> for ErroProcessamento`, o operador '?'
    // converte automaticamente ParseIntError no nosso enum ErroProcessamento!
    let valor: i32 = entrada.trim().parse()?; 

    if valor < 1 || valor > 100 {
        return Err(ErroProcessamento::ValorForaDoLimite(valor));
    }

    Ok(valor * 3)
}

// --- Operador '?' com Option ---
// Também funciona para Option<T> dentro de funções que retornam Option<T>
fn primeira_palavra(texto: &str) -> Option<&str> {
    let mut palavras = texto.split_whitespace();
    // 'palavras.next()?' retorna a primeira palavra, ou faz return None caso a coleção esteja vazia
    let primeira = palavras.next()?;
    Some(primeira)
}

/*
Resumo sobre o Operador de Interrogação (?):
1. Elimina a necessidade de blocos 'match' repetitivos para repassar erros/ausência.
2. Interrompe a execução da função imediatamente se o valor for Err/None e retorna esse erro.
3. Realiza conversão automática do tipo de erro se houver uma implementação de 'From'.
4. Só pode ser utilizado em funções cujo tipo de retorno seja compatível (Result, Option, etc).
5. A própria função `main` pode retornar `Result<(), E>` para permitir o uso do `?` em seu corpo.
*/
