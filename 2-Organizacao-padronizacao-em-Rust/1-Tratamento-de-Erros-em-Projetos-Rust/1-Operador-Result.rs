// O Tipo Result<T, E> e o Tratamento de Erros Recuperáveis em Rust

/*
Em Rust, erros são divididos em duas categorias principais:
1. Erros Irrecuperáveis (Unrecoverable): Situações onde o programa não pode continuar (ex: estouro de array). 
   São tratados com a macro `panic!`.
2. Erros Recuperáveis (Recoverable): Falhas esperadas das quais o programa pode se recuperar (ex: arquivo não encontrado, falha de parse).
   São representados pela enumeração `Result<T, E>`:

enum Result<T, E> {
    Ok(T),  // Operação bem-sucedida, contendo o resultado do tipo T
    Err(E), // Operação falhou, contendo o erro do tipo E
}

Assim como `Option`, `Ok` e `Err` estão no escopo global (prelude).
*/

use std::num::ParseIntError;

fn main() {
    println!("=== 1. Criando e Retornando Result ===");
    let sucesso: Result<i32, String> = Ok(200);
    let falha: Result<i32, String> = Err(String::from("Falha de conexão com o servidor"));

    println!("Sucesso: {:?}", sucesso);
    println!("Falha: {:?}", falha);

    println!("\n=== 2. Manipulando Result com 'match' ===");
    match converter_para_numero("42") {
        Ok(n) => println!("Número convertido com sucesso: {}", n),
        Err(e) => println!("Erro na conversão: {}", e),
    }

    match converter_para_numero("abc") {
        Ok(n) => println!("Número convertido com sucesso: {}", n),
        Err(e) => println!("Erro na conversão: {}", e),
    }

    println!("\n=== 3. Manipulando Result com 'if let' ===");
    // Útil quando desejamos tratar apenas o sucesso ou a falha pontualmente
    if let Ok(valor) = converter_para_numero("100") {
        println!("Valor válido extraído com if let: {}", valor);
    }

    if let Err(erro) = converter_para_numero("invalido") {
        println!("Tratando o erro especificamente com if let: {}", erro);
    }

    println!("\n=== 4. Métodos Utilitários do Result ===");
    let res_ok: Result<i32, &str> = Ok(10);
    let res_err: Result<i32, &str> = Err("Erro inesperado");

    // is_ok() e is_err()
    println!("res_ok.is_ok(): {}", res_ok.is_ok());
    println!("res_err.is_err(): {}", res_err.is_err());

    // unwrap_or(): Retorna o valor de Ok ou o valor padrão fornecido
    println!("Valor com unwrap_or (Ok): {}", res_ok.unwrap_or(0));
    println!("Valor com unwrap_or (Err): {}", res_err.unwrap_or(0));

    // unwrap_or_else(): Calcula o valor padrão usando uma função/closure
    let valor_fallback = res_err.unwrap_or_else(|e| {
        println!("Log do erro: '{}'", e);
        -1
    });
    println!("Valor obtido via unwrap_or_else: {}", valor_fallback);

    // expect(): Semelhante ao unwrap(), mas inclui mensagem explicativa em caso de panic
    let valor_explicito = res_ok.expect("Deveria conter um número válido");
    println!("Valor obtido com expect(): {}", valor_explicito);
    // let causa_panic = res_err.expect("Falha crítica ao obter valor"); // Causaria panic!

    println!("\n=== 5. Combinadores (map, map_err e and_then) ===");
    let texto_num = "50";

    // map(): Transforma o valor dentro de Ok, mantendo o Err inalterado
    let dobrado: Result<i32, ParseIntError> = converter_para_numero(texto_num).map(|n| n * 2);
    println!("Resultado dobrado com map: {:?}", dobrado);

    // map_err(): Transforma o erro contido em Err, mantendo Ok inalterado
    let erro_personalizado: Result<i32, String> = converter_para_numero("xyz")
        .map_err(|e| format!("Entrada inválida! Detalhes do sistema: {}", e));
    println!("Erro formatado com map_err: {:?}", erro_personalizado);

    // and_then(): Encadeia operações que também retornam Result
    let resultado_encadeado = converter_para_numero("10")
        .and_then(|n| dividir_inteiros(n, 2));
    println!("Resultado encadeado: {:?}", resultado_encadeado);

    println!("\n=== 6. Propagação de Erros com o Operador '?' ===");
    match processar_transacao("500", "50") {
        Ok(saldo_restante) => println!("Transação concluída! Saldo restante: R$ {}", saldo_restante),
        Err(e) => println!("Falha no processamento da transação: {}", e),
    }

    match processar_transacao("cem", "50") {
        Ok(saldo_restante) => println!("Transação concluída! Saldo restante: R$ {}", saldo_restante),
        Err(e) => println!("Falha no processamento da transação: {}", e),
    }
}

// Função que converte String para número e retorna Result<i32, ParseIntError>
fn converter_para_numero(val: &str) -> Result<i32, ParseIntError> {
    val.parse::<i32>()
}

// Função para divisão de inteiros com retorno de Result
fn dividir_inteiros(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divisão por zero não é permitida"))
    } else {
        Ok(a / b)
    }
}

// Demonstração do operador '?' para propagar erros
// O operador '?' tenta desembalar 'Ok'. Se for 'Err', ele faz 'return Err(...)' imediatamente.
fn processar_transacao(saldo_str: &str, saque_str: &str) -> Result<i32, String> {
    let saldo = saldo_str
        .parse::<i32>()
        .map_err(|_| String::from("Valor do saldo inválido"))?; // Propaga o erro se falhar

    let saque = saque_str
        .parse::<i32>()
        .map_err(|_| String::from("Valor do saque inválido"))?; // Propaga o erro se falhar

    if saque > saldo {
        return Err(String::from("Saldo insuficiente para a operação"));
    }

    Ok(saldo - saque)
}

/*
Resumo sobre Result<T, E>:
1. Representa operações que podem ser bem-sucedidas ('Ok(T)') ou falhar com erro ('Err(E)').
2. Força o desenvolvedor a tratar a possibilidade de falha explicitamente.
3. Métodos como 'unwrap_or', 'map', 'map_err' e 'and_then' ajudam na manipulação dos resultados.
4. O operador '?' é a forma idiomatica em Rust para propagar erros em funções que retornam Result.
*/
