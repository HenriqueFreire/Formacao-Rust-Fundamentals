// Funções Recursivas em Rust

/*
   Uma função recursiva é uma função que chama a si mesma.
   Para evitar loops infinitos e estouro de pilha (stack overflow),
   toda função recursiva deve ter:
   1. Caso Base: A condição que para a recursão.
   2. Caso Recursivo: A chamada da função para um problema menor.
*/

fn main() {
    println!("--- Estudo de Recursividade ---");

    // Exemplo 1: Fatorial
    // 5! = 5 * 4 * 3 * 2 * 1 = 120
    let num_fatorial = 5;
    let res_fatorial = fatorial(num_fatorial);
    println!("O fatorial de {} é: {}", num_fatorial, res_fatorial);

    // Exemplo 2: Sequência de Fibonacci
    // 0, 1, 1, 2, 3, 5, 8, 13...
    let posicao_fib = 6;
    let res_fib = fibonacci(posicao_fib);
    println!("O número na posição {} da sequência de Fibonacci é: {}", posicao_fib, res_fib);
    
    // Exemplo 3: Contagem Regressiva
    println!("Contagem regressiva:");
    contagem_regressiva(3);
}

/// Calcula o fatorial de um número de forma recursiva.
fn fatorial(n: u32) -> u32 {
    // Caso Base
    if n == 0 {
        return 1;
    }
    // Caso Recursivo
    n * fatorial(n - 1)
}

/// Calcula o n-ésimo termo da sequência de Fibonacci.
fn fibonacci(n: u32) -> u32 {
    // Casos Base
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    
    // Caso Recursivo
    fibonacci(n - 1) + fibonacci(n - 2)
}

/// Uma contagem regressiva simples.
fn contagem_regressiva(n: i32) {
    // Caso Base
    if n < 0 {
        println!("Fim!");
        return;
    }

    println!("{}", n);
    
    // Caso Recursivo
    contagem_regressiva(n - 1);
}

/*
   PONTOS IMPORTANTES:
   - Cuidado com o Stack Overflow: Recursões muito profundas podem esgotar a memória da pilha.
   - Em Rust, não há otimização de chamada de cauda (tail call optimization - TCO) garantida pelo compilador no momento.
   - Para problemas com muitos passos, prefira loops (iteração) para melhor performance e segurança.
*/
