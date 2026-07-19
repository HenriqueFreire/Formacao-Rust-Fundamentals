/*
===============================================================================
TEMA: Declaração de Módulos Diretamente no Corpo do Arquivo (Inline Modules)
===============================================================================

Quando declaramos um módulo usando chaves `{ }`, estamos criando um escopo 
isolado exatamente como se ele estivesse em outro arquivo. As regras de 
privacidade (`pub`) continuam valendo estritamente!

-------------------------------------------------------------------------------
*/

// 1. DECLARAÇÃO DO MÓDULO EMBUTIDO (INLINE)
mod controle_motor {
    // Esta struct é pública, mas seus campos são privados por padrão!
    pub struct Motor {
        pub id: u32,
        rpm_atual: u32, // Privado: Ninguém fora do módulo pode alterar diretamente
    }

    impl Motor {
        // Função associada (construtor) pública
        pub fn novo(id: u32) -> Self {
            Motor { id, rpm_atual: 0 }
        }

        // Método público para alterar o estado interno com segurança
        pub fn injetar_tensao(&mut self, voltagem: f32) {
            // Lógica de Engenharia: Converte voltagem simulada em rotação
            self.rpm_atual = (voltagem * 100.0) as u32;
            println!("[MOTOR {}] Voltagem {:.1}V aplicada. Giro atual: {} RPM", self.id, voltagem, self.rpm_atual);
        }
    }
}

// 2. USO DOS MÓDULOS NA FUNÇÃO PRINCIPAL
fn main() {
    println!("========================================");
    println!("    MÓDULOS DECLARADOS DIRETO NA MAIN   ");
    println!("========================================");

    // Para usar os elementos do módulo embutido, acessamos via caminho de escopo `::`
    let mut motor_injetor = controle_motor::Motor::novo(1);
    
    // Interagindo com o motor através da API pública dele
    motor_injetor.injetar_tensao(12.5);
    motor_injetor.injetar_tensao(24.0);

    // motor_injetor.rpm_atual = 5000;
    // ^ ERRO DE COMPILAÇÃO! Mesmo o módulo estando na main, 'rpm_atual' é privado 
    // dentro do bloco 'mod controle_motor'. Segurança de encapsulamento garantida!

    println!("\n----------------------------------------");

    // --- ENCURTANDO O CAMINHO COM O `use` ---
    // Você pode trazer o tipo de dentro do módulo para o escopo local do bloco main
    use controle_motor::Motor;

    let mut segundo_motor = Motor::novo(2);
    segundo_motor.injetar_tensao(5.0);
}
