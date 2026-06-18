// Shadowing (Sombreamento) em Rust

/*
Shadowing ocorre quando declaramos uma nova variável com o mesmo nome de uma variável anterior.
A segunda variável "esconde" ou "sombreia" a primeira até que ela mesma saia de escopo ou seja sombreada novamente.
*/

fn main() {
    // 1. Shadowing Básico
    let x = 5;
    let x = x + 1; // A primeira variável x é sombreada
    
    {
        // 2. Shadowing em Escopos Internos
        // Aqui, x será sombreado apenas dentro deste bloco {}
        let x = x * 2;
        println!("O valor de x no escopo interno é: {}", x); // Saída: 12
    }

    println!("O valor de x no escopo externo ainda é: {}", x); // Saída: 6

    // 3. Diferença entre Shadowing e Mutabilidade (mut)
    
    // Com 'mut', você altera o valor, mas NÃO pode alterar o tipo:
    let mut y = 10;
    y = 20; // Ok
    // y = "texto"; // ERRO! Não é possível mudar o tipo de i32 para &str
    println!("y mutado: {}", y);

    // Com Shadowing, você pode alterar o TIPO mantendo o mesmo nome:
    let espacos = "   ";           // Tipo: &str (string slice)
    let espacos = espacos.len();   // Tipo: usize (inteiro)
    println!("Quantidade de espaços (via shadowing): {}", espacos); // Saída: 3

    /*
    Por que usar Shadowing em vez de 'mut'?
    1. Para transformar dados sem precisar criar nomes feios como 'espacos_str' e 'espacos_int'.
    2. Para manter a imutabilidade após a transformação (as variáveis resultantes ainda são imutáveis por padrão).
    3. Para reutilizar nomes de variáveis em cálculos sequenciais.
    */
    
    // 4. Exemplo de cálculo sequencial
    let valor = "42";
    let valor: i32 = valor.parse().expect("Não é um número!");
    let valor = valor + 10;
    println!("Valor final processado: {}", valor); // Saída: 52
}

/*
Resumo:
- Shadowing usa a palavra-chave 'let' repetidamente.
- Permite mudar o tipo da variável.
- Respeita o escopo (blocos {}).
- Diferente de 'mut', que apenas permite alterar o VALOR do mesmo tipo.
*/
