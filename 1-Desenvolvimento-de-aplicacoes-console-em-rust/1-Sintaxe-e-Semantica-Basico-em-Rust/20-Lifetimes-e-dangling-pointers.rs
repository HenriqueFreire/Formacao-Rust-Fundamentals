// ============================================================
//  LIFETIMES E DANGLING POINTERS EM RUST
// ============================================================
//
//  Em linguagens como C/C++, "dangling pointers" (ponteiros
//  pendurados) são referências a memória que já foi liberada.
//  Rust elimina esse problema em tempo de COMPILAÇÃO usando
//  o sistema de Lifetimes (tempos de vida).
//
//  Um lifetime ('a) é uma anotação que diz ao compilador
//  "esta referência é válida enquanto 'a estiver ativo".
//  Não tem custo em tempo de execução — é puro tempo de
//  compilação.
//
//  Tópicos:
//  1.  O problema: dangling pointer (o que Rust previne)
//  2.  Borrow checker — como Rust detecta o problema
//  3.  Lifetime básico em funções
//  4.  Lifetime em structs
//  5.  Lifetime em métodos (impl)
//  6.  Lifetime 'static
//  7.  Múltiplos lifetimes
//  8.  Lifetime elision (quando omitir a anotação)
//  9.  Lifetime + Generics + Traits juntos
//  10. Casos reais: slice, split, parser
//
//  Execute com:  rustc lifetimes.rs && ./lifetimes
// ============================================================

// ============================================================
// 1. O PROBLEMA: DANGLING POINTER
//    O código abaixo NÃO compila — e é exatamente isso que
//    queremos! O comentário mostra o que aconteceria em C.
// ============================================================

/*  ← descomente para ver o erro do compilador

fn dangling() -> &String {          // erro: falta lifetime
    let s = String::from("perigo"); // s é alocada aqui
    &s                              // retornamos referência...
}                                   // ...mas s é destruída aqui!
                                    // Em C isso seria UB silencioso.

fn main() {
    let r = dangling();
    println!("{}", r); // leitura de memória inválida!
}

*/

// Rust recusa a compilar. A solução é retornar o valor,
// não uma referência para um valor local:

fn sem_dangling() -> String {
    let s = String::from("seguro");
    s   // move a propriedade — nenhuma referência pendente
}

// ============================================================
// 2. BORROW CHECKER — O GUARDIÃO DOS LIFETIMES
//    Rust analisa o escopo de cada variável e garante que
//    nenhuma referência sobrevive mais que o dado original.
// ============================================================

fn exemplo_borrow_checker() {
    // --- Caso inválido (comentado para compilar) ---
    /*
    let referencia;
    {
        let valor = 5;
        referencia = &valor; // valor vive só dentro do bloco
    }
    // valor foi destruído; referencia seria dangling:
    println!("{}", referencia); // ERRO de compilação
    */

    // --- Caso válido: referência não sobrevive ao dado ---
    let valor = 5;           // 'valor' começa aqui
    let referencia = &valor; // referencia "empresta" valor
    println!("  ref = {}", referencia); // ok: valor ainda vivo
}   // referencia e valor terminam juntos — sem problema

// ============================================================
// 3. LIFETIME BÁSICO EM FUNÇÕES
//    Quando retornamos uma referência que vem de parâmetros,
//    o compilador precisa saber de qual deles ela vem.
// ============================================================

// 'a é um parâmetro de lifetime genérico.
// Lê-se: "o retorno vive pelo menos enquanto 'a viver,
//         e 'a é a interseção dos lifetimes de x e y."
fn maior_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// Sem anotação isso não compilaria: o compilador não saberia
// se o retorno vem de x ou de y — e portanto qual lifetime usar.

fn exemplo_maior_str() {
    let s1 = String::from("Programação");

    let resultado;
    {
        let s2 = String::from("Rust");
        resultado = maior_str(&s1, &s2);
        // resultado só pode ser usado aqui dentro, pois s2
        // (o menor escopo) termina no fechamento do bloco.
        println!("  Maior: '{}'", resultado);
    }
    // println!("{}", resultado); // ERRO: s2 não existe mais
}

