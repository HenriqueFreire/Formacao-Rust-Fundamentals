// 4-Porque-o-Rust-e-considerado-linguagem-de-baixo-nível.rs

/*
 * POR QUE RUST É CONSIDERADO UMA LINGUAGEM DE BAIXO NÍVEL?
 * 
 * Uma linguagem de "baixo nível" é aquela que oferece controle direto sobre o hardware 
 * e a memória do sistema. Rust é frequentemente chamado de "linguagem de programação de sistemas".
 *
 * RAZÕES PRINCIPAIS:
 * 1. GERENCIAMENTO DE MEMÓRIA SEM GARBAGE COLLECTOR:
 *    Diferente de Java, Python ou Go, o Rust não tem um coletor de lixo. 
 *    Ele usa o sistema de Ownership para liberar memória exatamente quando ela não é mais necessária.
 *
 * 2. CONTROLE SOBRE O LAYOUT DE MEMÓRIA:
 *    Você pode decidir exatamente como seus dados são organizados na memória (Stack vs Heap).
 *
 * 3. ACESSO DIRETO AO HARDWARE E PONTEIROS:
 *    Através de blocos unsafe, o Rust permite manipular ponteiros brutos (raw pointers),
 *    essencial para escrever drivers de dispositivo ou kernels de sistemas operacionais.
 *
 * 4. SEM RUNTIME PESADO:
 *    O código Rust compila para código de máquina nativo, quase sem nenhuma camada entre
 *    o código e o sistema operacional.
 */

fn main() {
    println!("--- Por que Rust é de Baixo Nível? ---");

    // EXEMPLO 1: Stack vs Heap
    // Baixo nível significa entender onde os dados residem.
    exemplo_stack_heap();

    // EXEMPLO 2: Manipulação de Ponteiros Brutos (Unsafe Rust)
    // Mostra como o Rust pode agir como C quando necessário.
    exemplo_ponteiro_bruto();
}

fn exemplo_stack_heap() {
    // Alocado na STACK (Pilha): Rápido, tamanho fixo conhecido em tempo de compilação.
    let x = 10;
    
    // Alocado na HEAP (Monte): Dinâmico, tamanho pode mudar em tempo de execução.
    let y = Box::new(20); 

    println!("x (stack): {}, y (heap): {}", x, y);
}

fn exemplo_ponteiro_bruto() {
    let mut numero = 5;

    // Criando ponteiros brutos (Raw Pointers)
    let p1 = &numero as *const i32;
    let p2 = &mut numero as *mut i32;

    // Para desreferenciar (acessar o valor), precisamos de um bloco unsafe.
    // Isso é "baixo nível": o programador assume a responsabilidade que o compilador normalmente teria.
    unsafe {
        println!("p1 aponta para: {}", *p1);
        *p2 = 10;
        println!("Valor alterado via ponteiro bruto: {}", *p2);
    }
}
