// Reborrowing em Rust
//
// O conceito de "reborrowing" (re-empréstimo) ocorre quando criamos uma nova referência 
// a partir de uma referência já existente, em vez de criar diretamente a partir do dono (owner).
//
// Isso é especialmente importante com referências mutáveis (&mut T). Diferente de tipos
// que possuem o trait 'Copy', referências mutáveis não podem ser copiadas, elas são movidas.
// No entanto, o Rust permite o reborrowing para facilitar o uso de referências mutáveis
// sem perder o acesso a elas permanentemente.

fn main() {
    let mut valor = 10;

    // Criamos uma referência mutável
    let ref_mut = &mut valor;

    // Exemplo 1: Reborrowing implícito ao passar para uma função
    // Quando passamos 'ref_mut' para a função, o Rust faz um re-empréstimo: &mut *ref_mut
    adicionar_um(ref_mut);

    // ref_mut ainda é válido aqui porque a função apenas "pegou emprestado o empréstimo"
    *ref_mut += 1;
    println!("Valor após reborrowing implícito: {}", ref_mut);

    // Exemplo 2: Reborrowing explícito
    {
        // Criamos uma nova referência mutável a partir de ref_mut
        // Isso é um reborrowing explícito: &mut *ref_mut
        let outro_ref_mut: &mut i32 = &mut *ref_mut;
        
        *outro_ref_mut += 10;
        // println!("{}", ref_mut); // ERRO: Não podemos usar ref_mut enquanto outro_ref_mut (o reborrow) estiver ativo
    }
    // Agora que outro_ref_mut saiu de escopo, ref_mut volta a ser utilizável
    println!("Valor após reborrowing explícito: {}", ref_mut);
}

fn adicionar_um(n: &mut i32) {
    *n += 1;
}

/*
Explicação Técnica:
1. Referências mutáveis têm semântica de "Move". Se você atribuir uma &mut T a outra variável, 
   a original normalmente seria invalidada.
2. O Reborrowing permite que você "empreste" temporariamente uma referência mutável. 
3. Enquanto o re-empréstimo estiver ativo, a referência original fica "congelada" e não pode ser usada.
4. Assim que o re-empréstimo termina (sai de escopo), a referência original volta a ficar ativa.
*/
