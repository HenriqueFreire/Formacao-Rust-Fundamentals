/*
===============================================================================
TEMA: Destructuring (Desestruturação de Dados)
===============================================================================

A desestruturação no Rust pode ser usada tanto em atribuições comuns com `let`, 
quanto em parâmetros de funções, loops `for` ou braços de `match`.

-------------------------------------------------------------------------------
1. ESTRUTURAS DE SUPORTE
-------------------------------------------------------------------------------
*/

struct CoordenadaGps {
    latitude: f64,
    longitude: f64,
    altitude: f32,
}

enum SinalDigital {
    Alto,
    Baixo,
    Modulado(u32, f32), // Frequência e Amplitude
}

/*
-------------------------------------------------------------------------------
2. EXECUÇÃO PRÁTICA
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("       TESTANDO DESESTRUTURAÇÃO         ");
    println!("========================================");

    // -------------------------------------------------------------------------
    // A) DESESTRUTURANDO TUPLAS
    // -------------------------------------------------------------------------
    let leitura_sensor = (22.5, 1013.2, "OK");
    
    // Extraímos os três valores de uma só vez para variáveis independentes
    let (temperatura, pressao, status) = leitura_sensor;
    println!("Tupla desestruturada -> Temp: {}°C, Pressão: {}hPa, Status: {}", temperatura, pressao, status);

    // Ignorando valores: Se você só quisesse a temperatura, usaria o underscore `_`
    let (temp_apenas, _, _) = leitura_sensor;
    println!("Apenas temperatura extraída: {}", temp_apenas);


    // -------------------------------------------------------------------------
    // B) DESESTRUTURANDO STRUCTS
    // -------------------------------------------------------------------------
    let ponto_local = CoordenadaGps {
        latitude: -23.5505,
        longitude: -46.6333,
        altitude: 760.0,
    };

    // 1. Desestruturação padrão criando variáveis com os mesmos nomes dos campos
    let CoordenadaGps { latitude, longitude, altitude } = ponto_local;
    println!("\nStruct -> Lat: {}, Lon: {}, Alt: {}m", latitude, longitude, altitude);

    // 2. Renomeando variáveis durante a desestruturação (util se houver colisão de nomes)
    let CoordenadaGps { latitude: lat, longitude: lon, .. } = ponto_local;
    // O operador `..` diz ao Rust para ignorar o resto dos campos (altitude)
    println!("Campos renomeados -> lat local: {}, lon local: {}", lat, lon);


    // -------------------------------------------------------------------------
    // C) DESESTRUTURANDO ARRAYS / SLICES
    // -------------------------------------------------------------------------
    let comandos_maquina = [0x01, 0xAA, 0xBB, 0x00, 0x00, 0x02];

    // Podemos extrair os primeiros elementos e agrupar o resto em um sub-slice
    let [opcode, param1, param2, ref resto @ ..] = comandos_maquina;
    println!("\nArray -> Opcode: 0x{:X}, Params: [0x{:X}, 0x{:X}]", opcode, param1, param2);
    println!("Bytes restantes no buffer: {:?}", resto);


    // -------------------------------------------------------------------------
    // D) DESESTRUTURAÇÃO EM ENUMS (Via Match ou If Let)
    // -------------------------------------------------------------------------
    let sinal = SinalDigital::Modulado(440, 5.0);

    // Extraindo os dados de dentro da variante Modulado
    if let SinalDigital::Modulado(frequencia, amplitude) = sinal {
        println!("\nEnum -> Sinal modulado detectado a {} Hz com {}V", frequencia, amplitude);
    }


    // -------------------------------------------------------------------------
    // E) APLICAÇÃO EM LOOPS (Muito comum no dia a dia)
    // -------------------------------------------------------------------------
    println!("\n--- Desestruturação dentro de um Loop For ---");
    let lista_leituras = vec![
        (101, 36.5),
        (102, 38.2),
        (103, 41.0),
    ];

    // Desestruturamos a tupla diretamente na assinatura do loop for!
    for (id_sensor, valor) in lista_leituras {
        println!("Sensor ID {}: Gerou leitura de {} unidade(s).", id_sensor, valor);
    }
}

/*
-------------------------------------------------------------------------------
VISÃO DE ENGENHARIA DA COMPUTAÇÃO (O custo de abstração é ZERO):
-------------------------------------------------------------------------------
Muitos programadores vindos de linguagens interpretadas (como JavaScript) associam 
a desestruturação à criação de cópias ou ao desperdício de performance na CPU.

No Rust, a desestruturação é pura açúcar sintático (*syntactic sugar*). Em nível 
de hardware, o compilador traduz isso exatamente como acessos diretos aos offsets 
de memória originais na Stack. Não há alocação de memória extra, não há clones 
invisíveis e nenhuma instrução extra de processamento é gerada. 

É a filosofia de "Abstração de Custo Zero" do Rust em sua melhor forma: código 
limpo para o engenheiro ler, binário enxuto para o processador executar.
*/
