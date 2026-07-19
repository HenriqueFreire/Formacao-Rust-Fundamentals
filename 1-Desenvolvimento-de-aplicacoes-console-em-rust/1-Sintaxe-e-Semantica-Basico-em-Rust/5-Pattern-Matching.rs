/*
===============================================================================
TEMA: Pattern Matching (A Arte do Controle de Fluxo Exaustivo)
===============================================================================

A principal ferramenta de Pattern Matching no Rust é a palavra-chave `match`.
Uma regra de ouro do `match`: ele é **exaustivo**. Isso significa que o compilador 
não vai compilar o seu código se você esquecer de tratar uma única possibilidade 
sequer do dado que está avaliando.

-------------------------------------------------------------------------------
1. ESTRUTURAS DE DADOS PARA O NOSSO TESTE
-------------------------------------------------------------------------------
*/

enum SensorStatus {
    Ativo(f32), // Carrega a leitura atual
    Calibrando,
    Erro(u32),  // Carrega o código numérico do erro
}

struct Pacote {
    origem: u8,
    destino: u8,
    carga_util: String,
}

/*
-------------------------------------------------------------------------------
2. EXECUÇÃO PRÁTICA E CASOS DE USO
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("       TESTANDO PATTERN MATCHING        ");
    println!("========================================");

    // -------------------------------------------------------------------------
    // CASO 1: Casamento Básico com Ranges (Intervalos)
    // -------------------------------------------------------------------------
    let codigo_componente = 45;

    match codigo_componente {
        1 => println!("Componente: Resistor"),
        2 | 3 | 4 => println!("Componente: Capacitor ou Indutor"), // Operador OU (`|`)
        10..=50 => println!("Componente: Circuito Integrado de Série Especial"), // Range inclusivo `..=`
        _ => println!("Componente Desconhecido"), // O braço padrão/fallback (`_`)
    }

    // -------------------------------------------------------------------------
    // CASO 2: Desestruturando Enums e usando Match Guards (Condicionais extras)
    // -------------------------------------------------------------------------
    let status_sensor = SensorStatus::Ativo(42.5);

    match status_sensor {
        SensorStatus::Calibrando => println!("O sensor está se preparando..."),
        
        // Usando um "Match Guard" (um `if` adicional dentro do braço do match)
        SensorStatus::Ativo(temp) if temp > 40.0 => {
            println!("[PERIGO] Sensor ativo e superaquecido: {}°C!", temp);
        }
        
        // Captura qualquer outro valor ativo que não entrou na condição acima
        SensorStatus::Ativo(temp) => println!("Sensor operando normalmente a {}°C.", temp),
        
        // Tratando a variante de erro e extraindo o código interno
        SensorStatus::Erro(codigo) => println!("Sensor travado com código de erro físico: Ox{:X}", codigo),
    }

    // -------------------------------------------------------------------------
    // CASO 3: Desestruturando Structs por completo
    // -------------------------------------------------------------------------
    let meu_pacote = Pacote {
        origem: 1,
        destino: 255, // 255 costuma ser o endereço de Broadcast na rede
        carga_util: String::from("PING"),
    };

    // Podemos extrair os dados de dentro da struct no próprio casamento
    match meu_pacote {
        // Se o destino for 255, captura a origem e a mensagem ignorando o resto se quisesse
        Pacote { origem, destino: 255, carga_util } => {
            println!("Pacote de Broadcast enviado por nó {} contendo: '{}'", origem, carga_util);
        }
        // Braço genérico para pacotes normais unicast
        Pacote { origem, destino, .. } => {
            println!("Pacote direcionado do nó {} para o nó {}", origem, destino);
        }
    }

    // -------------------------------------------------------------------------
    // CASO 4: O Operador `@` (Binding / Vinculação)
    // -------------------------------------------------------------------------
    let id_dispositivo = 15;

    match id_dispositivo {
        // O `@` nos permite testar se o valor está na faixa E ao mesmo tempo 
        // salvá-lo em uma variável para usar dentro do bloco do braço.
        faixa_critica @ 10..=20 => {
            println!("ID {} está dentro da faixa crítica de hardware reservada!", faixa_critica);
        }
        _ => println!("ID de dispositivo comum."),
    }
}

/*
-------------------------------------------------------------------------------
VISÃO DE ENGENHARIA DA COMPUTAÇÃO (Otimização do compilador):
-------------------------------------------------------------------------------
Você pode pensar que uma sequência longa de verificações no `match` se transforma 
em um monte de instruções `if/else` lentas no processador (o que causaria problemas 
de previsão de salto/branch misprediction).

Porém, o compilador do Rust (LLVM) é extremamente inteligente. Quando você faz um 
`match` sobre números sequenciais ou variantes de enums, ele frequentemente compila 
essa lógica gerando uma **Jump Table** (Tabela de Saltos) direto em Assembly. 

Em vez de testar linha por linha, o processador calcula matematicamente o endereço 
do código correto e pula direto para ele em tempo de execução constante ($O(1)$). 
É por isso que o Pattern Matching em Rust é assustadoramente rápido!
    */
