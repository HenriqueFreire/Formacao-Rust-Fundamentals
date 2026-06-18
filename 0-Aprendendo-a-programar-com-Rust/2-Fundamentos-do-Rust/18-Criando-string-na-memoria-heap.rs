// Criando String na memória Heap em Rust
//
// No Rust, existem dois tipos principais de strings:
// 1. &str (String Slice): Geralmente aponta para strings literais ou partes de outras strings.
//    O tamanho é fixo e conhecido em tempo de compilação (geralmente na Stack ou seção estática).
//
// 2. String (Tipo String): É uma string crescível, mutável e de propriedade (owned).
//    Os dados reais da string são armazenados na memória HEAP.
//    A Stack contém apenas o ponteiro para a Heap, o tamanho (length) e a capacidade.

fn main() {
    // 1. Criando uma String vazia na Heap
    let mut s = String::new();
    s.push_str("Olá"); // Agora a string cresce na Heap
    s.push('!');
    println!("String vazia que cresceu: {}", s);

    // 2. Criando uma String a partir de um literal (&str)
    // String::from() aloca memória na Heap e copia o conteúdo literal
    let saudacao = String::from("Bem-vindo ao curso de Rust");
    println!("String::from: {}", saudacao);

    // 3. Usando o método .to_string()
    // Comum para converter outros tipos (ou literais) em Strings na Heap
    let nome = "Henrique".to_string();
    println!("Usando .to_string(): {}", nome);

    // 4. Exemplo de Mutabilidade e Crescimento
    let mut mensagem = String::from("Aprendendo ");
    mensagem.push_str("Fundamentos ");
    mensagem.push_str("de Rust!");
    
    println!("Mensagem final: {}", mensagem);
    println!("Tamanho: {} bytes", mensagem.len());
    println!("Capacidade: {} bytes", mensagem.capacity());
}

/*
Por que usar a Heap para Strings?
1. Tamanho Dinâmico: Se você não sabe o tamanho final da string (ex: entrada do usuário), 
   precisa da Heap para redimensionar o espaço conforme necessário.
2. Propriedade (Ownership): Ao criar uma String na Heap, você é o dono dela. 
   Ela será limpa automaticamente quando sair de escopo (Drop).
3. Transferência de Dados: É fácil mover a propriedade de uma String entre funções, 
   já que apenas os dados na Stack (ponteiro, len, cap) são copiados.
*/
