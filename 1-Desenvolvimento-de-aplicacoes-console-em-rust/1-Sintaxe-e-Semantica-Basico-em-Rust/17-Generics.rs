// ============================================================
//  GENERICS EM RUST — Guia com Exemplos
// ============================================================
//
//  Generics (tipos genéricos) permitem escrever código que
//  funciona para múltiplos tipos, sem repetição. Em vez de
//  criar uma função separada para i32, f64, String etc.,
//  você escreve uma só usando um parâmetro de tipo (ex: T).
//
//  Execute com:  rustc generics.rs && ./generics
// ============================================================

// ------------------------------------------------------------
// 1. FUNÇÃO GENÉRICA
//    <T> declara T como um parâmetro de tipo.
//    O trait bound `PartialOrd` exige que T suporte comparação.
// ------------------------------------------------------------

fn maior<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// ------------------------------------------------------------
// 2. STRUCT GENÉRICA
//    Par<T> pode guardar dois valores de qualquer tipo T.
// ------------------------------------------------------------

struct Par<T> {
    primeiro: T,
    segundo: T,
}

// Implementação de métodos para qualquer T
impl<T> Par<T> {
    fn novo(primeiro: T, segundo: T) -> Self {
        Par { primeiro, segundo }
    }
}

// Implementação condicional: só para T que suporta Display + PartialOrd
use std::fmt::Display;

impl<T: Display + PartialOrd> Par<T> {
    fn exibir_maior(&self) {
        if self.primeiro >= self.segundo {
            println!("O maior é: {}", self.primeiro);
        } else {
            println!("O maior é: {}", self.segundo);
        }
    }
}

// ------------------------------------------------------------
// 3. ENUM GENÉRICO
//    Parecido com Option<T> e Result<T, E> da stdlib.
// ------------------------------------------------------------

enum Resultado<T, E> {
    Ok(T),
    Erro(E),
}

fn dividir(a: f64, b: f64) -> Resultado<f64, String> {
    if b == 0.0 {
        Resultado::Erro(String::from("Divisão por zero!"))
    } else {
        Resultado::Ok(a / b)
    }
}

// ------------------------------------------------------------
// 4. MÚLTIPLOS PARÂMETROS DE TIPO
//    Diferentes tipos para campos distintos.
// ------------------------------------------------------------

struct Caixa<T, U> {
    inteiro: T,
    flutuante: U,
}

impl<T: Display, U: Display> Caixa<T, U> {
    fn mostrar(&self) {
        println!("inteiro = {}, flutuante = {}", self.inteiro, self.flutuante);
    }
}

// ------------------------------------------------------------
// 5. GENERICS COM TRAITS (Trait Bounds)
//    Exigimos que T implemente o trait `Resumivel`.
// ------------------------------------------------------------

trait Resumivel {
    fn resumo(&self) -> String;
}

struct Artigo {
    titulo: String,
    autor: String,
}

struct Tweet {
    usuario: String,
    conteudo: String,
}

impl Resumivel for Artigo {
    fn resumo(&self) -> String {
        format!("{} — por {}", self.titulo, self.autor)
    }
}

impl Resumivel for Tweet {
    fn resumo(&self) -> String {
        format!("@{}: {}", self.usuario, self.conteudo)
    }
}

// Função genérica que aceita qualquer T que implemente Resumivel
fn notificar<T: Resumivel>(item: &T) {
    println!("Notificação: {}", item.resumo());
}

// Sintaxe alternativa com `impl Trait` (mais legível para um único parâmetro)
fn notificar_impl(item: &impl Resumivel) {
    println!("(impl Trait) Notificação: {}", item.resumo());
}

// ------------------------------------------------------------
// 6. WHERE CLAUSE
//    Deixa a assinatura mais limpa quando há muitos bounds.
// ------------------------------------------------------------

fn comparar_e_exibir<T, U>(t: &T, u: &U) -> String
where
    T: Display + PartialOrd,
    U: Display,
{
    format!("t = {}, u = {}", t, u)
}

// ------------------------------------------------------------
// 7. STRUCTS GENÉRICAS COM LIFETIME (bônus)
//    'a garante que a referência vive o suficiente.
// ------------------------------------------------------------

