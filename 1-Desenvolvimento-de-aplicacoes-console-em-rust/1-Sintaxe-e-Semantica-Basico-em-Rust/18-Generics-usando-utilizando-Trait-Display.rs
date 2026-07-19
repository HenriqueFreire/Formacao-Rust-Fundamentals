// ============================================================
//  GENERICS + TRAIT DISPLAY EM RUST — Guia com Exemplos
// ============================================================
//
//  O trait `Display` (std::fmt::Display) define como um tipo
//  é formatado com `{}` em macros como println!, format! etc.
//
//  Combinado com Generics, ele permite escrever funções e
//  structs que aceitam qualquer tipo — desde que esse tipo
//  saiba "se exibir" na tela.
//
//  Execute com:  rustc generics_display.rs && ./generics_display
// ============================================================

use std::fmt::Display;

// ------------------------------------------------------------
// 1. FUNÇÃO GENÉRICA COM BOUND `Display`
//    T: Display garante que podemos usar {} com qualquer T.
// ------------------------------------------------------------

fn imprimir<T: Display>(valor: T) {
    println!("Valor: {}", valor);
}

// Múltiplos parâmetros, cada um com Display
fn imprimir_par<T: Display, U: Display>(a: T, b: U) {
    println!("a = {}  |  b = {}", a, b);
}

// ------------------------------------------------------------
// 2. STRUCT GENÉRICA COM Display
//    Invólucro<T> embala qualquer valor que saiba se exibir.
// ------------------------------------------------------------

struct Involucro<T: Display> {
    valor: T,
    rotulo: String,
}

impl<T: Display> Involucro<T> {
    fn novo(rotulo: &str, valor: T) -> Self {
        Involucro {
            valor,
            rotulo: rotulo.to_string(),
        }
    }

    fn exibir(&self) {
        println!("[{}] → {}", self.rotulo, self.valor);
    }
}

// Implementamos Display para Involucro<T> — agora ele também
// pode ser usado com {} em outras funções genéricas!
impl<T: Display> Display for Involucro<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] → {}", self.rotulo, self.valor)
    }
}

// ------------------------------------------------------------
// 3. IMPLEMENTANDO Display EM TIPOS PRÓPRIOS
//    Sem Display, o tipo não pode ser usado como T: Display.
// ------------------------------------------------------------

struct Ponto {
    x: f64,
    y: f64,
}

impl Display for Ponto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Cor {
    r: u8,
    g: u8,
    b: u8,
}

impl Display for Cor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

// Agora Ponto e Cor podem ser usados em qualquer fn<T: Display>
fn descrever<T: Display>(nome: &str, item: T) {
    println!("{}: {}", nome, item);
}

// ------------------------------------------------------------
// 4. MÚLTIPLOS TRAIT BOUNDS (Display + outros)
//    Use `+` para exigir mais de um trait ao mesmo tempo.
// ------------------------------------------------------------

// T deve ser Display E PartialOrd (para comparar)
fn maior_com_aviso<T: Display + PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        println!("  {} >= {}, retornando o primeiro.", a, b);
        a
    } else {
        println!("  {} < {}, retornando o segundo.", a, b);
        b
    }
}

// T deve ser Display E Clone (para duplicar sem mover)
fn duplicar_e_exibir<T: Display + Clone>(valor: T) {
    let copia = valor.clone();
    println!("Original: {}  |  Cópia: {}", valor, copia);
}

// ------------------------------------------------------------
// 5. WHERE CLAUSE com Display
//    Deixa a assinatura legível quando há muitos bounds.
// ------------------------------------------------------------

fn formatar_tabela<T, U>(chave: T, valor: U) -> String
where
    T: Display,
    U: Display,
{
    format!("{:<20} | {}", chave, valor)
}

// ------------------------------------------------------------
// 6. RETORNAR String FORMATADA GENERICAMENTE
//    Útil para logging, relatórios, serialização simples.
// ------------------------------------------------------------

