// 1-O-que-eh-o-Rust-e-quando-foi-lancado.rs

/*
 * O QUE É RUST?
 * Rust é uma linguagem de programação multi-paradigma, focada em performance e segurança,
 * especialmente segurança de memória e concorrência segura.
 *
 * QUANDO FOI LANÇADO?
 * - Criada por Graydon Hoare na Mozilla Research em 2006 (projeto pessoal).
 * - Anunciada oficialmente em 2010.
 * - Versão estável 1.0 lançada em 15 de maio de 2015.
 */

fn main() {
    println!("--- O que é Rust? ---");
    
    // EXEMPLO 1: Segurança de Memória (Ownership)
    // Em Rust, o compilador garante que você não acesse memória inválida.
    exemplo_ownership();

    // EXEMPLO 2: Concorrência Segura
    // Rust evita "data races" em tempo de compilação.
    exemplo_concorrencia();
}

fn exemplo_ownership() {
    let s1 = String::from("Olá");
    let s2 = s1; // O valor foi "movido" para s2. s1 não é mais válido.
    
    // println!("{}", s1); // Isso causaria um erro de compilação!
    println!("Valor movido para s2: {}", s2);
}

fn exemplo_concorrencia() {
    use std::thread;

    let v = vec![1, 2, 3];

    // 'move' transfere a propriedade de v para a thread.
    let handle = thread::spawn(move || {
        println!("Vetor na thread: {:?}", v);
    });

    handle.join().unwrap();
}
