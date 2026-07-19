/*
===============================================================================
TEMA: Aprofundando em Structs (Visibilidade, Ciclo de Vida e Layout de Memória)
===============================================================================

No Rust, por padrão, TODOS os campos de uma struct são **privados**, mesmo que a 
struct em si seja pública (`pub struct`). Eles só podem ser acessados de dentro 
do mesmo módulo onde a struct foi definida. Isso garante o encapsulamento perfeito.

-------------------------------------------------------------------------------
1. ENCAPSULAMENTO E VISIBILIDADE
-------------------------------------------------------------------------------
*/

// Vamos encapsular um registrador de hardware simulado.
pub struct Registrador {
    pub nome: String,        // Campo público: qualquer um pode ler/escrever diretamente.
    endereco: u32,           // Privado: apenas métodos internos controlam o acesso.
    valor: u8,               // Privado: evita que o usuário mude o estado sem validação.
}

impl Registrador {
    // Construtor público obrigatório, já que há campos privados que o mundo 
    // externo não consegue preencher diretamente.
    pub fn new(nome: &str, endereco: u32) -> Self {
        Self {
            nome: nome.to_string(),
            endereco,
            valor: 0, // Inicializa zerado por segurança
        }
    }

    // Getter público para ler o valor privado
    pub fn ler_valor(&self) -> u8 {
        self.valor
    }

    // Setter com validação (Garante que só aceitamos valores pares, por exemplo)
    pub fn escrever_valor(&mut self, novo_valor: u8) -> Result<(), &'static str> {
        if novo_valor % 2 != 0 {
            return Err("Este registrador simulado só aceita valores pares!");
        }
        self.valor = novo_valor;
        Ok(())
    }
}

/*
-------------------------------------------------------------------------------
2. STRUCTS COM REFERÊNCIAS E LIFETIMES (Ciclos de Vida)
-------------------------------------------------------------------------------
Até agora, nossas structs "possuíam" seus dados (usando `String`, `u32`, etc.). 
Mas e se uma struct precisar apenas "pegar emprestado" (&) um dado que pertence 
a outra parte do programa? 

O Rust exige que você especifique um **Lifetime** (`'a`) para garantir que a 
struct não viva mais tempo do que o dado que ela está apontando, evitando ponteiros soltos.
*/

// Lemos isto como: "A struct Logger possui um ciclo de vida 'a, e a referência 
// do prefixo que ela carrega também deve durar pelo menos o tempo 'a".
struct Logger<'a> {
    prefixo: &'a str, // Não é uma String alocada na Heap, é apenas um ponteiro para um texto existente.
}

impl<'a> Logger<'a> {
    fn log(&self, mensagem: &str) {
        println!("{}: {}", self.prefixo, mensagem);
    }
}

/*
-------------------------------------------------------------------------------
3. LAYOUT DE MEMÓRIA NO HARDWARE (Foco em Engenharia da Computação)
-------------------------------------------------------------------------------
Por padrão, o Rust pode reordenar a posição dos campos de uma struct na memória 
para diminuir o espaço gasto com alinhamento (*padding*). 

Se você estiver mapeando uma struct diretamente para os registradores físicos de 
um microcontrolador ou se comunicando com um código em C, você precisa desativar 
essa otimização e forçar o Rust a seguir a ordem exata que você escreveu.
*/

#[repr(C)] // Força o layout de memória padrão da linguagem C (Previsível)
struct PacoteRede {
    id_protocolo: u8,  // 1 byte
    // O Rust normal colocaria o `dado` antes para otimizar espaço, 
    // mas o `repr(C)` garante que o hardware verá o id_protocolo primeiro.
    checksum: u16,     // 2 bytes
    dado: u32,         // 4 bytes
}

/*
-------------------------------------------------------------------------------
4. EXECUÇÃO PRÁTICA
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("     APROFUNDANDO EM STRUCTS NO RUST    ");
    println!("========================================");

    // 1. Testando Encapsulamento
    let mut reg_adc = Registrador::new("ADC_CTRL", 0x4002_1000);
    reg_adc.nome = String::from("ADC_CONTROLADOR"); // Permitido (Público)
    
    // reg_adc.valor = 5; // ERRO DE COMPILAÇÃO! O campo é privado.
    
    match reg_adc.escrever_valor(12) {
        Ok(_) => println!("Valor escrito com sucesso!"),
        Err(e) => println!("Erro: {}", e),
    }
    println!("Valor atual do registrador: {}", reg_adc.ler_valor());

    println!("\n----------------------------------------");

    // 2. Testando Lifetimes (Empréstimo de dados)
    let texto_base = String::from("[SISTEMA]");
    
    let meu_logger = Logger {
        prefixo: &texto_base, // Pegando emprestado de texto_base
    };
    
    meu_logger.log("Inicializando drivers...");
    // Se `texto_base` saísse de escopo aqui, o Rust proibiria o uso de `meu_logger`.

    println!("\n----------------------------------------");
    
    // 3. Testando Tamanho em Memória
    println!("Tamanho da struct PacoteRede em bytes: {}", std::mem::size_of::<PacoteRede>());
    // Nota de Engenharia: Devido ao alinhamento de memória (padding) exigido pela CPU, 
    // o tamanho pode ser maior que a soma estrita dos tipos (1 + 2 + 4 = 7). 
    // O compilador alinha para múltiplos de 2 ou 4 bytes para o processador ler mais rápido.
    }
