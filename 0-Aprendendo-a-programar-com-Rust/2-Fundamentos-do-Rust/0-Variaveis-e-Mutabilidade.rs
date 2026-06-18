// Variáveis e Mutabilidade em Rust

/*
Em Rust, a segurança de memória e a concorrência são fundamentais. 
Um dos pilares para isso é como a linguagem lida com variáveis e mutabilidade.
*/

fn main() {
    // 1. Variáveis Imutáveis
    // Por padrão, todas as variáveis em Rust são imutáveis. 
    // Isso significa que uma vez que um valor é atribuído a um nome, você não pode mudá-lo.
    let x = 5;
    println!("O valor de x é: {}", x);
    // x = 6; // Erro de compilação! "cannot assign twice to immutable variable"

    // 2. Variáveis Mutáveis
    // Para tornar uma variável mutável, adicionamos a palavra-chave 'mut' antes do nome da variável.
    let mut y = 10;
    println!("O valor inicial de y é: {}", y);
    y = 15; // Agora é permitido!
    println!("O novo valor de y é: {}", y);

    // 3. Constantes (Constants)
    // Constantes são sempre imutáveis e não podem ser usadas com 'mut'.
    // Elas devem ter o tipo anotado explicitamente e podem ser declaradas em qualquer escopo (inclusive global).
    // Por convenção, usamos SCREAMING_SNAKE_CASE.
    const SEGUNDOS_EM_UM_MINUTO: u32 = 60;
    println!("Segundos em um minuto: {}", SEGUNDOS_EM_UM_MINUTO);

    // 4. Shadowing (Sombreamento)
    // Rust permite declarar uma nova variável com o mesmo nome de uma variável anterior.
    // Dizemos que a primeira variável é "sombreada" pela segunda.
    let z = 5;
    let z = z + 1; // Cria uma nova variável z, escondendo a anterior
    {
        let z = z * 2; // Sombra novamente, mas apenas dentro deste escopo (bloco)
        println!("Valor de z no escopo interno: {}", z); // 12
    }
    println!("Valor de z no escopo externo: {}", z); // 6

    // Vantagem do shadowing: Podemos mudar o TIPO da variável mantendo o mesmo nome.
    let espacos = "   "; // Tipo: &str
    let espacos = espacos.len(); // Tipo: usize (número de espaços)
    println!("Quantidade de espaços: {}", espacos);
    
    // Isso não seria permitido com 'mut':
    // let mut s = "   ";
    // s = s.len(); // Erro! Não se pode mudar o tipo de uma variável mutável.
}

/*
Resumo:
- 'let' define variáveis imutáveis por padrão.
- 'let mut' permite alteração do valor.
- 'const' define valores fixos em tempo de compilação.
- Shadowing permite reutilizar nomes e trocar tipos.
*/
