// O Tipo Option<T> e o Tratamento de Ausência de Valor em Rust

/*
Em Rust, não existe o conceito de valor 'null' ou 'nil' como em outras linguagens (Java, C++, JavaScript).
Para representar a presença ou ausência de um valor de forma segura em tempo de compilação, 
Rust utiliza a enumeração padrão `Option<T>`:

enum Option<T> {
    Some(T), // Contém um valor do tipo T
    None,    // Indica a ausência de valor
}

Os variantes `Some` e `None` estão no escopo global (prelude), portanto não é necessário importar `Option::Some` ou `Option::None`.
*/

fn main() {
    println!("=== 1. Criando Valores com Option ===");
    let numero: Option<i32> = Some(42);
    let texto_ausente: Option<String> = None;

    println!("Numero: {:?}", numero);
    println!("Texto ausente: {:?}", texto_ausente);

    println!("\n=== 2. Manipulando Option com 'match' ===");
    let nome_usuario: Option<&str> = Some("Henrique");
    let usuario_anonimo: Option<&str> = None;

    saudacao(nome_usuario);
    saudacao(usuario_anonimo);

    println!("\n=== 3. Manipulando Option com 'if let' ===");
    // 'if let' é uma sintaxe mais concisa quando só nos importamos com o caso Some
    if let Some(nome) = nome_usuario {
        println!("Bem-vindo de volta, {}!", nome);
    } else {
        println!("Usuário não encontrado.");
    }

    println!("\n=== 4. Métodos Utilitários Comuns do Option ===");
    let fruta: Option<&str> = Some("Maçã");
    let sem_fruta: Option<&str> = None;

    // is_some() e is_none()
    println!("fruta.is_some(): {}", fruta.is_some());
    println!("sem_fruta.is_none(): {}", sem_fruta.is_none());

    // unwrap_or(): Retorna o valor contido em Some, ou um valor padrão caso seja None
    let valor_fruta = fruta.unwrap_or("Fruta Desconhecida");
    let valor_sem_fruta = sem_fruta.unwrap_or("Fruta Desconhecida");
    println!("Fruta selecionada: {}", valor_fruta);
    println!("Sem fruta fallback: {}", valor_sem_fruta);

    // unwrap_or_else(): Permite passar um closure/função para computar o valor padrão sob demanda
    let valor_calculado = sem_fruta.unwrap_or_else(|| "Banana de fallback");
    println!("Valor calculado: {}", valor_calculado);

    // unwrap() e expect()
    // ATENÇÃO: unwrap() causará panic se o valor for None! Use com cautela.
    // expect() também causa panic, mas permite especificar uma mensagem de erro customizada.
    let valor_seguro = fruta.expect("Esperava uma fruta, mas encontrou None");
    println!("Valor obtido com expect(): {}", valor_seguro);

    println!("\n=== 5. Combinadores e Transformações (map e and_then) ===");
    let numero_str: Option<&str> = Some("10");
    
    // map(): Aplica uma função ao valor dentro de Some, mantendo None se for None
    let tamanho: Option<usize> = numero_str.map(|s| s.len());
    println!("Tamanho da string em Some: {:?}", tamanho);

    // and_then(): Util para encadear operações que também retornam Option (evita Option<Option<T>>)
    let resultado_divisao = dividir(10.0, 2.0).and_then(|r| dividir(r, 2.0));
    println!("Resultado da divisão encadeada: {:?}", resultado_divisao); // Some(2.5)

    let divisao_por_zero = dividir(10.0, 0.0).and_then(|r| dividir(r, 2.0));
    println!("Resultado com divisão por zero: {:?}", divisao_por_zero); // None

    println!("\n=== 6. O Operador '?' em Funções que Retornam Option ===");
    match buscar_e_dobrar(&[10, 20, 30, 40], 2) {
        Some(val) => println!("Elemento no índice 2 dobrado: {}", val),
        None => println!("Elemento não foi encontrado."),
    }

    match buscar_e_dobrar(&[10, 20], 5) {
        Some(val) => println!("Elemento no índice 5 dobrado: {}", val),
        None => println!("Índice 5 fora dos limites da lista."),
    }
}

// Função auxiliar usando 'match' para extrair o valor de Option
fn saudacao(nome: Option<&str>) {
    match nome {
        Some(n) => println!("Olá, {}! Seja bem-vindo.", n),
        None => println!("Olá, visitante anônimo!"),
    }
}

// Função que pode falhar (ex: divisão por zero) retornando Option
fn dividir(numerador: f64, denominador: f64) -> Option<f64> {
    if denominador == 0.0 {
        None
    } else {
        Some(numerador / denominador)
    }
}

// Uso do operador '?' para propagar 'None' em funções que retornam Option
fn buscar_e_dobrar(colecao: &[i32], indice: usize) -> Option<i32> {
    // O operador '?' extrai o valor de Some ou retorna None imediatamente para a função chamadora
    let elemento = colecao.get(indice)?; 
    Some(elemento * 2)
}

/*
Resumo sobre Option<T>:
1. Rust substitui o 'null' pelo enum 'Option<T>' contendo 'Some(T)' e 'None'.
2. Garante tratamento explícito de valores ausentes em tempo de compilação.
3. Pode ser desestruturado com 'match' ou 'if let'.
4. Fornece métodos como 'unwrap_or', 'map', 'and_then' para manipulação fluida.
5. O operador '?' simplifica a propagação de 'None'.
*/
