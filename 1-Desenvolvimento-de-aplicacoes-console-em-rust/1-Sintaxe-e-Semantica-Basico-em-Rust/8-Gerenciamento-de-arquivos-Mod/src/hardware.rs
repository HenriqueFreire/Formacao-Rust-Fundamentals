// Este arquivo representa o módulo `hardware`.
// Por padrão, tudo no Rust é privado. Para permitir que outros arquivos 
// usem esta função, precisamos adicionar a palavra-chave `pub` (público).

pub fn ler_sensor_analogico() -> i32 {
    println!("[HARDWARE] Lendo dados do conversor ADC...");
    2048 // Simulação de uma leitura de 12 bits
}

// Esta função não tem `pub`, então ela é PRIVADA. 
// Apenas outras funções DENTRO deste arquivo `hardware.rs` podem chamá-la.
fn calibrar_voltagem() {
    println!("Calibrando referências internas...");
}
