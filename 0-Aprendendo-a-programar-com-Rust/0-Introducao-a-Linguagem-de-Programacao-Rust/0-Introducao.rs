// 0-Introducao.rs
// Bem-vindo à Introdução à Linguagem Rust!

// 1. Hello World
// O ponto de entrada de qualquer programa Rust é a função `main`.
fn main() {
    // `println!` é uma macro que imprime texto no console.
    println!("Olá, mundo!");

    // Chamando outras funções de exemplo
    variaveis_e_mutabilidade();
    tipos_de_dados();
    funcoes(5, 'z');
    controle_de_fluxo();
}

// 2. Variáveis e Mutabilidade
// Por padrão, as variáveis em Rust são imutáveis.
fn variaveis_e_mutabilidade() {
    println!("\n--- Variáveis e Mutabilidade ---");

    let x = 5;
    println!("O valor de x é: {}", x);
    // x = 6; // Isso causaria um erro de compilação!

    // Para tornar uma variável mutável, use a palavra-chave `mut`.
    let mut y = 10;
    println!("O valor original de y é: {}", y);
    y = 15;
    println!("O novo valor de y é: {}", y);

    // Constantes são sempre imutáveis e devem ter o tipo declarado.
    const TRES_HORAS_EM_SEGUNDOS: u32 = 60 * 60 * 3;
    println!("Constante: {}", TRES_HORAS_EM_SEGUNDOS);
}

// 3. Tipos de Dados
fn tipos_de_dados() {
    println!("\n--- Tipos de Dados ---");

    // Tipos Escalares: inteiros, ponto flutuante, booleanos, caracteres.
    let inteiro: i32 = -42;
    let flutuante: f64 = 3.14;
    let booleano: bool = true;
    let caractere: char = 'R';

    println!("Escalares: {}, {}, {}, {}", inteiro, flutuante, booleano, caractere);

    // Tipos Compostos: Tuplas e Arrays.
    let tupla: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tupla; // Desestruturação
    println!("Tupla: a={}, b={}, c={}", a, b, c);

    let array = [1, 2, 3, 4, 5];
    println!("Primeiro elemento do array: {}", array[0]);
}

// 4. Funções
// Funções podem receber parâmetros e retornar valores.
fn funcoes(valor: i32, unidade: char) {
    println!("\n--- Funções ---");
    println!("A medida é: {}{}", valor, unidade);

    let resultado = soma(10, 20);
    println!("Soma de 10 + 20 = {}", resultado);
}

fn soma(a: i32, b: i32) -> i32 {
    // Em Rust, a última expressão em uma função é retornada automaticamente (sem ponto e vírgula).
    a + b
}

// 5. Controle de Fluxo
fn controle_de_fluxo() {
    println!("\n--- Controle de Fluxo ---");

    let numero = 7;

    // if/else
    if numero < 5 {
        println!("Condição verdadeira");
    } else {
        println!("Condição falsa");
    }

    // Loops: loop, while, for
    println!("Contagem regressiva:");
    let mut contagem = 3;
    while contagem != 0 {
        println!("{}!", contagem);
        contagem -= 1;
    }

    println!("Iterando sobre um array:");
    let a = [10, 20, 30, 40, 50];
    for elemento in a {
        println!("O valor é: {}", elemento);
    }
}
