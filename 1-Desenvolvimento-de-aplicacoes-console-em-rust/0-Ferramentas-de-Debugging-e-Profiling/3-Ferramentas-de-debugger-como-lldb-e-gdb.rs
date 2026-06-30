/*
===============================================================================
TEMA: Depuração de Código com LLDB e GDB no Rust
===============================================================================

O que são o GDB e o LLDB?
- GDB: O depurador clássico do projeto GNU, amplamente usado no ecossistema Linux/GCC.
- LLDB: O depurador moderno do projeto LLVM. Como o compilador do Rust (rustc) 
  é baseado no LLVM, o LLDB costuma ter uma integração ligeiramente mais natural, 
  especialmente no macOS e Windows (via VS Code).

A comunidade Rust fornece versões customizadas de ambos (`rust-gdb` e `rust-lldb`) 
que já vêm instaladas com o Rustup. Elas servem para traduzir tipos complexos do 
Rust (como Strings, Vectors e Enums) em texto legível para nós durante a inspeção.

-------------------------------------------------------------------------------
1. PREPARANDO O BINÁRIO PARA DEPURAÇÃO:
-------------------------------------------------------------------------------
Para debugar, você DEVE usar o binário gerado em modo **Debug** (`cargo build`). 
Se tentar debugar um binário gerado com `--release`, o compilador terá otimizado e 
reorganizado o código de tal forma que o debugger não conseguirá associar as linhas 
do seu arquivo `.rs` com as instruções que o processador está executando.

-------------------------------------------------------------------------------
2. COMANDOS ESSENCIAIS NO TERMINAL (GDB/LLDB):
-------------------------------------------------------------------------------
Abra o terminal na pasta do projeto e inicie o debugger apontando para o binário:
$ rust-lldb ./target/debug/nome_do_seu_projeto
(Ou `rust-gdb ./target/debug/nome_do_seu_projeto`)

Dentro do painel do debugger, use estes comandos de controle:

- `b main.rs:48` -> (Breakpoint) Diz ao programa para pausar na linha 48.
- `r` ou `run`  -> Inicializa a execução do programa até encontrar o breakpoint.
- `n` ou `next` -> Executa a próxima linha de código (passa por cima de funções).
- `s` ou `step` -> Entra dentro da função da linha atual.
- `p minha_variavel` -> (Print) Mostra o valor atual de uma variável.
- `c` ou `continue` -> Retoma a execução normal até o próximo breakpoint ou fim do app.
- `q` ou `quit` -> Sai do debugger.

-------------------------------------------------------------------------------
3. EXEMPLO PRÁTICO (O código com comportamento inesperado):
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("      RELAXANDO COM O DEBUGGER          ");
    println!("========================================");

    let mut saldo = 1000.0;
    let compras = [150.0, 450.0, 500.0];

    println!("Saldo inicial: R$ {}", saldo);

    // Imagine que este loop roda e, ao final, o saldo fica negativo, 
    // mas você não sabe exatamente em qual iteração o erro aconteceu.
    for compra in compras.iter() {
        saldo = debugar_transacao(saldo, *compra);
    }

    println!("Saldo final: R$ {}", saldo);
}

fn debugar_transacao(saldo_atual: f64, valor_compra: f64) -> f64 {
    // Se você colocar um Breakpoint nesta linha pelo terminal ou pelo editor,
    // poderá inspecionar os valores de `saldo_atual` e `valor_compra` a cada ciclo.
    let novo_saldo = saldo_atual - valor_compra;
    
    // Aqui simulamos uma regra de negócio que pode disparar um comportamento estranho
    if novo_saldo < 0.0 {
        println!("Alerta: Compra de R$ {} excedeu o saldo!", valor_compra);
    }
    
    novo_saldo
}

/*
-------------------------------------------------------------------------------
BOA PRÁTICA DE ENGENHARIA DA COMPUTAÇÃO (Interface Gráfica vs Terminal):
-------------------------------------------------------------------------------
Embora conhecer os comandos `b`, `r` e
