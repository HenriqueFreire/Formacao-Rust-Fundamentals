// Este arquivo está completamente fora da pasta src/ e não sabe nada 
// sobre a árvore padrão do compilador.

pub fn validar_checksum(dados: &[u8], checksum_esperado: u8) -> bool {
    println!("[VALIDADOR] Calculando XOR Checksum dos dados...");
    
    // Um algoritmo simples de XOR para validação de integridade de bytes
    let mut checksum_calculado = 0;
    for &byte in dados {
        checksum_calculado ^= byte;
    }
    
    checksum_calculado == checksum_esperado
}
