// Este arquivo é o módulo `matematica`.

pub fn calcular_frequencia_resonancia(indutancia: f64, capacitancia: f64) -> f64 {
    // Fórmula de ressonância: $f = \frac{1}{2\pi\sqrt{LC}}$
    let pi = std::f64::consts::PI;
    1.0 / (2.0 * pi * (indutancia * capacitancia).sqrt())
}
