/*
===============================================================================
TEMA: Estruturas de Dados (Structs) e Comportamentos (impl)
===============================================================================

No Rust, existem 3 tipos principais de structs:
1. Classic Structs (Estruturas Clássicas): Com campos nomeados (tipo um JSON/Objeto).
2. Tuple Structs (Estruturas de Tupla): Sem campos nomeados, apenas tipos posicionais.
3. Unit-Like Structs (Estruturas Unitárias): Sem campos, ótimas para criar comportamentos ou estados.

Diferente do C ou C++, onde você pode acabar acessando lixo de memória se não 
inicializar uma struct, o Rust obriga a inicialização de TODOS os campos no momento da criação.

-------------------------------------------------------------------------------
1. DEFINIÇÃO DAS STRUCTS
-------------------------------------------------------------------------------
*/

// Para podermos imprimir nossa struct com `println!("{:?}", x)`, 
// precisamos adicionar este atributo que "ensina" o Rust a exibir a estrutura.
#[derive(Debug)]
struct ComponenteEletronico {
    nome: String,
    tensao_operacao: f32, // Em Volts (comum em Engenharia)
    corrente_maxima: f32, // Em Amperes
    em_estoque: bool,
}

// 2. Tuple Struct (Útil para criar tipos distintos para coisas parecidas)
#[derive(Debug)]
struct Ponto3D(f64, f64, f64); // Ex: Coordenadas em um espaço cartesiano

// 3. Unit-Like Struct (Útil para implementar comportamentos compartilhados - Traits)
#[derive(Debug)]
struct ProtocoloComunicacao;

/*
-------------------------------------------------------------------------------
2. IMPLEMENTANDO COMPORTAMENTO (O Bloco `impl`)
-------------------------------------------------------------------------------
No Rust, os dados da struct ficam separados das suas funções. Para criar métodos 
(funções que operam sobre os dados da struct), usamos o bloco `impl`.
*/

impl ComponenteEletronico {
    // Construtor (Por convenção, uma função associada chamada `new`)
    // Repare que não recebe `self`, pois ela serve para CRIAR a struct.
    fn new(nome: &str, tensao: f32, corrente: f32) -> Self {
        Self {
            nome: nome.to_string(),
            tensao_operacao: tensao,
            corrente_maxima: corrente,
            em_estoque: true,
        }
    }

    // Método de Leitura (&self): Apenas lê os dados sem alterá-los
    fn calcular_potencia_maxima(&self) -> f32 {
        // Potência = Tensão * Corrente ($P = V \times I$)
        self.tensao_operacao * self.corrente_maxima
    }

    // Método de Modificação (&mut self): Permite alterar os dados da struct
    fn alterar_estoque(&mut self, status: bool) {
        self.em_estoque = status;
    }
}

/*
-------------------------------------------------------------------------------
3. EXECUÇÃO PRÁTICA
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("        TRABALHANDO COM STRUCTS         ");
    println!("========================================");

    // --- INSTANCIANDO UMA CLASSIC STRUCT ---
    // Usando o construtor customizado que criamos
    let mut microcontrolador = ComponenteEletronico::new("ESP32-S3", 3.3, 0.5);

    // Acessando campos individuais
    println!("Componente: {}", microcontrolador.nome);
    
    // Chamando um método de leitura
    let potencia = microcontrolador.calcular_potencia_maxima();
    println!("Potência máxima do {}: {} Watts", microcontrolador.nome, potencia);

    // Chamando um método mutável (exige que a variável seja `let mut`)
    microcontrolador.alterar_estoque(false);
    
    // Imprimindo a struct inteira usando a flag de debug `{:?}`
    println!("Estado atual do componente:\n{:#?}", microcontrolador);

    println!("\n----------------------------------------");

    // --- INSTANCIANDO UMA TUPLE STRUCT ---
    let origem = Ponto3D(0.0, 0.0, 0.0);
    // Acessamos os elementos usando índices (como em uma tupla normal)
    println!("Coordenada X da origem: {}", origem.0);
    println!("Ponto completo: {:?}", origem);

    // --- STRUCT UPDATE SYNTAX (Sintaxe de atualização) ---
    // Se você quiser criar uma nova struct aproveitando a maior parte dos dados de outra:
    let sensor_temperatura = ComponenteEletronico {
        nome: String::from("DHT22"),
        ..microcontrolador // Copia os valores de tensao, corrente e estoque do microcontrolador
    };
    
    println!("\nNovo sensor criado a partir do anterior:\n{:#?}", sensor_temperatura);
}

/*
-------------------------------------------------------------------------------
VISÃO DE ENGENHARIA DA COMPUTAÇÃO (Gerenciamento de Memória):
-------------------------------------------------------------------------------
Nas linguagens baseadas em objetos (como Java ou C#), criar um objeto significa 
alocar memória na Heap e gerar um ponteiro invisível para ele. 

No Rust (assim como no C++), as structs são alocadas contiguamente por padrão onde 
você as criou. Se você declará-la dentro da `fn main`, todos os seus dados ficam 
diretamente na Stack (com exceção do conteúdo da String `nome`, que aponta para a Heap). 
Isso traz um ganho de performance gigantesco, crucial para sistemas embarcados de 
baixa latência, pois evita alocações dinâmicas desnecessárias.
*/
