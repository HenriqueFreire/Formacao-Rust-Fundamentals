// Exercício: Variáveis e Tipos de Dados em Rust

fn main() {
    /*
        1. Variáveis e Mutabilidade
        Em Rust, variáveis são imutáveis por padrão. 
        Para torná-las mutáveis, usamos a palavra-chave `mut`.
    */

    let x = 5; // Imutável
    // x = 6; // Isso causaria um erro de compilação

    let mut y = 10; // Mutável
    println!("O valor de y é: {}", y);
    y = 15;
    println!("O novo valor de y é: {}", y);

    /*
        2. Tipos de Dados Escalares
        Rust possui quatro tipos escalares primários: inteiros, números de ponto flutuante, booleanos e caracteres.
    */

    // Inteiros (i8, u8, i32, u32, i64, u64, i128, u128, isize, usize)
    let inteiro: i32 = -42;
    let natural: u32 = 100;

    // Ponto Flutuante (f32, f64)
    let pi: f64 = 3.14159;

    // Booleano (bool)
    let e_rust_legal: bool = true;

    // Caractere (char) - Representa um valor escalar Unicode de 4 bytes
    let letra: char = 'R';
    let emoji: char = '🦀';

    /*
        3. Tipos Compostos
        Rust tem dois tipos compostos primitivos: tuplas e matrizes (arrays).
    */

    // Tupla: agrupa valores de diferentes tipos
    let tupla: (i32, f64, char) = (500, 6.4, 'Z');
    let (a, b, c) = tupla; // Destruturação
    println!("O segundo valor da tupla é: {}", tupla.1);

    // Array: agrupa valores do mesmo tipo com tamanho fixo
    let meses = ["Janeiro", "Fevereiro", "Março"];
    let primeiro_mes = meses[0];

    /*
        EXERCÍCIO PRÁTICO:
        Crie variáveis para representar os dados de um produto:
        - Nome (string)
        - Preço (f64)
        - Quantidade em estoque (i32)
        - Disponível (bool)
        
        Depois, imprima esses valores.
    */

    let nome_produto = "Teclado Mecânico";
    let preco_produto = 250.0;
    let mut estoque = 10;
    let disponivel = true;

    println!("--- Dados do Produto ---");
    println!("Nome: {}", nome_produto);
    println!("Preço: R$ {:.2}", preco_produto);
    println!("Estoque: {}", estoque);
    println!("Disponível: {}", disponivel);

    // Simulação de venda
    estoque -= 1;
    println!("Venda realizada! Novo estoque: {}", estoque);
}
