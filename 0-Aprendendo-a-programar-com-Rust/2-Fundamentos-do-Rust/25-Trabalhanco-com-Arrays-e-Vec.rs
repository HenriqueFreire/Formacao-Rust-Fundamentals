// Trabalhando com Arrays e Vectors (Vec) em Rust
//
// Rust possui duas formas principais de lidar com coleções de elementos do mesmo tipo:
// Arrays (tamanho fixo) e Vectors (tamanho dinâmico).

fn main() {
    // --- 1. ARRAYS ---
    // - Tamanho fixo e conhecido em tempo de compilação.
    // - Armazenados na STACK.
    // - Sintaxe: [tipo; tamanho]
    let meu_array: [i32; 5] = [10, 20, 30, 40, 50];

    // Inicializando com o mesmo valor: [valor; repetições]
    let array_zeros = [0; 10]; 

    println!("Array: {:?}", meu_array);
    println!("Primeiro elemento: {}", meu_array[0]);
    println!("Tamanho do array: {}", meu_array.len());

    // --- 2. VECTORS (Vec<T>) ---
    // - Tamanho dinâmico (pode crescer ou diminuir).
    // - Armazenados na HEAP.
    // - Muito mais flexíveis que arrays para a maioria dos casos.
    
    // Criando um vetor vazio e adicionando elementos
    let mut meu_vetor = Vec::new();
    meu_vetor.push(1);
    meu_vetor.push(2);
    meu_vetor.push(3);

    // Criando um vetor com valores iniciais usando a macro vec!
    let mut outro_vetor = vec!["Rust", "C++", "Python"];
    
    println!("Vetor inicial: {:?}", outro_vetor);

    // Removendo o último elemento
    outro_vetor.pop(); // Remove "Python"

    // Acessando elementos com segurança usando .get()
    // Retorna um Option, evitando que o programa quebre se o índice não existir
    match outro_vetor.get(1) {
        Some(linguagem) => println!("Linguagem no índice 1: {}", linguagem),
        None => println!("Índice não encontrado!"),
    }

    // --- 3. SLICES (&[T]) ---
    // Uma "visão" ou referência a uma parte de um array ou vetor.
    let slice_do_array = &meu_array[1..4]; // Elementos nos índices 1, 2 e 3
    println!("Slice do array: {:?}", slice_do_array);

    // --- 4. ITERAÇÃO ---
    println!("Iterando sobre o vetor:");
    for ling in &outro_vetor {
        println!(" - {}", ling);
    }
}

/*
DIFERENÇAS PRINCIPAIS:

| Característica | Array [T; N]               | Vector Vec<T>              |
|----------------|----------------------------|----------------------------|
| Tamanho        | Fixo (conhecido no compile) | Dinâmico (muda no runtime) |
| Memória        | Stack                      | Heap                       |
| Flexibilidade  | Baixa                      | Alta                       |
| Uso comum      | Buffers pequenos, coords   | Listas de dados, coleções  |

Dica: 
Use Arrays quando você tiver um número fixo de itens que nunca mudará 
(ex: meses do ano). Para quase tudo o mais, use Vec.
*/
