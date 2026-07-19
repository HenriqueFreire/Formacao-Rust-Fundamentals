/*
===============================================================================
TEMA: Módulos em Arquivos Separados (Irmãos no mesmo nível)
===============================================================================

Uma dúvida muito comum de quem vem do C/C++ ou de linguagens como Python é: 
"Se `matematica.rs` e `utilitarios.rs` estão na mesma pasta que o `main.rs`, 
por que eu não consigo usar as funções deles direto ou usando um include?"

No Rust, a existência de um arquivo no disco NÃO significa que ele faz parte 
do programa. Você precisa explicitamente registrar o arquivo na raiz do projeto 
usando a palavra-chave `mod`.

-------------------------------------------------------------------------------
*/

// 1. Registrando os arquivos irmãos como módulos do projeto
mod matematica;
mod utilitarios;

fn main() {
    println!("========================================");
    println!("     MÓDULOS EM ARQUIVOS SEPARADOS      ");
    println!("========================================");

    // Valores de componentes eletrônicos simulados (Engenharia da Computação)
    let l = 0.001; // 1 mH (Indutância)
    let c = 0.000_000_1; // 100 nF (Capacitância)

    // 2. Chamando a função do módulo `matematica.rs`
    let freq = matematica::calcular_frequencia_resonancia(l, c);

    // 3. Chamando a função do módulo `utilitarios.rs`
    let resultado_formatado = utilitarios::formatar_unidade(freq, "Hz");

    println!("Dados do Circuito RLC:");
    println!("Frequência de Ressonância: {}", resultado_formatado);

    println!("\n----------------------------------------");

    // --- REEXPORTAÇÃO COM `pub use` (Conceito Avançado de Organização) ---
    // Você pode usar o `use` dentro do main para simplificar as chamadas:
    use matematica::calcular_frequencia_resonancia;
    
    let nova_freq = calcular_frequencia_resonancia(0.002, 0.000_000_05);
    println!("Nova frequência calculada de forma direta: {:.2} Hz", nova_freq);
}
