use std::time::Duration;
use std::thread::sleep;

pub fn limpar_tela(){
    clearscreen::clear().expect("Falha ao limpar a tela");
}

pub fn esperar(tempo: u64) {
    sleep(Duration::from_secs(tempo));
}
