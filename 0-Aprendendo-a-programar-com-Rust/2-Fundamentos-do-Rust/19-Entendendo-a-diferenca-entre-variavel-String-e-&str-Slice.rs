// Diferença entre String e &str (String Slice) em Rust
//
// Entender a diferença entre esses dois tipos é fundamental para gerenciar 
// memória e performance em Rust.

fn main() {
    // 1. &str (String Slice / Referência de String)
    // - É uma referência a uma sequência de bytes UTF-8.
    // - O tamanho é fixo.
    // - Frequentemente aponta para "string literals" que estão no binário do programa (memória estática).
    // - É imutável por padrão.
    let texto_estatico: &str = "Olá, eu sou um slice!"; 

    // 2. String (Tipo String)
    // - É um tipo "Owned" (com dono).
    // - Armazenada na HEAP.
    // - Pode crescer, diminuir e ser modificada.
    // - Útil quando você não sabe o tamanho do texto em tempo de compilação (ex: input do usuário).
    let mut texto_heap: String = String::from("Olá, eu sou uma String!");
    texto_heap.push_str(" E posso crescer.");

    // --- COMPARAÇÃO PRÁTICA ---

    // A. Converter &str para String
    let de_slice_para_string = "texto".to_string();
    let de_slice_para_string_2 = String::from("outro texto");

    // B. Converter String para &str (Borrowing)
    // Usamos o operador '&' para pegar uma referência (slice) da String
    let string_complexa = String::from("Conteúdo dinâmico");
    let slice_da_string: &str = &string_complexa; 

    // C. Slicing parcial
    // Você pode pegar apenas uma parte da String ou de outro slice
    let parte = &string_complexa[0..8]; // "Conteúdo"

    println!("Original: {}", string_complexa);
    println!("Slice parcial: {}", parte);

    // D. Passando para funções
    // RECOMENDAÇÃO: Use &str em argumentos de função para maior flexibilidade,
    // pois ele aceita tanto literais quanto referências de String.
    imprimir_texto("Sou um literal");
    imprimir_texto(&texto_heap); 
}

fn imprimir_texto(texto: &str) {
    println!("Função recebeu: {}", texto);
}

/*
RESUMO DAS DIFERENÇAS:

| Característica | String                      | &str (Slice)                |
|----------------|-----------------------------|-----------------------------|
| Propriedade    | Possui os dados (Owned)     | Apenas aponta (Borrowed)    |
| Localização    | Memória Heap                | Estática, Heap ou Stack     |
| Tamanho        | Dinâmico (pode crescer)     | Fixo                        |
| Performance    | Um pouco mais lenta (aloca) | Extremamente rápida (refer) |
| Uso comum      | Criar/Modificar dados       | Ler/Analisar dados          |

Dica de Ouro: 
Sempre que uma função apenas precisar LER um texto, use &str. 
Isso permite que a função aceite tanto Strings quanto literais sem alocações extras.
*/
