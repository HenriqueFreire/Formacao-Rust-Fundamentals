/*
===============================================================================
TEMA: Gerando o Binário de uma Aplicação Rust (Compilação e Artefatos)
===============================================================================

No Rust, existem duas formas principais de gerar o binário (o arquivo executável) 
da sua aplicação: o modo de Desenvolvimento (Debug) e o modo de Produção (Release). 

Cada um deles altera drasticamente o tamanho do arquivo final e a velocidade 
com que o seu programa é executado.

-------------------------------------------------------------------------------
1. MODO DE DESENVOLVIMENTO (Debug)
-------------------------------------------------------------------------------
Quando você executa o comando tradicional do Cargo:
$ cargo build
Ou quando usa o `cargo run`, o Rust compila o seu código no modo Debug.

Características:
- Compilação muito mais rápida.
- Inclui "Símbolos de Debug" (informações extras para ferramentas de rastreio de erros).
- Não aplica otimizações pesadas no código.
- O executável fica localizado em: `./target/debug/nome_do_seu_projeto`

-------------------------------------------------------------------------------
2. MODO DE PRODUÇÃO (Release) - O Binário Oficial
-------------------------------------------------------------------------------
Quando você terminou o software e vai enviá-lo para o cliente, para um servidor, 
ou para rodar num sistema embarcado (foco da Engenharia da Computação), você deve 
gerar o binário otimizado usando a flag `--release`:
$ cargo build --release

Características:
- A compilação demora bem mais (o compilador reconstrói loops, remove código morto, etc.).
- Remove todos os símbolos de debug.
- O código roda até 10x ou 100x mais rápido que no modo Debug!
- O executável final fica em: `./target/release/nome_do_seu_projeto`

-------------------------------------------------------------------------------
3. EXEMPLO PRÁTICO PARA MEDIR A PERFORMANCE:
-------------------------------------------------------------------------------
*/

use std::time::Instant;

fn main() {
    println!("====================================================");
    println!("   TESTE DE COMPILAÇÃO: BINÁRIO DEBUG VS RELEASE   ");
    println!("====================================================");

    // Vamos criar uma operação pesada de processamento matemático 
    // para ver como o compilador otimiza o binário final.
    let agora = Instant::now();
    
    let mut contador: u64 = 0;
    for i in 0..50_000_000 {
        // Uma operação bitwise simples (comum em Engenharia da Computação)
        contador = contador.wrapping_add(i ^ 2);
    }

    let duracao = agora.elapsed();
    
    println!("Resultado do processamento: {}", contador);
    println!("Tempo total gasto nesta execução: {:?}", duracao);

    /*
       EXERCÍCIO PARA ENGENHARIA DA COMPUTAÇÃO:
       
       1. Abra o terminal e execute em modo Debug:
          $ cargo run
          (Anote o tempo de execução exibido no terminal).
          
       2. Agora, gere o binário otimizado de produção e execute:
          $ cargo run --release
          (Compare o tempo. A diferença é brutal!).
          
       3. Vá até a pasta do seu projeto usando o gerenciador de arquivos:
          - Olhe o tamanho do arquivo dentro de `target/debug/`
          - Olhe o tamanho do arquivo dentro de `target/release/`
          O binário de release será consideravelmente menor e autônomo (pode
          ser movido para qualquer computador sem precisar do Rust instalado).
    */
}

/*
-------------------------------------------------------------------------------
DICA DE ENGENHARIA DA COMPUTAÇÃO (Cross-Compilation / Compilação Cruzada):
-------------------------------------------------------------------------------
Como futuro Engenheiro da Computação, você frequentemente escreverá código no seu 
computador (x86_64) mas precisará que o binário rode numa placa de desenvolvimento 
ou microcontrolador (como um processador ARM ou RISC-V).

O Rust facilita muito a "compilação cruzada". Você pode dizer ao Cargo para qual 
arquitetura de processador (Target) deseja gerar o binário.

Exemplo de comando (para adicionar e gerar um binário focado em sistemas ARM64):
$ rustup target add aarch64-unknown-linux-gnu
$ cargo build --release --target aarch64-unknown-linux-gnu

Isso gerará o binário exato para rodar nativamente em dispositivos como uma 
Raspberry Pi ou servidores ARM na nuvem.
*/
