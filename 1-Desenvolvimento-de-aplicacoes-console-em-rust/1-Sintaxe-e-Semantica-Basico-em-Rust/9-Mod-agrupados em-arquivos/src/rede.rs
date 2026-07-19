// Este arquivo tem o mesmo nome da pasta 'rede/'. 
// O compilador do Rust entende automaticamente que tudo o que for declarado 
// aqui dentro como `mod` deve ser procurado dentro da pasta 'rede/'.

// 1. Declaramos e exportamos os submódulos que estão na pasta vizinha
pub mod serial;
pub mod ethernet;

// 2. Podemos criar funções gerais para o módulo de rede aqui também
pub fn inicializar_interface_rede() {
    println!("[REDE] Ligando periféricos de comunicação...");
}