// Função que retorna sempre x — o lifetime de y é irrelevante.
// Podemos anotar só o que importa para o retorno:
fn primeiro<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// ============================================================
// 4. LIFETIME EM STRUCTS
//    Se uma struct guarda uma referência, ela precisa declarar
//    que não vai sobreviver mais que o dado referenciado.
// ============================================================

// Trecho<'a> não pode existir mais que o &str que ela aponta.
struct Trecho<'a> {
    conteudo: &'a str,
}

impl<'a> Trecho<'a> {
    fn novo(texto: &'a str, inicio: usize, fim: usize) -> Self {
        Trecho {
            conteudo: &texto[inicio..fim],
        }
    }

    fn exibir(&self) {
        println!("  Trecho: \"{}\"", self.conteudo);
    }

    fn tamanho(&self) -> usize {
        self.conteudo.len()
    }
}

fn exemplo_struct_lifetime() {
    let texto = String::from("aprendendo lifetimes em Rust");
    let t = Trecho::novo(&texto, 0, 10);
    t.exibir();
    println!("  Tamanho: {}", t.tamanho());
    // t não pode ser usado depois que texto for destruído.
}

// ============================================================
// 5. LIFETIME EM MÉTODOS (impl)
//    'a declarado em impl precisa aparecer depois do nome
//    do tipo. O compilador infere na maioria dos casos.
// ============================================================

struct Analisador<'a> {
    entrada: &'a str,
    posicao: usize,
}

impl<'a> Analisador<'a> {
    fn novo(entrada: &'a str) -> Self {
        Analisador { entrada, posicao: 0 }
    }

    // Retorna &self — lifetime do retorno = lifetime de self
    fn restante(&self) -> &str {
        &self.entrada[self.posicao..]
    }

    // Avança n caracteres
    fn avancar(&mut self, n: usize) {
        self.posicao = (self.posicao + n).min(self.entrada.len());
    }

    // Retorna próxima "palavra" (até espaço ou fim)
    fn proxima_palavra(&mut self) -> Option<&'a str> {
        let resto = &self.entrada[self.posicao..];
        if resto.is_empty() { return None; }

        let fim = resto.find(' ').unwrap_or(resto.len());
        let palavra = &self.entrada[self.posicao..self.posicao + fim];
        self.posicao += fim + 1;
        Some(palavra)
    }
}

fn exemplo_analisador() {
    let codigo = String::from("fn main ( ) { }");
    let mut parser = Analisador::novo(&codigo);

    print!("  Tokens: ");
    while let Some(tok) = parser.proxima_palavra() {
        print!("\"{}\" ", tok);
    }
    println!();
}

// ============================================================
// 6. LIFETIME 'STATIC
//    'static significa que a referência é válida pelo
//    programa inteiro. Literais de string (&str) têm 'static
//    porque ficam gravadas no binário.
// ============================================================

// Retornar 'static é sempre seguro: vive para sempre.
fn mensagem_fixa() -> &'static str {
    "Rust é seguro por padrão"   // gravada no binário
}

// Trait objects em heap também podem ser 'static:
fn criar_exibidor() -> Box<dyn std::fmt::Display + 'static> {
    Box::new(String::from("valor em heap com 'static"))
}

fn exemplo_static() {
    let m = mensagem_fixa();
    println!("  'static: \"{}\"", m);

    let e = criar_exibidor();
    println!("  Box 'static: {}", e);

    // 'static pode ser usado onde um lifetime menor é esperado:
    let s: &'static str = "sempre vivo";
    let curto: &str = s; // ok: 'static satisfaz qualquer 'a
    println!("  Encurtado: {}", curto);
}

// ============================================================
// 7. MÚLTIPLOS LIFETIMES
//    Quando diferentes parâmetros têm tempos de vida
//    independentes, usamos 'a, 'b, 'c...
// ============================================================

