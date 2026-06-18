// Exercício: Montando um Menu com Loops em Rust

/*
    Para criar um menu interativo em Rust, geralmente usamos:
    1. Um `loop` infinito para manter o programa rodando.
    2. Uma forma de capturar a entrada do usuário (ou simular via variável).
    3. Um `match` ou `if/else` para processar a escolha.
    4. O comando `break` para sair do programa.
*/

fn main() {
    // Exemplo de Menu usando `loop` e Simulação de Entrada
    
    let mut opcao_simulada = 1; // Vamos simular que o usuário escolhe 1, depois 2, depois 0

    println!("=== BEM-VINDO AO SISTEMA RUST ===");

    loop {
        println!("\nEscolha uma opção:");
        println!("1. Ver Saldo");
        println!("2. Depositar");
        println!("0. Sair");

        // Em um programa real, usaríamos std::io::stdin() para ler do teclado.
        // Aqui simularemos a mudança de escolha para demonstrar o loop:
        let escolha = opcao_simulada;

        if escolha == 1 {
            println!(">> Seu saldo atual é R$ 1.000,00");
            opcao_simulada = 2; // Simulando próxima interação
        } else if escolha == 2 {
            println!(">> Depósito de R$ 50,00 realizado com sucesso!");
            opcao_simulada = 0; // Simulando saída na próxima interação
        } else if escolha == 0 {
            println!(">> Saindo do sistema... Até logo!");
            break; // O break é essencial para encerrar o loop do menu
        } else {
            println!(">> Opção inválida!");
            break;
        }
    }

    /*
        DICA: Como ler a entrada real do usuário?
        Para capturar o que o usuário digita no terminal, você precisaria de algo assim:

        use std::io;

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler");
        let escolha: i32 = input.trim().parse().expect("Digite um número!");
    */

    /*
        EXERCÍCIO PRÁTICO:
        Crie um menu para um sistema de pedidos de uma lanchonete:
        1. Hambúrguer
        2. Batata Frita
        3. Refrigerante
        0. Finalizar Pedido
    */

    println!("\n--- LANCHONETE RUST ---");
    let mut pedido_concluido = false;
    let mut total = 0.0;
    let mut contador_simulado = 1;

    while !pedido_concluido {
        println!("1. Hambúrguer (R$ 15,00)");
        println!("2. Batata Frita (R$ 8,00)");
        println!("0. Finalizar");

        let item = contador_simulado;

        match item {
            1 => {
                println!("+ Hambúrguer adicionado.");
                total += 15.0;
                contador_simulado = 2;
            },
            2 => {
                println!("+ Batata Frita adicionada.");
                total += 8.0;
                contador_simulado = 0;
            },
            0 => {
                println!("Total do pedido: R$ {:.2}", total);
                println!("Obrigado pela preferência!");
                pedido_concluido = true; // Altera a condição do while
            },
            _ => println!("Opção inválida"),
        }
    }
}
