/*
===============================================================================
TEMA: Traits (Polimorfismo e Contratos de Comportamento)
===============================================================================

Uma trait define uma interface comum que múltiplos tipos podem compartilhar.
Ela pode conter:
1. Assinaturas de métodos (que o tipo é OBRIGADO a implementar).
2. Métodos padrão (com corpo, que o tipo ganha "de graça", mas pode sobrescrever).

-------------------------------------------------------------------------------
1. DEFINIÇÃO DA TRAIT
-------------------------------------------------------------------------------
*/

pub trait DispositivoIot {
    // Assinatura de método: qualquer struct que queira ser um 'DispositivoIot'
    // deve fornecer sua própria implementação deste método.
    fn transmitir_dados(&self) -> String;

    // Método Padrão: structs ganham essa lógica automaticamente, a menos que 
    // decidam customizá-la (sobrescrever).
    fn checar_status(&self) {
        println!("[SISTEMA] Verificando integridade física do hardware...");
    }
}

/*
-------------------------------------------------------------------------------
2. CRIANDO AS STRUCTS E IMPLEMENTANDO A TRAIT
-------------------------------------------------------------------------------
*/

struct Termometro {
    id: u32,
    temperatura: f32,
}

// Implementando a trait 'DispositivoIot' para a nossa struct 'Termometro'
impl DispositivoIot for Termometro {
    fn transmitir_dados(&self) -> String {
        format!("{{\"id\": {}, \"temp\": {:.1}°C}}", self.id, self.temperatura)
    }
}

struct MedidorVazao {
    codigo: String,
    litros_por_segundo: f64,
}

impl DispositivoIot for MedidorVazao {
    fn transmitir_dados(&self) -> String {
        format!("{{\"sensor\": \"{}\", \"fluxo\": {:.2} L/s}}", self.codigo, self.litros_por_segundo)
    }

    // Sobrescrevendo o método padrão para dar um aviso customizado
    fn checar_status(&self) {
        println!("[ALERTA-VAZÃO] Checando calibração das hélices do medidor {}...", self.codigo);
    }
}

/*
-------------------------------------------------------------------------------
3. CONSUMINDO O POLIMORFISMO (Trait Bounds)
-------------------------------------------------------------------------------
Para que serve ter uma trait? Para criarmos funções genéricas que aceitam 
QUALQUER tipo, desde que ele obedeça ao nosso contrato (trait).
*/

// Lemos isto como: "Esta função aceita qualquer argumento 'dispositivo' do tipo T, 
// desde que T implemente a trait 'DispositivoIot'".
fn processar_telemetria<T: DispositivoIot>(dispositivo: &T) {
    println!("--- Processando Pacote de Rede ---");
    dispositivo.checar_status(); // Chama o método correto de cada um
    let json = dispositivo.transmitir_dados();
    println!("Payload enviado para a nuvem: {}", json);
}

/*
-------------------------------------------------------------------------------
4. EXECUÇÃO PRÁTICA
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("          TRABALHANDO COM TRAITS        ");
    println!("========================================");

    let sensor_sala_a = Termometro { id: 42, temperatura: 24.8 };
    let cano_principal = MedidorVazao { codigo: String::from("M-101"), litros_por_segundo: 15.4 };

    // 1. Chamando os métodos da trait diretamente nas instâncias
    println!("Leitura direta Termômetro: {}", sensor_sala_a.transmitir_dados());
    
    // 2. Vendo o método padrão em ação vs método sobrescrito
    sensor_sala_a.checar_status();   // Usa a implementação padrão da trait
    cano_principal.checar_status();  // Usa a lógica customizada que escrevemos nela

    println!("\n----------------------------------------");

    // 3. Passando tipos totalmente diferentes para a mesma função genérica!
    processar_telemetria(&sensor_sala_a);
    processar_telemetria(&cano_principal);
}

/*
-------------------------------------------------------------------------------
VISÃO DE ENGENHARIA DA COMPUTAÇÃO (Monomorfização / Static Dispatch):
-------------------------------------------------------------------------------
Em linguagens como Java ou C++, o polimorfismo via interfaces geralmente causa 
um custo em tempo de execução devido ao *Dynamic Dispatch* (onde o programa precisa 
consultar uma tabela de ponteiros virtuais - vtable - para descobrir qual função chamar).

No Rust, quando você usa a sintaxe `<T: DispositivoIot>`, o compilador faz algo 
chamado **Monomorfização**. Em tempo de compilação, ele analisa quais tipos reais 
usaram aquela função e duplica o código em Assembly para cada um deles.

Em nível de hardware, a função `processar_telemetria` se divide em duas funções 
reais e diretas no binário final. Isso elimina totalmente a lentidão de busca de 
ponteiros na memória, resultando em chamadas de função diretas e ultra velozes. 
Mais uma vez: Abstração de Custo Zero!
*/