// x e y têm lifetimes diferentes; retorno vem só de x.
fn prefixo<'a, 'b>(x: &'a str, _separador: &'b str) -> &'a str {
    // _separador é usado como delimitador lógico mas não
    // aparece no retorno — seu lifetime é independente.
    x.split_at(x.len() / 2).0
}

// Struct com dois lifetimes independentes
struct Par<'a, 'b> {
    esquerda: &'a str,
    direita:  &'b str,
}

impl<'a, 'b> Par<'a, 'b> {
    fn combinar(&self) -> String {
        format!("{} + {}", self.esquerda, self.direita)
    }
}

fn exemplo_multiplos_lifetimes() {
    let texto1 = String::from("alfa");
    let resultado;
    {
        let texto2 = String::from("separador");
        resultado = prefixo(&texto1, &texto2);
        // resultado usa 'a (texto1), independente de 'b (texto2)
        println!("  Prefixo: \"{}\"", resultado);
    }
    // resultado ainda válido aqui: depende só de texto1
    println!("  Ainda válido: \"{}\"", resultado);

    let a = String::from("esquerda");
    let b = String::from("direita");
    let par = Par { esquerda: &a, direita: &b };
    println!("  Par: {}", par.combinar());
}

// ============================================================
// 8. LIFETIME ELISION (omissão de anotação)
//    O compilador infere lifetimes em 3 situações comuns,
//    dispensando anotação explícita.
// ============================================================

// Regra 1: cada parâmetro referência recebe seu próprio lifetime
// Regra 2: se há só um parâmetro referência, o retorno herda dele
// Regra 3: se há &self ou &mut self, o retorno herda de self

// Sem anotação (elision aplica regra 2):
fn primeira_palavra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }
    &s[..]
}

// Equivalente com anotação explícita:
fn primeira_palavra_explicita<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }
    &s[..]
}

fn exemplo_elision() {
    let frase = String::from("Olá mundo Rust");
    let p1 = primeira_palavra(&frase);
    let p2 = primeira_palavra_explicita(&frase);
    println!("  Elision:   \"{}\"", p1);
    println!("  Explícito: \"{}\"", p2);
}

// ============================================================
// 9. LIFETIME + GENERICS + TRAITS JUNTOS
//    O caso mais completo: struct genérica com referência,
//    trait bound e lifetime ao mesmo tempo.
// ============================================================

use std::fmt::Display;

// T pode ser qualquer tipo que implemente Display.
// 'a garante que a referência anuncio vive o suficiente.
struct Noticia<'a, T: Display> {
    titulo:  &'a str,
    corpo:   &'a str,
    destaque: T,
}

impl<'a, T: Display> Noticia<'a, T> {
    fn nova(titulo: &'a str, corpo: &'a str, destaque: T) -> Self {
        Noticia { titulo, corpo, destaque }
    }

    fn publicar(&self) {
        println!("  === {} ===", self.titulo);
        println!("  {}", self.corpo);
        println!("  Destaque: {}", self.destaque);
    }
}

// Função com lifetime + generic + trait bound
fn mais_longo_com_contexto<'a, T>(
    x:       &'a str,
    y:       &'a str,
    contexto: T,
) -> &'a str
where
    T: Display,
{
    println!("  Contexto: {}", contexto);
    if x.len() >= y.len() { x } else { y }
}

fn exemplo_lifetime_generics_traits() {
    let t1 = String::from("Rust 2024 Edition lançada");
    let t2 = String::from("Nova versão disponível");

    let noticia = Noticia::nova(&t1, "Confira as novidades...", 42_u32);
    noticia.publicar();

    let resultado = mais_longo_com_contexto(&t1, &t2, "comparação de títulos");
    println!("  Mais longo: \"{}\"", resultado);
}

// ============================================================
// 10. CASOS REAIS: slice, split, parser de CSV simples
//     Lifetimes são onipresentes em código que trabalha com
//     fatias de strings sem alocar nova memória.
// ============================================================

