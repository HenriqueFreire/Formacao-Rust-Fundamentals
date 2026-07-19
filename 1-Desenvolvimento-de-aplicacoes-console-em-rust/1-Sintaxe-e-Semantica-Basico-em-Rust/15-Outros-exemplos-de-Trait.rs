/*
===============================================================================
TEMA: Exemplos Avançados de Traits (Sobrecarga, Ciclo de Vida e Extensão)
===============================================================================

Neste arquivo vamos explorar três conceitos fundamentais:
1. Sobrecarga de Operadores (Trait `Add`): Fazer o sinal de `+` funcionar em structs.
2. Gerenciamento de Recursos (Trait `Drop`): O destruidor automático de hardware.
3. Extension Traits: Adicionar métodos novos a tipos nativos do Rust (como `i32`).

-------------------------------------------------------------------------------
1. SOBRECARGA DE OPERADORES (Trait `std::ops::Add`)
-------------------------------------------------------------------------------
*/

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Vetor2D {
    x: f32,
    y: f32,
}

// Implementando a trait `Add` para permitir a soma física de dois vetores
impl Add for Vetor2D {
    type Output = Vetor2D; // Tipo que será retornado pela operação

    // Sobrescreve o comportamento do operador `+`
    fn add(self, outro: Vetor2D) -> Self::Output {
        Vetor2D {
            x: self.x + outro.x,
            y: self.y + outro.y,
        }
    }
}

/*
-------------------------------------------------------------------------------
2. DESTRUIDORES AUTOMÁTICOS (Trait `Drop`)
-------------------------------------------------------------------------------
No Rust, não existe coletor de lixo (Garbage Collector). Quando uma variável sai 
de escopo, ela é limpa. A trait `Drop` permite injetar um código customizado 
nesse exato milissegundo de destruição. Útil para fechar conexões ou desligar pinos.
*/

struct ConexaoUart {
    porta: String,
}

impl Drop for ConexaoUart {
    fn drop(&mut self) {
        // Este código roda sozinho no fim do ciclo de vida da struct
        println!("[RAII] Fechando o descritor de arquivo e liberando a porta {} de forma segura!", self.porta);
    }
}

/*
-------------------------------------------------------------------------------
3. EXTENSION TRAITS (Injetando superpoderes em tipos nativos)
-------------------------------------------------------------------------------
Você pode criar uma trait sua e implementá-la em um tipo que NÃO é seu (como `i32`).
Isso se chama "Extension Trait".
*/

pub trait ConversorFrequencia {
    fn para_khz(&self) -> f64;
}

// Implementando nossa trait para o tipo nativo i32 do Rust!
impl ConversorFrequencia for i32 {
    fn para_khz(&self) -> f64 {
        *self as f64 / 1000.0
    }
}

/*
-------------------------------------------------------------------------------
4. EXECUÇÃO PRÁTICA
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("        OUTROS EXEMPLOS DE TRAITS       ");
    println!("========================================");

    // 1. Testando Sobrecarga do Operador `+`
    let v1 = Vetor2D { x: 1.5, y: 2.0 };
    let v2 = Vetor2D { x: 3.0, y: 4.5 };
    
    // O sinal de + evoca o método .add() da nossa trait por baixo dos panos!
    let v_resultado = v1 + v2; 
    println!("Resultado da soma de vetores físicos: {:?}", v_resultado);

    println!("\n----------------------------------------");

    // 2. Testando Extension Traits em tipos nativos
    let clock_hz: i32 = 400_000; // 400kHz (Frequência comum de barramento I2C)
    println!("Clock do barramento: {} kHz", clock_hz.para_khz());

    println!("\n----------------------------------------");

    // 3. Testando a Trait `Drop` (Padrão RAII - Resource Acquisition Is Initialization)
    println!("Abrindo escopo interno artificial...");
    {
        let _conexao_gps = ConexaoUart { porta: String::from("/dev/ttyUSB0") };
        println!("Usando a conexão serial dentro do escopo...");
        // _conexao_gps vai morrer na linha abaixo. Watch out no terminal!
    } 
    println!("Escopo interno encerrado.");
}
