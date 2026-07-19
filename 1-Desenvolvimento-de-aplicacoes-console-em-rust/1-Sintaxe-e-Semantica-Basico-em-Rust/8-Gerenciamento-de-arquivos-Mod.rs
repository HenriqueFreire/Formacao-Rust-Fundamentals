/*
===============================================================================
TEMA: Gerenciamento de Arquivos e Módulos (`mod`)
===============================================================================

No Rust, você não usa "includes" mágicos que saem puxando arquivos. 
O arquivo `main.rs` é a raiz de tudo. É ELE quem deve declarar ao compilador 
quais arquivos existem no projeto usando a palavra-chave `mod`.

Pense no `mod` como um comando que diz: "Compilador, vá buscar o arquivo/pasta 
com este nome e monte uma árvore de módulos".

-------------------------------------------------------------------------------
Declaração de Módulos (Obrigatório na raiz do projeto):
-------------------------------------------------------------------------------
*/

// Diz ao Rust para procurar por `src/hardware.rs`
mod hardware; 

// Diz ao Rust para procurar por uma pasta `src/rede/` contendo um `mod.rs`
mod rede;     

/*
-------------------------------------------------------------------------------
Execução e Caminhos de Acesso (Path)
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("      GERENCIAMENTO DE MÓDULOS          ");
    println!("========================================");

    // --- CAMINHO ABSOLUTO / RELATIVO ---
    
    // Acessando a função do módulo hardware através do caminho (path)
    let leitura = hardware::ler_sensor_analogico();
    println!("Dado recebido no main: {}", leitura);

    // hardware::calibrar_voltagem(); 
    // ^ ERRO DE COMPILAÇÃO! A função é privada dentro do módulo hardware.

    println!("\n----------------------------------------");

    // Acessando o módulo pai de rede
    rede::conectar_gateway();

    // Acessando o submódulo aninhado (rede -> serial)
    rede::serial::enviar_byte(0xAA);

    println!("\n----------------------------------------");

    // --- A PALAVRA CHAVE `use` (Criando Atalhos) ---
    // Se você não quiser ficar digitando `rede::serial::...` o tempo todo, 
    // pode trazer o caminho para o escopo local com o `use`:
    use rede::serial::enviar_byte;

    // Agora a função pode ser chamada diretamente!
    enviar_byte(0x55);
}
