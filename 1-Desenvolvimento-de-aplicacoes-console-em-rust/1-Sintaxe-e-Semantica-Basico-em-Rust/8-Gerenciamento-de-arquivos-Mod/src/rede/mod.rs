// Este arquivo diz ao Rust o que existe dentro da pasta `rede/`.
// Ele funciona como o "porteiro" da pasta.

// 1. Declaramos que o arquivo `serial.rs` faz parte deste módulo
// 2. Tornamos o submódulo público (`pub`) para que o `main.rs` consiga vê-lo
pub mod serial;

// Também podemos ter funções diretamente no módulo pai
pub fn conectar_gateway() {
    println!("[REDE] Conectando ao gateway local...");
}
