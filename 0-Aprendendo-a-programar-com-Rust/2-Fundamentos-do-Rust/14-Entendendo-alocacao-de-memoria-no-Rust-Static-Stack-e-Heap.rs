// Entendendo Alocação de Memória no Rust: Static, Stack e Heap

/*
   Para programar bem em Rust, é fundamental entender onde seus dados vivem.
   Existem três áreas principais de memória:
*/

// 1. MEMÓRIA ESTÁTICA (Static Memory)
// Dados que são compilados diretamente no binário e vivem por toda a execução.
static GLOBAL_VARIABLE: i32 = 100;
const CONSTANTE: &str = "Eu sou uma constante";

fn main() {
    println!("--- Alocação de Memória ---");

    // 2. STACK (Pilha)
    // - Armazena dados com tamanho conhecido em tempo de compilação.
    // - É extremamente rápida.
    // - Funciona no sistema LIFO (Last In, First Out).
    // - Exemplos: tipos primitivos (i32, bool, f64, char) e arrays de tamanho fixo.
    
    let x = 42;          // Alocado na Stack
    let y = x;           // Copiado na Stack (inteiros implementam a trait 'Copy')
    let array = [1, 2, 3]; // Alocado na Stack (tamanho fixo)

    println!("Stack: x = {}, y = {}, array = {:?}", x, y, array);

    // 3. HEAP (Monte)
    // - Armazena dados cujo tamanho pode mudar ou é desconhecido na compilação.
    // - É um pouco mais lenta que a Stack (exige busca de espaço livre).
    // - No Rust, a Heap é gerenciada pelo sistema de Ownership (Dono).
    // - Exemplos: String, Vec, Box.

    let mut nome = String::from("Henrique"); // O texto "Henrique" vai para a Heap
    // 'nome' na Stack guarda apenas o ponteiro, tamanho e capacidade para os dados na Heap.
    
    nome.push_str(" Silva"); // Podemos crescer o dado porque ele está na Heap
    
    println!("Heap: nome = {}", nome);

    exemplo_escopo(); // Demonstração de limpeza de memória
}

fn exemplo_escopo() {
    // 's' entra em escopo aqui
    let s = String::from("Olá Heap"); 
    
    println!("Dentro do escopo: {}", s);

} // 's' sai de escopo aqui. 
  // Rust chama automaticamente a função 'drop' e libera a memória na Heap!

/*
   RESUMO:
   
   | Característica | Stack (Pilha)           | Heap (Monte)                |
   |----------------|-------------------------|-----------------------------|
   | Velocidade     | Muito Rápida            | Mais Lenta                  |
   | Tamanho        | Fixo / Conhecido        | Dinâmico / Flexível         |
   | Gerenciamento  | Automático (LIFO)       | Via Ownership (Dono)        |
   | Exemplo        | i32, f64, [T; N]        | String, Vec<T>, Box<T>      |

   ESTÁTICA: Usada para globais e constantes que nunca mudam de endereço 
   durante a vida do programa.
*/
