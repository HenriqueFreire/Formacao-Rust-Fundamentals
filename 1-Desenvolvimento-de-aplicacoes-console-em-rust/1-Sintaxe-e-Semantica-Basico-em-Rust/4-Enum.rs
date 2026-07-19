/*
===============================================================================
TEMA: Enumerações (Enums) e o Poder do Pattern Matching (`match`)
===============================================================================

No Rust, um Enum permite definir um tipo que pode ser apenas UMA de várias 
variantes possíveis. O compilador garante que você trate todas as variantes, 
evitando bugs de estados esquecidos em produção.

-------------------------------------------------------------------------------
1. DEFINIÇÃO DE ENUMS (Do Simples ao Avançado)
-------------------------------------------------------------------------------
*/

// A) Enum Clássico (Estilo C): Ótimo para flags e estados simples
#[derive(Debug)]
enum StatusConexao {
    Desconectado,
    Conectando,
    Conectado,
}

// B) Enum Avançado (Com Dados Embutidos): Cada variante carrega dados diferentes!
// Isso substitui com muito mais segurança as hierarquias de herança orientada a objetos.
#[derive(Debug)]
enum ComandoRover {
    Parar,                                   // Sem dados extras
    Mover { direcao: char, velocidade: u8 }, // Estrutura anônima embutida
    TransmitirDados(String),                 // Tupla com tipo embutido
    AjustarGanhoAntena(f32),                 // Valor flutuante embutido
}

/*
-------------------------------------------------------------------------------
2. IMPLEMENTANDO MÉTODOS EM ENUMS
-------------------------------------------------------------------------------
Sim! Assim como as structs, os enums no Rust também podem ter blocos `impl` 
e conter métodos internos.
*/

impl ComandoRover {
    // Método que avalia o próprio enum usando `match`
    fn executar(&self) {
        // O `match` é o parceiro ideal do enum. Ele obriga você a tratar 
        // TODAS as possibilidades do enum (Exaustividade).
        match self {
            ComandoRover::Parar => {
                println!("[ROVER] Parando todos os motores imediatamente.");
            }
            // Extraindo as variáveis nomeadas de dentro da variante
            ComandoRover::Mover { direcao, velocidade } => {
                println!("[ROVER] Movendo para a direção '{}' a {} km/h.", direcao, velocidade);
            }
            // Extraindo o valor posicional da tupla
            ComandoRover::TransmitirDados(msg) => {
                println!("[ROVER] Telemetria enviada para a Terra: {}", msg);
            }
            // Se você não quiser tratar uma variante especificamente aqui, 
            // poderia usar o operador de descarte `_`, mas vamos mapear tudo:
            ComandoRover::AjustarGanhoAntena(ganho) => {
                println!("[ROVER] Ajustando ganho do sinal para: {} dB.", ganho);
            }
        }
    }
}

/*
-------------------------------------------------------------------------------
3. EXECUÇÃO PRÁTICA
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("          TRABALHANDO COM ENUMS         ");
    println!("========================================");

    // 1. Usando o Enum Simples
    let status_atual = StatusConexao::Conectando;
    println!("Status da rede: {:?}", status_atual);

    println!("\n--- Enviando Comandos para o Rover Marciano ---");

    // 2. Criando instâncias do Enum com dados
    let cmd1 = ComandoRover::Parar;
    let cmd2 = ComandoRover::Mover { direcao: 'N', velocidade: 12 };
    let cmd3 = ComandoRover::TransmitirDados(String::from("Bateria em 85%, solo coletado."));

    // Executando os comportamentos encapsulados
    cmd1.executar();
    cmd2.executar();
    cmd3.executar();

    // 3. O Atalho `if let` (Útil quando você só se importa com UMA variante)
    let comando_especial = ComandoRover::AjustarGanhoAntena(4.5);

    // Se o comando_especial for especificamente da variante AjustarGanhoAntena, extraia o valor:
    if let ComandoRover::AjustarGanhoAntena(db) = comando_especial {
        println!("\n[CONTAGEM] Verificação rápida: Antena configurada em {} dB.", db);
    }
}

/*
-------------------------------------------------------------------------------
VISÃO DE ENGENHARIA DA COMPUTAÇÃO (Representação de Memória - Tagged Unions):
-------------------------------------------------------------------------------
Por baixo dos panos, como o processador sabe qual variante do Enum está ativa 
se elas possuem tamanhos e tipos totalmente diferentes?

O Rust implementa os Enums como **Tagged Unions** (Uniões Etiquetadas). Em memória, 
o compilador reserva um pequeno espaço (geralmente 1 byte) chamado **Tag** (Etiqueta/Discriminante) 
para salvar o número da variante ativa (ex: 0 para Parar, 1 para Mover, etc.). 

O espaço total ocupado pelo Enum na memória será o tamanho da sua MAIOR variante, 
mais o tamanho da Tag.
*/
