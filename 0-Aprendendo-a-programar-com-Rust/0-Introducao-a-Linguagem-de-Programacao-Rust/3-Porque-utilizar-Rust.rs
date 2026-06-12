// 3-Porque-utilizar-Rust.rs

/*
 * PORQUE UTILIZAR RUST?
 * 
 * 1. PERFORMANCE:
 *    Rust é tão rápido quanto C e C++. Ele não possui um Garbage Collector (coletor de lixo),
 *    o que significa que não há pausas imprevisíveis durante a execução.
 *
 * 2. SEGURANÇA DE MEMÓRIA:
 *    O compilador do Rust (através do sistema de Ownership) impede erros comuns como:
 *    - Dangling pointers (ponteiros para memória já liberada)
 *    - Double free (liberar a mesma memória duas vezes)
 *    - Buffer overflows
 *
 * 3. ABSTRAÇÕES DE CUSTO ZERO:
 *    Você pode usar abstrações de alto nível (como iteradores e closures) sem pagar
 *    penalidade de performance em tempo de execução.
 *
 * 4. FEARLESS CONCURRENCY (Concorrência sem Medo):
 *    O sistema de tipos garante que você não tenha condições de corrida (data races).
 */

fn main() {
    println!("--- Por que utilizar Rust? ---");

    // EXEMPLO 1: Abstrações de Custo Zero
    exemplo_custo_zero();

    // EXEMPLO 2: Segurança com Option (evitando o "Erro de um Bilhão de Dólares")
    exemplo_seguranca_null();

    // EXEMPLO 3: Tratamento de Erros Robusto
    exemplo_result();
}

fn exemplo_custo_zero() {
    let numeros = vec![1, 2, 3, 4, 5];

    // O uso de iteradores funcionais é compilado para um código de máquina 
    // tão eficiente quanto um loop 'for' manual em C.
    let soma: i32 = numeros.iter().filter(|&&x| x % 2 == 0).sum();

    println!("Soma dos pares: {}", soma);
}

fn exemplo_seguranca_null() {
    // Rust não possui 'null'. Em vez disso, usa o enum Option<T>.
    let nome: Option<String> = Some(String::from("Rustacean"));
    let ausencia_de_nome: Option<String> = None;

    match nome {
        Some(n) => println!("Olá, {}!", n),
        None => println!("Nenhum nome encontrado."),
    }

    // O compilador te obriga a tratar o caso 'None', evitando crashes inesperados.
}

fn exemplo_result() {
    // Rust usa Result<T, E> para erros que podem ser recuperados.
    let resultado: Result<i32, &str> = "42".parse().map_err(|_| "Não é um número");

    match resultado {
        Ok(n) => println!("Número parseado com sucesso: {}", n),
        Err(e) => println!("Erro: {}", e),
    }
}
