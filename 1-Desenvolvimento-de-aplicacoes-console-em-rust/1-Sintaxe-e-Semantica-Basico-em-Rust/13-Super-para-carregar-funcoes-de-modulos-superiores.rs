/*
===============================================================================
TEMA: Navegação com `super` para Acessar Módulos Superiores
===============================================================================

Por padrão, os módulos filhos não conseguem enxergar o que está fora deles sem 
especificar o caminho completo. O `super` resolve isso de forma relativa, subindo 
exatamente um nível na hierarquia do escopo atual.

-------------------------------------------------------------------------------
*/

// Uma constante global definida no módulo superior (Pai)
const VOLTAGEM_SISTEMA: f32 = 5.0;

pub fn registrar_log_geral(msg: &str) {
    println!("[SISTEMA PAI] {}", msg);
}

// 1. DECLARAÇÃO DO MÓDULO FILHO
mod telemetria {
    
    // 2. DECLARAÇÃO DO MÓDULO NETO (Submódulo aninhado)
    pub mod sensores {
        
        pub fn ler_sensor_corrente() {
            // Cenário: Precisamos acessar a constante e a função do módulo raiz.
            
            // Se tentarmos usar apenas `VOLTAGEM_SISTEMA`, causará erro.
            // Como estamos no módulo 'sensores', um 'super' nos leva para 'telemetria'.
            // Dois 'super' (`super::super`) nos levam para a raiz do arquivo!
            
            let v = super::super::VOLTAGEM_SISTEMA;
            println!("[NETO - SENSORES] Lendo corrente sob referência de {}V.", v);
            
            // Chamando a função do módulo pai do meu pai
            super::super::registrar_log_geral("Leitura de sensores concluída.");
        }
    }
    
    pub fn inicializar_modulo_telemetria() {
        // Como 'telemetria' é filho direto da raiz, apenas UM 'super' basta 
        // para acessar os itens superiores.
        super::registrar_log_geral("Módulo de telemetria online.");
    }
}

fn main() {
    println!("========================================");
    println!("       USANDO A PALAVRA-CHAVE SUPER     ");
    println!("========================================");

    // Executando as funções dos submódulos
    telemetria::inicializar_modulo_telemetria();
    telemetria::sensores::ler_sensor_corrente();
}
