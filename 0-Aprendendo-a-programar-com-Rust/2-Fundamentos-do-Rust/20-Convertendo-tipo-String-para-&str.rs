// Convertendo String para &str em Rust
//
// Converter uma 'String' (Heap) para um '&str' (Slice) é uma operação muito comum
// e extremamente eficiente, pois não envolve a cópia dos dados do texto,
// apenas a criação de uma nova referência (ponteiro + tamanho).

fn main() {
    let minha_string = String::from("Rust é incrível!");

    // 1. Coerção por Desreferenciação (Deref Coercion) - A MAIS COMUM
    // O Rust converte automaticamente &String para &str quando necessário.
    recebe_slice(&minha_string);

    // 2. Empréstimo Explícito (Explicit Borrowing)
    // O operador '&' cria uma referência. Se o tipo esperado for &str,
    // o Rust aplicará a coerção.
    let slice1: &str = &minha_string;
    println!("Slice 1: {}", slice1);

    // 3. Usando o método .as_str()
    // Uma forma explícita e legível de obter o slice.
    let slice2: &str = minha_string.as_str();
    println!("Slice 2: {}", slice2);

    // 4. Usando Slicing Completo [..]
    // Útil quando você quer ser muito explícito ou está trabalhando com partes.
    let slice3: &str = &minha_string[..];
    println!("Slice 3: {}", slice3);

    // 5. Exemplo de Slicing Parcial
    // Note que isso também resulta em um &str.
    let slice_parcial: &str = &minha_string[0..4]; // "Rust"
    println!("Slice Parcial: {}", slice_parcial);
}

fn recebe_slice(texto: &str) {
    println!("A função recebeu o slice: {}", texto);
}

/*
PONTOS IMPORTANTES:

1. Performance: Todas essas conversões são O(1). Elas apenas criam um "fat pointer" 
   (ponteiro para o dado na Heap + comprimento). Os caracteres reais não são copiados.
2. Tempo de Vida (Lifetime): O &str resultante não pode viver mais que a String original.
   Se a String for deletada (sair de escopo), o &str se torna inválido (o compilador evita isso).
3. Mutabilidade: Você pode converter uma &mut String em um &str, mas perderá a 
   capacidade de modificar o texto através desse slice (já que &str é imutável).
*/
