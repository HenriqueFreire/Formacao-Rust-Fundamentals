/*
===============================================================================
TEMA: Declaração de Módulos Diretamente pelo Path (`#[path]`)
===============================================================================

Por padrão, se você escrevesse `mod validador;`, o Rust procuraria apenas por:
1. `src/validador.rs`
2. `src/validador/mod.rs`

Como nosso arquivo está fora, usamos o atributo `#[path = "..."]` logo acima 
da declaração do módulo para mudar esse comportamento. O caminho é sempre 
relativo ao arquivo onde você o está declarando.

-------------------------------------------------------------------------------
*/

// Forçando o compilador a sair de 'src' e ir para a pasta externa
#[path = "../utilitarios_globais/validador.rs"]
mod validador_externo; // O nome do módulo aqui dentro do código pode ser o que você quiser!

fn main() {
    println!("========================================");
    println!("    DECLARANDO MÓDULO DIRETAMENTE PELO PATH");
    println!("========================================");

    // Dados simulados recebidos de uma porta serial (Engenharia da Computação)
    let pacote_dados = [0x01, 0x02, 0x03, 0x04];
    let checksum_recebido = 0x04; // 1 ^ 2 ^ 3 ^ 4 = 4

    // Chamamos o módulo usando o nome que demos a ele na declaração
    let e_valido = validador_externo::validar_checksum(&pacote_dados, checksum_recebido);

    println!("Resultado da validação do pacote: {}", e_valido);
}
