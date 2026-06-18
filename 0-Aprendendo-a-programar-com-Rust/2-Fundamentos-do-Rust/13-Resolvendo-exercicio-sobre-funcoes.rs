// Resolvendo Exercícios sobre Funções em Rust

/*
   Neste arquivo, vamos aplicar o conhecimento de funções para resolver
   problemas práticos comuns.
*/

fn main() {
    println!("--- Resolução de Exercícios ---");

    // Exercício 1: Área de um Retângulo
    let largura = 10.0;
    let altura = 5.0;
    let area = calcular_area_retangulo(largura, altura);
    println!("1. A área do retângulo ({}x{}) é: {}", largura, altura, area);

    // Exercício 2: Verificador de Paridade
    let numero = 7;
    if eh_par(numero) {
        println!("2. O número {} é par.", numero);
    } else {
        println!("2. O número {} é ímpar.", numero);
    }

    // Exercício 3: Conversor de Temperatura (Celsius para Fahrenheit)
    let celsius = 25.0;
    let fahrenheit = celsius_para_fahrenheit(celsius);
    println!("3. {}°C equivalem a {}°F", celsius, fahrenheit);

    // Exercício 4: Calcular Média
    let n1 = 8.5;
    let n2 = 7.0;
    let n3 = 9.2;
    let media = calcular_media(n1, n2, n3);
    println!("4. A média das notas {}, {} e {} é: {:.2}", n1, n2, n3, media);
}

/* 
   EXPLICANDO AS SOLUÇÕES:
*/

/// 1. Calcula a área multiplicando base por altura.
/// Retorna um f64 (ponto flutuante).
fn calcular_area_retangulo(base: f64, altura: f64) -> f64 {
    base * altura // Retorno implícito (sem ';' e sem 'return')
}

/// 2. Usa o operador de resto (%) para verificar se o número é divisível por 2.
/// Retorna um bool (true/false).
fn eh_par(n: i32) -> bool {
    n % 2 == 0
}

/// 3. Aplica a fórmula: (Celsius * 1.8) + 32.
fn celsius_para_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

/// 4. Soma três valores e divide por 3.
/// O formato '{:.2}' no println acima limita as casas decimais.
fn calcular_media(a: f64, b: f64, c: f64) -> f64 {
    (a + b + c) / 3.0
}

/*
   RESUMO DO APRENDIZADO:
   - Funções ajudam a organizar o código em blocos lógicos reutilizáveis.
   - O uso de nomes descritivos (ex: `eh_par`) torna o código "auto-explicativo".
   - Tipos de entrada e saída garantem que os cálculos sejam seguros.
*/
