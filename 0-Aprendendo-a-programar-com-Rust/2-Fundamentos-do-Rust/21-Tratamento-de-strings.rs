// Tratamento de Strings em Rust
//
// O Rust oferece uma vasta gama de métodos para manipular e tratar strings. 
// Como strings em Rust são UTF-8 por padrão, o tratamento exige atenção a 
// detalhes como caracteres multioctetos.

fn main() {
    let mut s = String::from("  Olá, Rust!  ");

    // 1. Limpeza (Trimming)
    // Remove espaços em branco no início e no fim.
    let trimado = s.trim();
    println!("Trim: '{}'", trimado);

    // 2. Alteração de Caixa (Casing)
    println!("Maiúsculo: {}", trimado.to_uppercase());
    println!("Minúsculo: {}", trimado.to_lowercase());

    // 3. Substituição (Replace)
    let nova_string = trimado.replace("Rust", "Mundo");
    println!("Replace: {}", nova_string);

    // 4. Busca e Verificação
    println!("Contém 'Olá'? {}", trimado.contains("Olá"));
    println!("Começa com 'O'? {}", trimado.starts_with('O'));
    println!("Termina com '!'? {}", trimado.ends_with('!'));

    // 5. Concatenação
    // A. Usando o operador '+' (Nota: consome a primeira String)
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 foi movida aqui e não pode mais ser usada
    println!("Concatenação (+): {}", s3);

    // B. Usando format! (Mais flexível e não consome as variáveis)
    let nome = "Henrique";
    let saudacao = format!("Olá, {}! Bem-vindo ao curso.", nome);
    println!("Concatenação (format!): {}", saudacao);

    // 6. Iteração
    let palavra = "Café";
    
    println!("Iterando por caracteres (chars):");
    for c in palavra.chars() {
        print!("{} ", c);
    }
    println!();

    println!("Iterando por bytes:");
    for b in palavra.bytes() {
        print!("{} ", b);
    }
    println!();

    // 7. Divisão (Split)
    let lista = "maçã,banana,laranja";
    println!("Lista dividida:");
    for item in lista.split(',') {
        println!(" - {}", item);
    }
}

/*
DICAS IMPORTANTES:
1. Strings são UTF-8: Um caractere pode ter de 1 a 4 bytes. 
   Por isso, o Rust não permite indexação direta (ex: s[0]) para evitar erros.
2. Métodos que retornam Slices: Muitos métodos como .trim() retornam um &str (slice), 
   que é uma visão da string original sem copiá-la.
3. Alocação: Métodos como .to_uppercase() ou .replace() criam uma NOVA String na Heap.
*/
