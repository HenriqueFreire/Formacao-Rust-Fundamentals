/*
===============================================================================
TEMA: Adicionando Métodos em Structs (O Universo do bloco `impl`)
===============================================================================

Dentro de um bloco `impl`, você pode criar dois tipos de funções:
1. Funções Associadas: Não recebem `self`. Funcionam como métodos estáticos 
   (ex: construtores). São chamadas usando quatro pontos `::`.
2. Métodos Reais: Recebem `self` como primeiro parâmetro. São chamados usando 
   a sintaxe de ponto `.`.

O segredo dos métodos no Rust está em COMO eles recebem o `self`:
- `&self`: Apenas lê os dados (Empréstimo imutável).
- `&mut self`: Pode modificar os dados internos (Empréstimo mutável).
- `self`: Consome a struct (Toma a posse/ownership). A struct deixa de existir após a chamada!

-------------------------------------------------------------------------------
1. DEFINIÇÃO DA STRUCT E SEUS MÉTODOS
-------------------------------------------------------------------------------
*/

struct SensorTemperatura {
    id: u32,
    leituras: Vec<f32>, // Histórico de temperaturas em Celsius
    limite_alerta: f32,
}

impl SensorTemperatura {
    // -------------------------------------------------------------------------
    // A) FUNÇÃO ASSOCIADA (CONSTRUTOR)
    // -------------------------------------------------------------------------
    // Usada para instanciar a estrutura. Por convenção, chama-se `new`.
    // Repare: Não tem `self` nos parâmetros.
    fn new(id: u32, limite: f32) -> Self {
        Self {
            id,
            leituras: Vec::new(), // Inicializa o vetor vazio
            limite_alerta: limite,
        }
    }

    // -------------------------------------------------------------------------
    // B) MÉTODO DE LEITURA (`&self`)
    // -------------------------------------------------------------------------
    // Permite ler os dados sem alterá-los. Pode ser chamado várias vezes.
    fn calcular_media(&self) -> Option<f32> {
        if self.leituras.is_empty() {
            return None; // Retorna None se não houver leituras ainda
        }
        let soma: f32 = self.leituras.iter().sum();
        Some(soma / self.leituras.len() as f32)
    }

    // -------------------------------------------------------------------------
    // C) MÉTODO MUTÁVEL (`&mut self`)
    // -------------------------------------------------------------------------
    // Permite alterar o estado interno da struct. Exige que a instância seja `mut`.
    fn adicionar_leitura(&mut self, temperatura: f32) {
        self.leituras.push(temperatura);
        
        // Podemos chamar métodos de dentro de outros métodos usando `self.`
        if temperatura > self.limite_alerta {
            println!("[ALERTA] Sensor {}: Temperatura de {}°C excedeu o limite!", self.id, temperatura);
        }
    }

    // -------------------------------------------------------------------------
    // D) MÉTODO DE CONSUMO (`self`)
    // -------------------------------------------------------------------------
    // Raramente usado, mas vital. Ele "destrói" a struct original e pode retornar 
    // outra coisa. Útil para desligar dispositivos de forma segura.
    fn desativar_e_extrair_dados(self) -> Vec<f32> {
        println!("[SISTEMA] Desativando o sensor {} permanentemente...", self.id);
        // Retorna o vetor de leituras, transferindo a posse dele para quem chamou
        self.leituras 
    } // A struct `self` morre aqui ao final do escopo da função
}

/*
-------------------------------------------------------------------------------
2. EXECUÇÃO PRÁTICA
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("     ADICIONANDO MÉTODOS EM STRUCTS     ");
    println!("========================================");

    // 1. Chamando a Função Associada para criar o objeto
    // Usamos `::` porque não temos uma instância ainda
    let mut meu_sensor = SensorTemperatura::new(101, 40.0);

    // 2. Chamando o Método Mutável (`&mut self`)
    // Usamos `.` porque já temos a instância `meu_sensor`
    meu_sensor.adicionar_leitura(25.5);
    meu_sensor.adicionar_leitura(32.0);
    meu_sensor.adicionar_leitura(42.5); // Isso vai disparar o print de Alerta!

    // 3. Chamando o Método de Leitura (`&self`)
    if let Some(media) = meu_sensor.calcular_media() {
        println!("A média das temperaturas do sensor é: {:.2}°C", media);
    }

    println!("\n----------------------------------------");

    // 4. Chamando o Método de Consumo (`self`)
    // Passamos o controle total de `meu_sensor` para a função.
    let historico_final = meu_sensor.desativar_e_extrair_dados();

    println!("Histórico extraído possui {} registros.", historico_final.len());

    /*
       O PODER DA SEGURANÇA DO RUST EM AÇÃO:
       Se você tentar descomentar a linha abaixo, o compilador vai dar um ERRO!
       Como chamamos um método que recebe `self`, a variável `meu_sensor` foi 
       completamente desalocada da memória.
    */
    // meu_sensor.calcular_media(); // <- ERRO: value borrowed here after move
}

/*
-------------------------------------------------------------------------------
DICA DE ENGENHARIA (Múltiplos blocos `impl`):
-------------------------------------------------------------------------------
No Rust, você não está limitado a usar apenas um bloco `impl` por struct. Você 
pode abrir quantos blocos `impl SensorTemperatura` quiser ao longo do seu código, 
até mesmo em arquivos diferentes (desde que estejam no mesmo módulo).

Isso é excelente para organizar códigos extensos, permitindo separar, por exemplo, 
os métodos de inicialização dos métodos de processamento matemático ou de rede.
*/