fn para_string<T: Display>(valor: T) -> String {
    format!("{}", valor)
}

fn envolver_em_tags<T: Display>(tag: &str, conteudo: T) -> String {
    format!("<{tag}>{}</{tag}>", conteudo, tag = tag)
}

// ------------------------------------------------------------
// 7. VEC GENÉRICO: imprimir todos os elementos
//    T: Display permite usar {} dentro do loop.
// ------------------------------------------------------------

fn imprimir_lista<T: Display>(lista: &[T]) {
    for (i, item) in lista.iter().enumerate() {
        println!("  [{}] {}", i, item);
    }
}

fn listar_formatado<T: Display>(titulo: &str, lista: &[T]) {
    println!("── {} ──", titulo);
    for item in lista {
        println!("  • {}", item);
    }
}

// ------------------------------------------------------------
// 8. STRUCT COM DISPLAY ANINHADO
//    Um tipo genérico que contém outro tipo Display dentro.
// ------------------------------------------------------------

struct Caixa<T: Display> {
    conteudo: Vec<T>,
    nome: String,
}

impl<T: Display> Caixa<T> {
    fn nova(nome: &str) -> Self {
        Caixa {
            conteudo: Vec::new(),
            nome: nome.to_string(),
        }
    }

    fn inserir(&mut self, item: T) {
        self.conteudo.push(item);
    }

    fn listar(&self) {
        println!("Caixa \"{}\" contém:", self.nome);
        for item in &self.conteudo {
            println!("  → {}", item);
        }
    }
}

impl<T: Display> Display for Caixa<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let itens: Vec<String> = self.conteudo.iter().map(|i| i.to_string()).collect();
        write!(f, "Caixa({}: [{}])", self.nome, itens.join(", "))
    }
}

// ------------------------------------------------------------
// 9. FUNÇÃO QUE ACEITA `impl Display` (açúcar sintático)
//    Equivalente a <T: Display>, porém mais concisa.
//    Limite: não dá para usar o tipo T em retorno.
// ------------------------------------------------------------

fn anunciar(mensagem: impl Display) {
    println!("📢 Anúncio: {}", mensagem);
}

fn anunciar_dois(a: impl Display, b: impl Display) {
    println!("📢 {} e {}", a, b);
}

// ------------------------------------------------------------
// 10. TRAIT PRÓPRIO QUE HERDA Display
//     Garante que todo implementador também implemente Display.
// ------------------------------------------------------------

trait Descritivel: Display {
    fn categoria(&self) -> &str;

    fn ficha(&self) -> String {
        format!("[{}] {}", self.categoria(), self)
        //                                   ^^^^ usa Display herdado
    }
}

struct Produto {
    nome: String,
    preco: f64,
}

impl Display for Produto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (R$ {:.2})", self.nome, self.preco)
    }
}

impl Descritivel for Produto {
    fn categoria(&self) -> &str {
        "Produto"
    }
}

fn exibir_ficha<T: Descritivel>(item: &T) {
    println!("{}", item.ficha());
}

// ============================================================
// MAIN — demonstra todos os exemplos
// ============================================================

