/*
===============================================================================
TEMA: Módulos Agrupados em Arquivos (Estilo Moderno - Rust 2018+)
===============================================================================

Por que usar este padrão em vez do antigo `mod.rs`?
1. Evita o problema de ter 10 arquivos abertos no seu editor de texto chamados 
   `mod.rs`, sem você saber de qual pasta cada um é.
2. Mantém a navegação de arquivos muito mais intuitiva.

-------------------------------------------------------------------------------
*/

// Declaramos que o nosso projeto possui o módulo 'rede'.
// O Rust vai procurar pelo arquivo `src/rede.rs`.
mod rede;

fn main() {
    println!("========================================");
    println!("    MÓDULOS AGRUPADOS EM ARQUIVOS       ");
    println!("========================================");

    // Chamando a função do arquivo pai (`src/rede.rs`)
    rede::inicializar_interface_rede();

    // Chamando as funções dos arquivos filhos de dentro da pasta (`src/rede/`)
    rede::serial::configurar_baud_rate(115200);
    rede::ethernet::configurar_ip("192.168.1.50");

    println!("\n----------------------------------------");

    // Usando atalhos aninhados para limpar o código
    use rede::{serial, ethernet};
    
    serial::configurar_baud_rate(9600);
    ethernet::configurar_ip("10.0.0.1");
}
