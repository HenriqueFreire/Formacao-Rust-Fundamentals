// Exercício: Trabalhando com Loops em Rust (loop, while, for)

fn main() {
    /*
        1. O loop Infinito (`loop`)
        O comando `loop` diz ao Rust para executar um bloco de código repetidamente
        até que você diga explicitamente para parar com `break`.
    */

    let mut contador = 0;

    println!("Iniciando o 'loop':");
    loop {
        contador += 1;
        println!("Contador: {}", contador);

        if contador == 3 {
            println!("Chegamos a 3, parando o loop.");
            break; // Sai do loop
        }
    }

    /*
        2. Loop Condicional (`while`)
        Executa o código enquanto uma condição for verdadeira.
    */

    let mut numero = 3;

    println!("\nIniciando o 'while':");
    while numero != 0 {
        println!("{}!", numero);
        numero -= 1;
    }
    println!("Lançar!");

    /*
        3. Iterando com `for`
        O `for` é o loop mais seguro e comum em Rust, usado para percorrer coleções ou intervalos.
    */

    println!("\nIniciando o 'for' com intervalo (range):");
    // (1..4) cria um intervalo de 1 a 3 (o 4 é exclusivo)
    for i in 1..4 {
        println!("Valor: {}", i);
    }

    println!("\nPercorrendo um array com 'for':");
    let a = [10, 20, 30, 40, 50];

    for elemento in a {
        println!("O valor do elemento é: {}", elemento);
    }

    /*
        4. Labels de Loop (Rótulos)
        Se você tiver loops aninhados, pode usar rótulos para especificar
        qual loop o `break` ou `continue` deve afetar.
    */

    println!("\nExemplo de Loop Label:");
    let mut cont = 0;
    'externo: loop {
        println!("Contagem externa: {}", cont);
        let mut restante = 10;

        loop {
            println!("Restante interno: {}", restante);
            if restante == 9 {
                break; // Sai do loop interno
            }
            if cont == 2 {
                break 'externo; // Sai do loop externo
            }
            restante -= 1;
        }

        cont += 1;
    }

    /*
        EXERCÍCIO PRÁTICO:
        Use um loop para imprimir apenas os números PARES de 1 a 10.
    */

    println!("\nNúmeros pares de 1 a 10:");
    for n in 1..=10 { // 1..=10 inclui o 10
        if n % 2 != 0 {
            continue; // Pula os ímpares
        }
        println!("{}", n);
    }
}