fn main() {
    // --- 1. Função genérica básica ---
    println!("=== 1. Função Genérica com Display ===");
    imprimir(42);
    imprimir(3.14);
    imprimir("olá, mundo");
    imprimir(true);
    imprimir_par("Rust", 2024);
    imprimir_par(9.99_f64, "reais");

    // --- 2. Struct genérica ---
    println!("\n=== 2. Struct Genérica com Display ===");
    let w1 = Involucro::novo("Inteiro", 100_i32);
    let w2 = Involucro::novo("Texto",   "Ferrugem");
    let w3 = Involucro::novo("Float",   2.718_f64);
    w1.exibir();
    w2.exibir();
    w3.exibir();
    // Involucro implementa Display, então pode ser passado para imprimir()
    imprimir(&w1);

    // --- 3. Tipos próprios com Display ---
    println!("\n=== 3. Tipos Próprios implementando Display ===");
    let p = Ponto { x: 3.0, y: -1.5 };
    let c = Cor { r: 255, g: 128, b: 0 };
    descrever("Ponto", &p);
    descrever("Cor",   &c);
    println!("Formatado diretamente: {} e {}", p, c);

    // --- 4. Múltiplos bounds ---
    println!("\n=== 4. Múltiplos Trait Bounds (Display + outros) ===");
    let m = maior_com_aviso(10, 20);
    println!("  Maior: {}", m);
    let m2 = maior_com_aviso("zebra", "abacate");
    println!("  Maior: {}", m2);
    duplicar_e_exibir(String::from("Rust"));
    duplicar_e_exibir(42_i32);

    // --- 5. Where clause ---
    println!("\n=== 5. Where Clause com Display ===");
    println!("{}", formatar_tabela("Nome",   "Ana"));
    println!("{}", formatar_tabela("Idade",  30));
    println!("{}", formatar_tabela("Altura", 1.68_f64));

    // --- 6. Retornar String formatada ---
    println!("\n=== 6. Retornar String Genérica ===");
    let s1 = para_string(42);
    let s2 = para_string(Ponto { x: 1.0, y: 2.0 });
    println!("para_string(42)    → \"{}\"", s1);
    println!("para_string(Ponto) → \"{}\"", s2);
    println!("{}", envolver_em_tags("b", "negrito"));
    println!("{}", envolver_em_tags("em", 3.14_f64));

    // --- 7. Vec genérico ---
    println!("\n=== 7. Listas Genéricas com Display ===");
    let nums = vec![10, 20, 30, 40];
    let palavras = vec!["Rust", "Genérico", "Display"];
    imprimir_lista(&nums);
    imprimir_lista(&palavras);
    listar_formatado("Linguagens", &["Rust", "Go", "C++"]);

    // --- 8. Struct com Display aninhado ---
    println!("\n=== 8. Caixa Genérica com Display ===");
    let mut caixa_num: Caixa<i32> = Caixa::nova("Números");
    caixa_num.inserir(7);
    caixa_num.inserir(14);
    caixa_num.inserir(21);
    caixa_num.listar();
    println!("{}", caixa_num); // usa Display de Caixa<T>

    let mut caixa_str: Caixa<&str> = Caixa::nova("Frutas");
    caixa_str.inserir("maçã");
    caixa_str.inserir("banana");
    caixa_str.listar();

    // --- 9. impl Display (açúcar sintático) ---
    println!("\n=== 9. impl Display (açúcar sintático) ===");
    anunciar("Rust 2024 Edition disponível!");
    anunciar(42);
    anunciar(Cor { r: 0, g: 200, b: 100 });
    anunciar_dois("temperatura", 36.5_f64);

    // --- 10. Trait herdando Display ---
    println!("\n=== 10. Trait próprio que herda Display ===");
    let prod = Produto {
        nome: String::from("Teclado Mecânico"),
        preco: 349.90,
    };
    println!("{}", prod);       // usa Display
    exibir_ficha(&prod);        // usa Descritivel (que herda Display)
}

// ============================================================
// RESUMO — Formas de usar T: Display
// ============================================================
//
//  Forma                          | Quando usar
//  -------------------------------|----------------------------
//  fn f<T: Display>(v: T)         | Parâmetro genérico padrão
//  fn f(v: impl Display)          | Atalho, sem nomear T
//  fn f<T, U>(a: T, b: U)        | Tipos diferentes por param
//    where T: Display, U: Display |
//  struct S<T: Display>           | Struct que embala T exibível
//  impl<T: Display> Display for S | S também vira Display
//  trait Meu: Display             | Trait que exige Display
//
//  Macros que usam Display:
//    println!("{}", x)
//    print!("{}", x)
//    format!("{}", x)
//    write!(f, "{}", x)
//    eprintln!("{}", x)   ← stderr
//    to_string()          ← qualquer T: Display vira String
// ============================================================
