// Este arquivo é o módulo `utilitarios`.

pub fn formatar_unidade(valor: f64, unidade: &str) -> String {
    format!("{:.2} {}", valor, unidade)
    }
