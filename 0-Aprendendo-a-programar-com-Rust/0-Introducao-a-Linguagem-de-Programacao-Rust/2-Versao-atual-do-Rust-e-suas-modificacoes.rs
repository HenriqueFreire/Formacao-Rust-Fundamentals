// 2-Versao-atual-do-Rust-e-suas-modificacoes.rs

/*
 * VERSÕES E EDIÇÕES DO RUST
 * 
 * O Rust segue um processo de lançamento rápido (trem de lançamento), com novas versões 
 * estáveis a cada 6 semanas. Além disso, o Rust usa o conceito de "Edições" para 
 * introduzir mudanças que poderiam quebrar a compatibilidade (como novas palavras-chave).
 *
 * EDIÇÕES PRINCIPAIS:
 * - Rust 2015: Foco em estabilidade (versão 1.0).
 * - Rust 2018: Foco em produtividade (introduziu async/await, melhorias no sistema de módulos).
 * - Rust 2021: Foco em refinamento (melhorias no prelúdio, fechamentos e pânico).
 * - Rust 2024: (Próxima grande edição focada em ergonomia e maturidade).
 */

fn main() {
    println!("--- Versões e Edições do Rust ---");

    // EXEMPLO 1: Iteração sobre arrays (Melhoria do Rust 2021)
    // Antes do Rust 2021, precisávamos usar .iter() ou .into_iter() explicitamente.
    exemplo_iteracao_2021();

    // EXEMPLO 2: Captura de Campos em Closures (Melhoria do Rust 2021)
    // Agora o Rust pode capturar apenas os campos necessários de uma struct.
    exemplo_closures_2021();
}

fn exemplo_iteracao_2021() {
    let numeros = [1, 2, 3];
    
    // No Rust 2021, arrays implementam IntoIterator diretamente em loops for.
    for n in numeros {
        println!("Número: {}", n);
    }
}

struct Usuario {
    nome: String,
    idade: u32,
}

fn exemplo_closures_2021() {
    let user = Usuario {
        nome: String::from("Alice"),
        idade: 30,
    };

    // No Rust 2021, esta closure captura apenas 'user.idade', não a struct 'user' inteira.
    let mostrar_idade = || println!("Idade: {}", user.idade);

    mostrar_idade();
    
    // Isso permite usar 'user.nome' simultaneamente, o que poderia ser restrito em edições anteriores.
    println!("Nome: {}", user.nome);
}
