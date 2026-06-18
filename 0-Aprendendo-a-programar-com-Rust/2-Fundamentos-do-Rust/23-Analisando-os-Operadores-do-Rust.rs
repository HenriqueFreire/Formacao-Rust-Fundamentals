// Analisando os Operadores do Rust
//
// O Rust possui diversos operadores para realizar operações aritméticas, 
// de comparação, lógicas e bitwise.

fn main() {
    // 1. OPERADORES ARITMÉTICOS
    // Adição, subtração, multiplicação, divisão e resto.
    let soma = 10 + 5;
    let subtracao = 20 - 7;
    let multiplicacao = 4 * 8;
    let divisao = 20 / 3; // Divisão inteira resulta em 6
    let resto = 20 % 3;   // Resto da divisão (módulo) resulta em 2

    println!("Aritméticos: {} {} {} {} {}", soma, subtracao, multiplicacao, divisao, resto);

    // 2. OPERADORES DE COMPARAÇÃO
    // Retornam um booleano (true ou false).
    let igual = 5 == 5;
    let diferente = 5 != 10;
    let maior = 10 > 5;
    let menor = 5 < 10;
    let maior_igual = 5 >= 5;
    let menor_igual = 10 <= 20;

    println!("Comparação: {} {} {} {} {} {}", igual, diferente, maior, menor, maior_igual, menor_igual);

    // 3. OPERADORES LÓGICOS
    // AND (&&), OR (||) e NOT (!).
    let verdadeiro = true;
    let falso = false;

    let e_logico = verdadeiro && falso; // false
    let ou_logico = verdadeiro || falso; // true
    let nao_logico = !verdadeiro;        // false

    println!("Lógicos: {} {} {}", e_logico, ou_logico, nao_logico);

    // 4. OPERADORES BITWISE (Bit a Bit)
    // AND (&), OR (|), XOR (^), SHIFT LEFT (<<), SHIFT RIGHT (>>).
    let a: u8 = 0b0000_1010; // 10
    let b: u8 = 0b0000_1100; // 12

    let bit_and = a & b; // 0b0000_1000 (8)
    let bit_or = a | b;  // 0b0000_1110 (14)
    let bit_xor = a ^ b; // 0b0000_0110 (6)
    let shift_l = a << 1; // 0b0001_0100 (20)
    let shift_r = a >> 1; // 0b0000_0101 (5)

    println!("Bitwise: {} {} {} {} {}", bit_and, bit_or, bit_xor, shift_l, shift_r);

    // 5. OPERADORES DE ATRIBUIÇÃO COMPOSTA
    let mut x = 10;
    x += 5; // x = x + 5
    x -= 2; // x = x - 2
    x *= 3; // x = x * 3
    x /= 2; // x = x / 2
    x %= 4; // x = x % 4

    println!("Atribuição Composta (x final): {}", x);
}

/*
NOTAS SOBRE OPERADORES EM RUST:
1. Verificação de Overflow: No modo debug, o Rust verifica se operações aritméticas 
   causam overflow (ex: 255u8 + 1). No modo release, ocorre o "wrapping".
2. Tipagem Forte: Você não pode realizar operações entre tipos diferentes 
   (ex: i32 + f64) sem fazer o casting explícito com 'as'.
3. Curto-Circuito: Os operadores lógicos && e || possuem curto-circuito. Se o 
   resultado puder ser determinado pelo primeiro operando, o segundo não é avaliado.
4. Sobrecarga de Operadores: O Rust permite sobrecarregar operadores implementando 
   traits específicos (como std::ops::Add para o operador '+').
*/
