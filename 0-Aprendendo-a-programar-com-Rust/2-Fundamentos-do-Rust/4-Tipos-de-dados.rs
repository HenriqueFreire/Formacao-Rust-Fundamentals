// Tipos de Dados em Rust

/*
Rust é uma linguagem estaticamente tipada, o que significa que o compilador deve conhecer o tipo de todas as variáveis em tempo de compilação.
Os tipos são divididos em duas categorias principais: Escalares e Compostos.
*/

fn main() {
    // --- 1. TIPOS ESCALARES ---
    // Representam um único valor. Existem quatro tipos principais:

    // A) Inteiros (Integers)
    // Podem ser assinados (i) ou não assinados (u).
    // Tamanhos: 8, 16, 32, 64, 128 bits e arch (isize, usize).
    let inteiro: i32 = -42;
    let natural: u32 = 100;
    let arquitetura: usize = 10; // Depende do sistema (32 ou 64 bits)
    
    // B) Ponto Flutuante (Floating-Point)
    // Rust possui f32 e f64 (padrão é f64 devido à precisão moderna).
    let decimal_simples: f32 = 3.14;
    let decimal_duplo = 2.71828; // f64

    // C) Booleano (Boolean)
    // Possui apenas dois valores: true ou false. Ocupa 1 byte.
    let eh_rust_legal: bool = true;
    let eh_dificil = false;

    // D) Caractere (Character)
    // Representa um valor Unicode Escalar (4 bytes). 
    // Diferente de C/C++, suporta emojis e caracteres especiais nativamente.
    let letra: char = 'R';
    let emoji: char = '🦀'; // Unicode!
    println!("Rust é representado pelo emoji: {}", emoji);


    // --- 2. TIPOS COMPOSTOS ---
    // Podem agrupar múltiplos valores em um único tipo.

    // A) Tuplas (Tuples)
    // Agrupam valores de tipos DIFERENTES. Têm tamanho fixo.
    let tupla: (i32, f64, char) = (500, 6.4, 'Z');
    
    // Acessando valores da tupla (Destruturação):
    let (x, y, z) = tupla;
    println!("O valor de y é: {}", y);

    // Acessando via ponto (.):
    let quinhentos = tupla.0;
    println!("O primeiro elemento é: {}", quinhentos);

    // B) Matrizes / Arrays
    // Agrupam valores do MESMO tipo. Têm tamanho fixo.
    // São úteis quando você quer que os dados sejam alocados na stack (pilha).
    let meses = ["Janeiro", "Fevereiro", "Março"];
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];
    let cinco_zeros = [0; 5]; // Atalho para [0, 0, 0, 0, 0]

    // Acessando elementos do array:
    let primeiro = numeros[0];
    println!("Primeiro número: {}", primeiro);


    // --- 3. INFERÊNCIA DE TIPO ---
    // O compilador do Rust é inteligente. Se você não anotar o tipo, 
    // ele tentará adivinhar com base no valor.
    let suposicao = 42; // O Rust assume i32 por padrão
}

/*
Dica: Se você precisar de uma lista que cresce de tamanho, 
use o tipo 'Vector' (Vec<T>), que veremos mais adiante!
*/
