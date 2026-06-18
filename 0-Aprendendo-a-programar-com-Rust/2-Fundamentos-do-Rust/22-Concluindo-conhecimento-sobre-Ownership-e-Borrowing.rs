// Concluindo Conhecimento sobre Ownership e Borrowing em Rust
//
// Este arquivo resume os pilares da gestão de memória do Rust sem Garbage Collector.

fn main() {
    /* 
       1. OWNERSHIP (PROPRIEDADE)
       - Cada valor tem um único "dono" (owner).
       - Quando o dono sai de escopo, o valor é limpo (Drop).
       - Ao atribuir a outra variável, o valor é MOVIDO (se não for Copy).
    */
    let s1 = String::from("Ownership");
    let s2 = s1; // s1 MOVIDO para s2. s1 agora é inválida.
    // println!("{}", s1); // ERRO!
    println!("Dono atual: {}", s2);


    /* 
       2. BORROWING (EMPRÉSTIMO IMUTÁVEL)
       - Você pode ter infinitas referências imutáveis (&T).
       - Os dados não podem ser alterados através delas.
       - A variável original deve permanecer válida.
    */
    let ref1 = &s2;
    let ref2 = &s2;
    println!("Referências imutáveis: {} e {}", ref1, ref2); // OK!


    /* 
       3. MUTABLE BORROWING (EMPRÉSTIMO MUTÁVEL)
       - Você só pode ter UMA referência mutável (&mut T) por vez em um escopo.
       - Se houver uma &mut T, você NÃO pode ter nenhuma outra referência (& ou &mut).
    */
    let mut dado = String::from("Original");
    
    {
        let ref_mut = &mut dado;
        ref_mut.push_str(" Alterado");
        println!("Dentro do escopo mutável: {}", ref_mut);
        // let erro = &dado; // ERRO! Não pode emprestar como imutável enquanto mutável existe.
    } // ref_mut sai de escopo aqui.

    println!("Após empréstimo mutável: {}", dado); // OK!


    /* 
       4. O PAPEL DO BORROW CHECKER (RESUMO)
       - Garante que referências nunca apontem para memória inválida (dangling pointers).
       - Impede "Data Races" (corrida de dados) em tempo de compilação.
    */
    let resultado = processar_dados(&dado);
    println!("Resultado do processamento: {}", resultado);
}

fn processar_dados(texto: &str) -> usize {
    texto.len() // Apenas lê os dados, sem tomar posse (ownership)
}

/*
REGRAS DE OURO PARA LEMBRAR:

1. Um Dono por vez: Evita dupla liberação de memória.
2. N imutáveis OU 1 mutável: Impede modificações concorrentes que causariam bugs.
3. Referências devem ser válidas: O compilador garante que você nunca acesse 
   memória que já foi limpa.

Parabéns! Entender esses conceitos é o passo mais difícil e importante na jornada Rust.
Com Ownership e Borrowing, o Rust garante segurança de memória sem o custo de um 
Garbage Collector em tempo de execução.
*/
