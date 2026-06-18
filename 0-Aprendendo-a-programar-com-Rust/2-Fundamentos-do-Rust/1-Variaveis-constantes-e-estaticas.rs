// Variáveis, Constantes e Estáticas em Rust

/*
Rust oferece três formas principais de declarar nomes para valores: 'let', 'const' e 'static'.
Cada uma tem propósitos e comportamentos diferentes em relação à memória e ao ciclo de vida do programa.
*/

// 1. Constantes (const)
// - Representam valores que nunca mudam.
// - São substituídas pelo valor literal em todos os lugares onde são usadas durante a compilação (inlining).
// - Devem ter o tipo explicitamente anotado.
// - Podem ser declaradas em qualquer escopo, inclusive global.
const LIMITE_MAXIMO: u32 = 1000;

// 2. Variáveis Estáticas (static)
// - Representam um local fixo na memória que dura por todo o tempo de execução do programa ('static lifetime).
// - Ao contrário das constantes, elas têm um endereço de memória real.
// - São úteis para dados globais grandes ou quando você precisa de um endereço de memória fixo.
static NOME_DO_SISTEMA: &str = "RustOS";

// Estáticas Mutáveis (Uso avançado e perigoso)
// Variáveis estáticas podem ser mutáveis, mas o acesso a elas é intrinsecamente inseguro (unsafe) 
// devido ao risco de condições de corrida em programas multi-thread.
static mut CONTADOR_GLOBAL: u32 = 0;

fn main() {
    // 3. Variáveis Locais (let)
    // - Criadas na pilha (stack) durante a execução.
    // - Ocupam memória apenas enquanto o escopo estiver ativo.
    // - Imutáveis por padrão, mas podem ser 'mut'.
    let versao = 1.2;
    let mut sessao_ativa = true;

    println!("Sistema: {} (Versão: {})", NOME_DO_SISTEMA, versao);
    println!("Limite configurado: {}", LIMITE_MAXIMO);

    // Exemplo de acesso a estática mutável (requer bloco unsafe)
    unsafe {
        CONTADOR_GLOBAL += 1;
        println!("Acessos globais: {}", CONTADOR_GLOBAL);
    }

    /*
    Resumo das Diferenças:

    | Característica | let                 | const                   | static                  |
    |----------------|---------------------|-------------------------|-------------------------|
    | Escopo         | Local (Bloco)       | Qualquer (Global/Local) | Qualquer (Global/Local) |
    | Mutabilidade   | Sim (com mut)       | Nunca                   | Sim (com mut + unsafe)  |
    | Tipo           | Inferido/Opcional   | Obrigatório             | Obrigatório             |
    | Valor          | Tempo de Execução   | Tempo de Compilação     | Tempo de Compilação     |
    | Memória        | Pilha (Stack)       | Inlining (Substituição) | Endereço Fixo (Data)    |
    */
}

fn outro_escopo() {
    // println!("{}", versao); // ERRO! 'versao' (let) não existe aqui.
    println!("Acesso global ok: {}", LIMITE_MAXIMO); // OK! Constantes são globais.
    println!("Acesso estático ok: {}", NOME_DO_SISTEMA); // OK! Estáticas são globais.
}
