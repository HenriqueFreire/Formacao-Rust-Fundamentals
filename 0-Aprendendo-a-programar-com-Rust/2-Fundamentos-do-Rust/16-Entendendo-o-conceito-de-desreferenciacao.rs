    // Entendendo o conceito de Desreferenciação (Dereferencing) em Rust

/*
   Desreferenciar significa seguir o ponteiro/referência para chegar ao valor real
   armazenado na memória. Em Rust, usamos o operador asterisco (`*`) para isso.
*/

fn main() {
    println!("--- O Conceito de Desreferenciação ---");

    // 1. O OPERADOR *
    // Quando temos uma referência, não temos o valor em si, mas o "endereço" dele.
    let x = 5;
    let y = &x; // y é uma referência para x (&i32)

    println!("x: {}", x);
    println!("y (referência): {:p}", y); // Mostra o endereço
    println!("*y (desreferenciado): {}", *y); // Segue o endereço para pegar o valor 5

    // 2. ALTERANDO VALORES VIA DESREFERENCIAÇÃO
    // É obrigatório desreferenciar para alterar o valor original através de uma referência mutável.
    let mut numero = 10;
    {
        let r = &mut numero;
        *r += 5; // Sem o '*', estaríamos tentando somar 5 ao endereço, o que não é permitido.
    }
    println!("Numero após alteração via *r: {}", numero);

    // 3. DESREFERENCIAÇÃO IMPLÍCITA (Deref Coercion)
    // Rust é inteligente! Em muitos casos, ele desreferencia automaticamente para você.
    let s = String::from("Olá");
    
    // O método .len() pertence a str, não a &String, mas o Rust faz a mágica.
    println!("Tamanho da string: {}", s.len()); 

    // 4. MÚLTIPLAS REFERÊNCIAS
    let valor = 100;
    let ref1 = &valor;
    let ref2 = &ref1;
    let ref3 = &ref2;

    // Para chegar ao valor original, precisaríamos de múltiplos '*'
    println!("Valor via tripla desreferenciação: {}", ***ref3);
    
    // Mas note que o println! também consegue lidar com referências automaticamente:
    println!("Println lida com ref3 direto: {}", ref3);
}

/*
   PONTOS CHAVE:
   
   1. Operador `*`: Usado para acessar o valor apontado por uma referência.
   2. Mutabilidade: Essencial para alterar dados através de ponteiros (`*r = novo_valor`).
   3. Comparação: Ao comparar referências, o Rust às vezes desreferencia 
      automaticamente, mas usar `*` garante que você está comparando os VALORES 
      e não os ENDEREÇOS.
   4. Trait Deref: Rust permite que tipos customizados se comportem como referências 
      implementando a trait `Deref`, permitindo o uso do operador `*`.
*/
