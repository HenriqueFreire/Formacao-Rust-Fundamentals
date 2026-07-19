// Este é um submódulo (rede::serial)

pub fn enviar_byte(dados: u8) {
    println!("[SERIAL] Enviando byte 0x{:X} via pinos TX/RX...", dados);
}