// Divide uma linha CSV retornando fatias do original —
// zero alocação, zero cópia, lifetime garante segurança.
fn campos_csv<'a>(linha: &'a str, delimitador: char) -> Vec<&'a str> {
    linha.split(delimitador).map(|c| c.trim()).collect()
}

// Busca em um slice de strings sem copiar
fn buscar<'a>(haystack: &'a [&str], agulha: &str) -> Option<&'a str> {
    haystack.iter().find(|&&s| s.contains(agulha)).copied()
}

// Mini parser: extrai chave e valor de "chave=valor"
fn parse_par<'a>(entrada: &'a str) -> Option<(&'a str, &'a str)> {
    let mut partes = entrada.splitn(2, '=');
    let chave = partes.next()?.trim();
    let valor = partes.next()?.trim();
    Some((chave, valor))
}

fn exemplo_casos_reais() {
    // CSV sem alocação extra
    let linha = "Alice, 30, Engenheira, São Paulo";
    let campos = campos_csv(linha, ',');
    println!("  Campos CSV:");
    for (i, c) in campos.iter().enumerate() {
        println!("    [{}] {}", i, c);
    }

    // Busca em slice
    let nomes = ["Alice", "Bob", "Carlos", "Ana"];
    match buscar(&nomes, "Car") {
        Some(n) => println!("  Encontrado: {}", n),
        None    => println!("  Não encontrado"),
    }

    // Parser de configuração
    let config = vec![
        "host=localhost",
        "porta=8080",
        "debug=true",
    ];
    println!("  Configuração:");
    for linha in &config {
        if let Some((k, v)) = parse_par(linha) {
            println!("    {} → {}", k, v);
        }
    }
}

// ============================================================
// MAIN
// ============================================================

fn main() {
    println!("=== 1. Sem Dangling Pointer ===");
    let s = sem_dangling();
    println!("  Retorno seguro: \"{}\"", s);

    println!("\n=== 2. Borrow Checker ===");
    exemplo_borrow_checker();

    println!("\n=== 3. Lifetime em Funções ===");
    exemplo_maior_str();
    let p = primeiro("primeiro", "segundo");
    println!("  primeiro(): \"{}\"", p);

    println!("\n=== 4. Lifetime em Structs ===");
    exemplo_struct_lifetime();

    println!("\n=== 5. Lifetime em Métodos ===");
    exemplo_analisador();

    println!("\n=== 6. Lifetime 'static ===");
    exemplo_static();

    println!("\n=== 7. Múltiplos Lifetimes ===");
    exemplo_multiplos_lifetimes();

    println!("\n=== 8. Lifetime Elision ===");
    exemplo_elision();

    println!("\n=== 9. Lifetime + Generics + Traits ===");
    exemplo_lifetime_generics_traits();

    println!("\n=== 10. Casos Reais ===");
    exemplo_casos_reais();
}

// ============================================================
// RESUMO — Regras de ouro dos Lifetimes
// ============================================================
//
//  Conceito              | Significado
//  ----------------------|--------------------------------------
//  'a                    | Parâmetro de lifetime (genérico)
//  &'a T                 | Referência a T válida por 'a
//  fn f<'a>(x: &'a str)  | x vive pelo menos enquanto 'a
//  -> &'a str            | Retorno vive tanto quanto 'a
//  struct S<'a>          | S não pode sobreviver ao dado 'a
//  'static               | Referência válida pelo programa todo
//  Elision               | Compilador infere 'a em casos simples
//
//  As 3 regras de Elision:
//    1. Cada &parâmetro recebe seu próprio lifetime implícito
//    2. Se há só um &parâmetro, o retorno herda dele
//    3. Se há &self / &mut self, o retorno herda de self
//
//  Dangling pointer → Rust recusa compilar. Sem exceções.
//  Custo em runtime → ZERO. É análise puramente estática.
// ============================================================
