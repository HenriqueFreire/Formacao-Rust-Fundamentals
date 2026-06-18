// Comentários em Rust

/*
Comentários são fundamentais para explicar o "porquê" do código, 
documentar funcionalidades e facilitar a manutenção.
Em Rust, existem três tipos principais de comentários.
*/

/// Comentários de Documentação (Doc Comments)
/// Estes comentários suportam Markdown e são usados pelo `rustdoc` 
/// para gerar a documentação HTML do seu projeto.
/// 
/// # Exemplo de uso do `rustdoc`:
/// No terminal, você pode rodar `cargo doc --open` para ver este comentário formatado.
fn main() {
    // 1. Comentários de Linha Única
    // Este é o tipo mais comum, iniciado por duas barras invertidas.
    let x = 5; // Você também pode comentar ao final de uma linha de código.

    /*
       2. Comentários de Múltiplas Linhas (Bloco)
       São úteis para explicações longas ou para desativar 
       grandes blocos de código temporariamente durante testes.
       Este tipo de comentário pode ser aninhado.
    */
    let y = 10;

    // 3. Comentários de Documentação Internos
    // Enquanto o `///` documenta o item que vem DEPOIS dele, 
    // o `//!` documenta o item que o CONTÉM (geralmente usado no topo de arquivos ou módulos).
    
    println!("Valor de x: {}, Valor de y: {}", x, y);
}

/// Esta função soma dois números.
/// 
/// # Arguments
/// * `a` - O primeiro número (i32)
/// * `b` - O segundo número (i32)
/// 
/// # Returns
/// A soma de `a` e `b`.
/// 
/// # Example
/// ```
/// let resultado = somar(2, 2);
/// assert_eq!(resultado, 4);
/// ```
fn somar(a: i32, b: i32) -> i32 {
    a + b // Comentário simples aqui dentro
}

/*
Dicas de Boas Práticas:
- Use // para lógica interna.
- Use /// para documentar funções, structs e enums públicos.
- Evite comentários óbvios (ex: i = i + 1; // Incrementa i).
- Comente a INTENÇÃO, não apenas o QUE o código faz.
*/
