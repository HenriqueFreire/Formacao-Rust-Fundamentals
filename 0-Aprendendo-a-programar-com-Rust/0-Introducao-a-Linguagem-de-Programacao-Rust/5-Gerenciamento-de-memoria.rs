// 5-Gerenciamento-de-memoria.rs

/*
 * GERENCIAMENTO DE MEMÓRIA EM RUST
 * 
 * O Rust utiliza um sistema inovador baseado em três conceitos principais:
 * 
 * 1. OWNERSHIP (Propriedade):
 *    - Cada valor em Rust tem uma variável que é seu "dono".
 *    - Só pode haver um dono por vez.
 *    - Quando o dono sai de escopo, o valor é automaticamente descartado (drop).
 * 
 * 2. BORROWING (Empréstimo):
 *    - Permite acessar dados sem tirar a propriedade do dono.
 *    - Referências Imutáveis (&T): Pode haver várias simultâneas.
 *    - Referências Mutáveis (&mut T): Só pode haver UMA por vez em um escopo.
 * 
 * 3. LIFETIMES (Tempo de Vida):
 *    - Garante que as referências sejam sempre válidas.
 */

fn main() {
    println!("--- Gerenciamento de Memória em Rust ---");

    // EXEMPLO 1: Ownership e Move
    exemplo_ownership();

    // EXEMPLO 2: Borrowing (Referências)
    exemplo_borrowing();

    // EXEMPLO 3: Referências Mutáveis
    exemplo_mut_borrowing();
}

fn exemplo_ownership() {
    let s1 = String::from("Rust");
    let s2 = s1; // O valor foi MOVIDO para s2. s1 não é mais válido.

    println!("s2 possui o valor: {}", s2);
    // println!("{}", s1); // ERRO DE COMPILAÇÃO: s1 não é mais dono do dado.
}

fn exemplo_borrowing() {
    let s1 = String::from("Linguagem");
    
    // Passamos uma referência (&s1). A função 'calcula_tamanho' pega o valor emprestado.
    let tam = calcula_tamanho(&s1);

    println!("O tamanho de '{}' é {}.", s1, tam); // s1 ainda é válido aqui!
}

fn calcula_tamanho(s: &String) -> usize {
    s.len()
}

fn exemplo_mut_borrowing() {
    let mut s = String::from("Olá");

    // Criamos uma referência mutável para alterar o valor original
    alterar(&mut s);

    println!("String alterada: {}", s);
}

fn alterar(s: &mut String) {
    s.push_str(", mundo!");
}
