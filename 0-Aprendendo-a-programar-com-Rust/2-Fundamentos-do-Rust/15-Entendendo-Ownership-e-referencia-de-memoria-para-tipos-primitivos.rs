// Entendendo Ownership e Referência de Memória para Tipos Primitivos

/*
   No Rust, o conceito de Ownership (Posse) rege como a memória é gerenciada.
   No entanto, tipos primitivos (como i32, bool, f64) se comportam de forma
   especial porque implementam a trait 'Copy'.
*/

fn main() {
    println!("--- Ownership e Tipos Primitivos ---");

    // 1. COMPORTAMENTO DE CÓPIA (Copy Trait)
    // Para tipos que vivem inteiramente na Stack, atribuir uma variável a outra
    // cria uma cópia bit a bit. O "dono" original ainda é válido.
    
    let x = 5;
    let y = x; // 'x' é copiado para 'y'. Ambos são válidos.

    println!("x: {}, y: {}", x, y); // Funciona perfeitamente.

    // 2. REFERÊNCIAS (Borrowing)
    // Podemos criar referências para tipos primitivos usando o símbolo '&'.
    // Isso permite acessar o valor sem tomar a posse ou copiar o dado.
    
    let a = 10;
    let b = &a; // 'b' é uma referência para 'a'

    println!("Valor de a: {}", a);
    println!("Valor através da referência b: {}", b);
    println!("Endereço de memória de a: {:p}", &a);
    println!("Endereço que b aponta: {:p}", b);

    // 3. REFERÊNCIAS MUTÁVEIS
    // Para alterar um valor através de uma referência, ambos devem ser mutáveis.
    
    let mut numero = 50;
    {
        let referencia_mutavel = &mut numero;
        *referencia_mutavel += 10; // Usamos '*' (desreferenciação) para alterar o valor
    } // 'referencia_mutavel' sai de escopo aqui

    println!("Valor de numero após alteração: {}", numero);

    // 4. PASSANDO PARA FUNÇÕES
    let valor = 100;
    
    // Como é um tipo primitivo, 'valor' é copiado para a função.
    // A variável 'valor' continua sendo válida após a chamada.
    faz_algo_com_copia(valor);
    println!("'valor' ainda existe aqui: {}", valor);

    // Passando por referência
    faz_algo_com_referencia(&valor);
}

fn faz_algo_com_copia(n: i32) {
    println!("Cópia dentro da função: {}", n);
}

fn faz_algo_com_referencia(n: &i32) {
    println!("Lendo via referência na função: {}", n);
}

/*
   RESUMO PARA TIPOS PRIMITIVOS:
   
   1. Copy Trait: Tipos como inteiros, booleanos e floats não sofrem "Move".
      Eles são copiados automaticamente na Stack.
   2. Borrowing: Você ainda pode usar referências (&) para evitar cópias 
      (embora para tipos pequenos a cópia seja mais rápida) ou para 
      atender requisitos de funções.
   3. Desreferenciação (*): Usado para acessar ou modificar o valor real 
      escondido atrás de uma referência mutável.
*/