struct Trecho<'a> {
    parte: &'a str,
}

impl<'a> Trecho<'a> {
    fn exibir(&self) {
        println!("Trecho: \"{}\"", self.parte);
    }
}

// ------------------------------------------------------------
// 8. PILHA GENÉRICA (exemplo prático)
//    Implementação simples de Stack<T> usando Vec<T>.
// ------------------------------------------------------------

struct Pilha<T> {
    elementos: Vec<T>,
}

impl<T> Pilha<T> {
    fn nova() -> Self {
        Pilha { elementos: Vec::new() }
    }

    fn empurrar(&mut self, valor: T) {
        self.elementos.push(valor);
    }

    fn retirar(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    fn esta_vazia(&self) -> bool {
        self.elementos.is_empty()
    }

    fn topo(&self) -> Option<&T> {
        self.elementos.last()
    }
}

// ============================================================
// MAIN — demonstra todos os exemplos acima
// ============================================================

fn main() {
    println!("=== 1. Função Genérica ===");
    println!("maior(3, 7)       = {}", maior(3, 7));
    println!("maior(2.5, 1.1)   = {}", maior(2.5, 1.1));
    println!("maior('a', 'z')   = {}", maior('a', 'z'));

    println!("\n=== 2. Struct Genérica ===");
    let par_int = Par::novo(10, 20);
    par_int.exibir_maior();
    let par_str = Par::novo("banana", "abacaxi");
    par_str.exibir_maior();

    println!("\n=== 3. Enum Genérico ===");
    match dividir(10.0, 3.0) {
        Resultado::Ok(v)  => println!("10 / 3 = {:.4}", v),
        Resultado::Erro(e) => println!("Erro: {}", e),
    }
    match dividir(5.0, 0.0) {
        Resultado::Ok(v)  => println!("Resultado: {}", v),
        Resultado::Erro(e) => println!("Erro capturado: {}", e),
    }

    println!("\n=== 4. Múltiplos Parâmetros de Tipo ===");
    let caixa = Caixa { inteiro: 42_i32, flutuante: 3.14_f64 };
    caixa.mostrar();

    println!("\n=== 5. Generics com Traits ===");
    let artigo = Artigo {
        titulo: String::from("Rust para iniciantes"),
        autor: String::from("Maria"),
    };
    let tweet = Tweet {
        usuario: String::from("joao"),
        conteudo: String::from("Rust é incrível!"),
    };
    notificar(&artigo);
    notificar(&tweet);
    notificar_impl(&artigo);

    println!("\n=== 6. Where Clause ===");
    let resultado = comparar_e_exibir(&100, &"cem");
    println!("{}", resultado);

    println!("\n=== 7. Lifetime com Struct ===");
    let texto = String::from("Aprendendo generics em Rust");
    let trecho = Trecho { parte: &texto[0..19] };
    trecho.exibir();

    println!("\n=== 8. Pilha Genérica (Stack<T>) ===");
    let mut pilha: Pilha<i32> = Pilha::nova();
    pilha.empurrar(1);
    pilha.empurrar(2);
    pilha.empurrar(3);
    println!("Topo: {:?}", pilha.topo());
    while let Some(v) = pilha.retirar() {
        println!("  retirado: {}", v);
    }
    println!("Pilha vazia? {}", pilha.esta_vazia());
}

// ============================================================
// RESUMO RÁPIDO
// ============================================================
//
//  Sintaxe          | Onde usar
//  -----------------+------------------------------------------
//  fn f<T>()        | Função genérica
//  struct S<T>      | Struct genérica
//  impl<T> S<T>     | Implementação para struct genérica
//  enum E<T>        | Enum genérico
//  T: Trait         | Trait bound (exige que T implemente Trait)
//  where T: Trait   | Trait bound em cláusula where (mais limpo)
//  impl Trait       | Atalho para parâmetro genérico simples
//  'a               | Lifetime (tempo de vida de referências)
//
//  Custo em tempo de execução: ZERO.
//  Rust usa "monomorphization" — gera código especializado para
//  cada tipo concreto usado em tempo de compilação.
// ============================================================
