/*
===============================================================================
TEMA: Inspecionando Estruturas (Dados, Impressão e Metadados de Memória)
===============================================================================

Para inspecionar o CONTEÚDO de uma struct, usamos o atributo `#[derive(Debug)]`.
Para inspecionar a ESTRUTURA FÍSICA (memória), usamos funções do módulo `std::mem`.

-------------------------------------------------------------------------------
1. PREPARANDO A STRUCT PARA INSPEÇÃO
-------------------------------------------------------------------------------
*/

// O atributo `Debug` permite a inspeção textual dos dados.
// O `Default` nos permite criar uma instância com valores padrão facilmente.
#[derive(Debug, Default)]
struct FrameRede {
    id: u8,            // 1 byte
    pago: bool,        // 1 byte
    timestamp: u32,    // 4 bytes
    pino_destino: u16, // 2 bytes
}

/*
-------------------------------------------------------------------------------
2. EXECUÇÃO E MÉTODOS DE INSPEÇÃO
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("       INSPECIONANDO ESTRUTURAS         ");
    println!("========================================");

    // Criando uma estrutura preenchida com valores padrão (zeros/false)
    let mut meu_frame = FrameRede::default();
    meu_frame.id = 42;
    meu_frame.timestamp = 1640995200;

    // -------------------------------------------------------------------------
    // MÉTODO 1: Inspeção de Dados via Print (Standard e Pretty)
    // -------------------------------------------------------------------------
    
    // {:?} Imprime tudo em uma única linha
    println!("Inspeção Simples: {:?}", meu_frame);
    
    // {:#?} Imprime de forma indentada (Pretty Print), ideal para structs grandes
    println!("\nInspeção Detalhada (Pretty Print):\n{:#?}", meu_frame);

    // -------------------------------------------------------------------------
    // MÉTODO 2: A Macro `dbg!` (A joia do Rust para Debug rápido)
    // -------------------------------------------------------------------------
    println!("\n--- Usando a Macro dbg! ---");
    // Diferente do println!, a macro `dbg!` imprime o arquivo, a linha exata 
    // onde ela foi chamada, o nome da variável e devolve a posse (ownership) do dado.
    let _copia_frame = dbg!(&meu_frame); 

    // -------------------------------------------------------------------------
    // MÉTODO 3: Inspeção de Tamanho e Alinhamento de Memória (Foco em Engenharia)
    // -------------------------------------------------------------------------
    println!("\n--- Inspeção de Arquitetura de Memória ---");
    
    // Tamanho total que a struct ocupa na memória em Bytes
    let tamanho = std::mem::size_of::<FrameRede>();
    println!("Tamanho total da struct: {} bytes", tamanho);

    // Alinhamento exigido pela CPU (múltiplo de quantos bytes ela precisa para ler o dado)
    let alinhamento = std::mem::align_of::<FrameRede>();
    println!("Alinhamento exigido na CPU: {} bytes", alinhamento);

    /*
       POR QUE O TAMANHO DEU 8 OU 12 BYTES E NÃO 8 EXATOS? (1 + 1 + 4 + 2 = 8?)
       
       Por padrão, o Rust rearranja a ordem dos campos para evitar desperdício de memória 
       com preenchimentos vazios (padding). Ele junta o u8 (1) e o bool (1) e tenta alinhar 
       com as fronteiras de leitura de 32-bits (4 bytes) da sua CPU.
    */

    // -------------------------------------------------------------------------
    // MÉTODO 4: Inspecionando Ponteiros e Endereços Físicos
    // -------------------------------------------------------------------------
    println!("\n--- Inspeção de Endereços de Memória ---");
    
    // Obtendo o endereço de memória base da nossa struct
    let endereco_base = &meu_frame as *const FrameRede;
    println!("Endereço base da struct na Stack: {:?}", endereco_base);

    // Inspecionando o endereço de um campo específico para ver o deslocamento (offset)
    let endereco_id = &meu_frame.id as *const u8;
    println!("Endereço do campo 'id':          {:?}", endereco_id);
    
    let endereco_time = &meu_frame.timestamp as *const u32;
    println!("Endereço do campo 'timestamp':   {:?}", endereco_time);
}

/*
-------------------------------------------------------------------------------
DICA DE OURO PARA PROVAS E PROJETOS DA FACULDADE:
-------------------------------------------------------------------------------
Quando você estiver escrevendo um código e ele apresentar um comportamento estranho, 
não perca tempo digitando `println!("Cheguei aqui na linha X")`. 

Basta colocar `dbg!(sua_variavel);` ou até mesmo dentro de uma condição:
`if dbg!(meu_frame.id == 42) { ... }`

Como a macro `dbg!` avalia a expressão e a retorna, você pode injetá-la no meio 
de equações matemáticas ou verificações lógicas sem quebrar o fluxo do seu programa, 
recebendo um relatório completo do estado daquela estrutura no seu terminal.
*/
