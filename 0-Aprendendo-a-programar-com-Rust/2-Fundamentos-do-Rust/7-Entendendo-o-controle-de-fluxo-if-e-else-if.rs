// Exercício: Controle de Fluxo com if, else if e else em Rust

fn main() {
    /*
        1. Estrutura Básica if
        O bloco `if` executa um código se a condição for verdadeira.
        Diferente de algumas linguagens, a condição NÃO precisa de parênteses,
        mas o corpo do bloco DEVE estar entre chaves `{}`.
    */

    let numero = 10;

    if numero > 5 {
        println!("O número {} é maior que 5.", numero);
    }

    /*
        2. Uso do else
        O `else` fornece um caminho alternativo caso a condição do `if` seja falsa.
    */

    let idade = 16;

    if idade >= 18 {
        println!("Você é maior de idade.");
    } else {
        println!("Você é menor de idade.");
    }

    /*
        3. Múltiplas Condições com else if
        Podemos verificar várias condições em sequência.
    */

    let nota = 85;

    if nota >= 90 {
        println!("Desempenho: Excelente (A)");
    } else if nota >= 70 {
        println!("Desempenho: Bom (B)");
    } else if nota >= 50 {
        println!("Desempenho: Regular (C)");
    } else {
        println!("Desempenho: Insuficiente (F)");
    }

    /*
        4. if em uma Declaração let
        Como o `if` é uma expressão em Rust, podemos usá-lo no lado direito de um `let`.
        Importante: todos os blocos do `if` devem retornar o mesmo tipo.
    */

    let condicao = true;
    let numero_escolhido = if condicao { 5 } else { 6 };

    println!("O valor do número escolhido é: {}", numero_escolhido);

    /*
        EXERCÍCIO PRÁTICO:
        Verifique se um número é positivo, negativo ou zero.
    */

    let valor = -10;

    if valor > 0 {
        println!("O valor {} é positivo.", valor);
    } else if valor < 0 {
        println!("O valor {} é negativo.", valor);
    } else {
        println!("O valor é zero.");
    }
}
