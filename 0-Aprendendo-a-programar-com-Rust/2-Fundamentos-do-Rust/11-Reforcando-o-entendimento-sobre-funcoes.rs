// Reforçando o entendimento sobre funções em Rust

/*
   Em Rust, as funções são declaradas usando a palavra-chave `fn`.
   O padrão para nomes de funções e variáveis é o 'snake_case'.
*/

fn main() {
    println!("Reforçando o entendimento sobre funções!");

    // 1. Chamando uma função simples
    saudacao();

    // 2. Passando parâmetros
    // Rust exige que você declare o tipo de cada parâmetro.
    exibir_idade(25);

    // 3. Funções com múltiplos parâmetros
    exibir_dados("Henrique", 30);

    // 4. Funções que retornam valores
    // O tipo de retorno é declarado após `->`.
    let soma = somar(10, 5);
    println!("A soma de 10 e 5 é: {}", soma);

    // 5. Expressões vs Statements
    // Rust é uma linguagem baseada em expressões. 
    // O valor de retorno de uma função pode ser a última expressão do bloco (sem ponto e vírgula).
    let resultado_dobro = dobrar(8);
    println!("O dobro de 8 é: {}", resultado_dobro);
}

// Função sem parâmetros e sem retorno
fn saudacao() {
    println!("Olá! Bem-vindo ao estudo de funções em Rust.");
}

// Função com um parâmetro
fn exibir_idade(idade: i32) {
    println!("A idade informada é: {}", idade);
}

// Função com múltiplos parâmetros de tipos diferentes
fn exibir_dados(nome: &str, idade: i32) {
    println!("Nome: {}, Idade: {}", nome, idade);
}

// Função que retorna um valor usando a palavra-chave `return`
fn somar(a: i32, b: i32) -> i32 {
    return a + b;
}

// Função que retorna um valor de forma implícita (idiomático em Rust)
// Note que não há ponto e vírgula na última linha. Isso a torna uma expressão.
fn dobrar(numero: i32) -> i32 {
    numero * 2
}

/*
   PONTOS CHAVE:
   - Declaradas com `fn`.
   - Parâmetros DEVEM ter tipos anotados.
   - O tipo de retorno é especificado com `->`.
   - A última expressão de uma função é retornada automaticamente se não houver ponto e vírgula.
   - Você pode usar `return` para retornos antecipados.
*/
