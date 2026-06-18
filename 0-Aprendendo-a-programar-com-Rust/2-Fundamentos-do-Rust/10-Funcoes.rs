// Exercício: Funções em Rust

/*
    As funções são fundamentais em Rust. A função principal de todo programa
    é a `main`. Para criar novas funções, usamos a palavra-chave `fn`.
*/

fn main() {
    println!("--- Aprendendo sobre Funções ---");

    // 1. Chamando uma função simples
    dizer_ola();

    // 2. Chamando uma função com parâmetros
    exibir_idade(25);

    // 3. Chamando uma função com múltiplos parâmetros
    exibir_dados("Henrique", 30);

    // 4. Funções que retornam valores
    let resultado_soma = somar(10, 20);
    println!("O resultado da soma é: {}", resultado_soma);

    // 5. Exemplo de Expressão vs Statement
    // Em Rust, quase tudo é uma expressão (retorna valor).
    let x = {
        let y = 5;
        y + 1 // Note que não tem ponto e vírgula! Isso torna o bloco uma expressão.
    };
    println!("O valor de x (vindo de um bloco de expressão) é: {}", x);
}

// --- DEFINIÇÕES DE FUNÇÕES ---

// Função simples sem parâmetros e sem retorno
fn dizer_ola() {
    println!("Olá, mundo!");
}

// Função com um parâmetro (o tipo deve ser especificado)
fn exibir_idade(idade: i32) {
    println!("A idade informada é: {} anos", idade);
}

// Função com múltiplos parâmetros
fn exibir_dados(nome: &str, idade: i32) {
    println!("Nome: {}, Idade: {}", nome, idade);
}

/*
    Função com Retorno:
    - Usamos `->` para especificar o tipo de retorno.
    - O Rust retorna automaticamente a ÚLTIMA expressão do bloco (sem ponto e vírgula).
    - Também é possível usar a palavra-chave `return` para retornos antecipados.
*/
fn somar(a: i32, b: i32) -> i32 {
    a + b // Retorno implícito (idiomático em Rust)
}

/*
    EXERCÍCIO PRÁTICO:
    Crie uma função chamada `calcular_area_retangulo` que recebe
    a base e a altura (f64) e retorna a área.
*/

fn calcular_area_retangulo(base: f64, altura: f64) -> f64 {
    base * altura
}

// No main, você poderia testar assim:
// let area = calcular_area_retangulo(5.5, 10.0);
// println!("Área: {}", area);
