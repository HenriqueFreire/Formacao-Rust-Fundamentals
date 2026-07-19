// ============================================================
//  OUTROS CASOS INTERESSANTES DE GENERICS EM RUST
// ============================================================
//
//  Além dos usos básicos (funções, structs, enums), generics
//  aparecem em padrões muito úteis no dia a dia:
//
//  1.  Newtype Pattern
//  2.  Builder Pattern
//  3.  Estado em tempo de compilação (Typestate)
//  4.  Cache / Memoização genérica
//  5.  Repositório genérico (padrão comum em back-end)
//  6.  Conversão genérica com From / Into
//  7.  Iteradores genéricos personalizados
//  8.  Funções de ordem superior genéricas
//  9.  Tratamento de erro genérico
//  10. Composição de comportamentos via múltiplos traits
//
//  Execute com:  rustc outros_generics.rs && ./outros_generics
// ============================================================

use std::fmt::Display;
use std::collections::HashMap;

// ============================================================
// 1. NEWTYPE PATTERN
//    Embrulha um tipo existente em uma struct com um único
//    campo, criando um tipo distinto com semântica própria.
//    Generics tornam o padrão reutilizável.
// ============================================================

struct NaoVazio<T>(Vec<T>);

impl<T: Display> NaoVazio<T> {
    fn novo(primeiro: T) -> Self {
        NaoVazio(vec![primeiro])
    }

    fn adicionar(&mut self, valor: T) {
        self.0.push(valor);
    }

    // Garantia em tempo de compilação: sempre tem ao menos 1 elemento
    fn primeiro(&self) -> &T {
        &self.0[0]          // seguro — nunca estará vazio
    }

    fn listar(&self) {
        for item in &self.0 {
            print!("{} ", item);
        }
        println!();
    }
}

// Newtype para Km e Milhas — evita misturar unidades
struct Km(f64);
struct Milhas(f64);

impl Km {
    fn para_milhas(&self) -> Milhas {
        Milhas(self.0 * 0.621_371)
    }
}

impl Display for Km     { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{:.2} km", self.0) } }
impl Display for Milhas { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{:.2} mi", self.0) } }

// ============================================================
// 2. BUILDER PATTERN GENÉRICO
//    Constrói objetos complexos passo a passo.
//    Generics evitam duplicação para diferentes tipos de carga.
// ============================================================

struct Requisicao<T> {
    url:     String,
    timeout: u32,
    corpo:   Option<T>,
}

struct RequisicaoBuilder<T> {
    url:     String,
    timeout: u32,
    corpo:   Option<T>,
}

impl<T> RequisicaoBuilder<T> {
    fn novo(url: &str) -> Self {
        RequisicaoBuilder {
            url:     url.to_string(),
            timeout: 30,
            corpo:   None,
        }
    }

    fn timeout(mut self, segundos: u32) -> Self {
        self.timeout = segundos;
        self
    }

    fn corpo(mut self, dados: T) -> Self {
        self.corpo = Some(dados);
        self
    }

    fn construir(self) -> Requisicao<T> {
        Requisicao {
            url:     self.url,
            timeout: self.timeout,
            corpo:   self.corpo,
        }
    }
}

impl<T: Display> Requisicao<T> {
    fn descrever(&self) {
        println!("  URL:     {}", self.url);
        println!("  Timeout: {}s", self.timeout);
        match &self.corpo {
            Some(c) => println!("  Corpo:   {}", c),
            None    => println!("  Corpo:   (vazio)"),
        }
    }
}

// ============================================================
// 3. TYPESTATE PATTERN (estado em tempo de compilação)
//    Usa tipos como "marcadores" para representar estados.
//    O compilador impede transições inválidas — sem if/panic.
// ============================================================

// Estados — tipos vazios usados apenas como marcadores
struct Rascunho;
struct Revisao;
struct Publicado;

struct Documento<Estado> {
    conteudo: String,
    _estado:  std::marker::PhantomData<Estado>,
}

// Só disponível no estado Rascunho
impl Documento<Rascunho> {
    fn novo(conteudo: &str) -> Self {
        Documento {
            conteudo: conteudo.to_string(),
            _estado:  std::marker::PhantomData,
        }
    }

    fn editar(&mut self, novo: &str) {
        self.conteudo = novo.to_string();
        println!("  [Rascunho] Conteúdo editado.");
    }

    fn enviar_para_revisao(self) -> Documento<Revisao> {
        println!("  [Rascunho → Revisão] Enviado.");
        Documento { conteudo: self.conteudo, _estado: std::marker::PhantomData }
    }
}

// Só disponível no estado Revisao
impl Documento<Revisao> {
    fn aprovar(self) -> Documento<Publicado> {
        println!("  [Revisão → Publicado] Aprovado.");
        Documento { conteudo: self.conteudo, _estado: std::marker::PhantomData }
    }

    fn rejeitar(self) -> Documento<Rascunho> {
        println!("  [Revisão → Rascunho] Rejeitado, volta ao autor.");
        Documento { conteudo: self.conteudo, _estado: std::marker::PhantomData }
    }
}

// Só disponível no estado Publicado
impl Documento<Publicado> {
    fn conteudo(&self) -> &str {
        &self.conteudo
    }
}

// ============================================================
// 4. CACHE / MEMOIZAÇÃO GENÉRICA
//    Calcula um valor caro uma só vez e reutiliza.
//    Fn() -> T é um trait genérico para closures.
// ============================================================

struct Cache<T> {
    calculo: Box<dyn Fn() -> T>,
    valor:   Option<T>,
}

impl<T: Clone> Cache<T> {
    fn novo(calculo: impl Fn() -> T + 'static) -> Self {
        Cache {
            calculo: Box::new(calculo),
            valor:   None,
        }
    }

    fn obter(&mut self) -> &T {
        if self.valor.is_none() {
            println!("  (calculando pela primeira vez...)");
            self.valor = Some((self.calculo)());
        } else {
            println!("  (usando valor em cache)");
        }
        self.valor.as_ref().unwrap()
    }
}

// ============================================================
// 5. REPOSITÓRIO GENÉRICO
//    Padrão comum em back-end: CRUD sobre qualquer entidade
//    que tenha um id numérico.
// ============================================================

trait Entidade {
    fn id(&self) -> u32;
    fn nome(&self) -> &str;
}

struct Repositorio<T: Entidade> {
    dados: HashMap<u32, T>,
}

impl<T: Entidade> Repositorio<T> {
    fn novo() -> Self {
        Repositorio { dados: HashMap::new() }
    }

    fn salvar(&mut self, item: T) {
        println!("  Salvando \"{}\" (id={})", item.nome(), item.id());
        self.dados.insert(item.id(), item);
    }

    fn buscar(&self, id: u32) -> Option<&T> {
        self.dados.get(&id)
    }

    fn remover(&mut self, id: u32) -> Option<T> {
        self.dados.remove(&id)
    }

    fn total(&self) -> usize {
        self.dados.len()
    }
}

// Dois tipos de entidade distintos
#[derive(Debug)]
struct Usuario { id: u32, nome: String, email: String }
#[derive(Debug)]
struct Produto  { id: u32, nome: String, preco: f64   }

impl Entidade for Usuario {
    fn id(&self)   -> u32  { self.id }
    fn nome(&self) -> &str { &self.nome }
}

impl Entidade for Produto {
    fn id(&self)   -> u32  { self.id }
    fn nome(&self) -> &str { &self.nome }
}

// ============================================================
// 6. CONVERSÃO GENÉRICA COM From / Into
//    From<T> e Into<T> são traits genéricos da stdlib.
//    Implementar um deles dá o outro de graça.
// ============================================================

struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

impl Display for Celsius     { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{:.1}°C", self.0) } }
impl Display for Fahrenheit  { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{:.1}°F", self.0) } }

// Função genérica que aceita qualquer T que possa virar Fahrenheit
fn para_fahrenheit<T: Into<Fahrenheit>>(valor: T) -> Fahrenheit {
    valor.into()
}

// ============================================================
// 7. ITERADOR GENÉRICO PERSONALIZADO
//    Implementar Iterator com type Item = T genérico.
// ============================================================

struct Contador<T: Clone> {
    itens:   Vec<T>,
    posicao: usize,
    passo:   usize,
}

impl<T: Clone> Contador<T> {
    fn novo(itens: Vec<T>, passo: usize) -> Self {
        Contador { itens, posicao: 0, passo }
    }
}

impl<T: Clone> Iterator for Contador<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.posicao < self.itens.len() {
            let item = self.itens[self.posicao].clone();
            self.posicao += self.passo;
            Some(item)
        } else {
            None
        }
    }
}

// ============================================================
// 8. FUNÇÕES DE ORDEM SUPERIOR GENÉRICAS
//    Recebem e retornam closures — Fn / FnMut / FnOnce.
// ============================================================

// Aplica uma transformação em cada elemento de um Vec
fn mapear<T, U, F>(lista: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    lista.into_iter().map(f).collect()
}

// Filtra elementos que satisfazem um predicado genérico
fn filtrar<T, F>(lista: Vec<T>, pred: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    lista.into_iter().filter(|x| pred(x)).collect()
}

// Compõe duas funções: g(f(x))
fn compor<T, U, V, F, G>(f: F, g: G) -> impl Fn(T) -> V
where
    F: Fn(T) -> U,
    G: Fn(U) -> V,
{
    move |x| g(f(x))
}

// ============================================================
// 9. TRATAMENTO DE ERRO GENÉRICO
//    Result<T, E> já é genérico, mas podemos construir
//    wrappers e helpers genéricos em cima dele.
// ============================================================

#[derive(Debug)]
enum ErroApp {
    NaoEncontrado(String),
    Invalido(String),
}

impl Display for ErroApp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErroApp::NaoEncontrado(m) => write!(f, "Não encontrado: {}", m),
            ErroApp::Invalido(m)      => write!(f, "Inválido: {}", m),
        }
    }
}

// Valida qualquer T com uma closure predicado
fn validar<T, F>(valor: T, predicado: F, mensagem: &str) -> Result<T, ErroApp>
where
    F: Fn(&T) -> bool,
{
    if predicado(&valor) {
        Ok(valor)
    } else {
        Err(ErroApp::Invalido(mensagem.to_string()))
    }
}

// Transforma Ok ou propaga Err — map genérico manual
fn transformar_ok<T, U, E, F>(resultado: Result<T, E>, f: F) -> Result<U, E>
where
    F: Fn(T) -> U,
{
    match resultado {
        Ok(v)  => Ok(f(v)),
        Err(e) => Err(e),
    }
}

// ============================================================
// 10. COMPOSIÇÃO DE COMPORTAMENTOS VIA MÚLTIPLOS TRAITS
//     Um tipo T pode satisfazer vários contratos ao mesmo
//     tempo; generics expressam esses contratos com precisão.
// ============================================================

trait Serializavel {
    fn serializar(&self) -> String;
}

trait Validavel {
    fn validar(&self) -> bool;
}

// Aceita qualquer T que seja Display + Serializavel + Validavel
fn processar<T>(item: &T) -> Result<String, String>
where
    T: Display + Serializavel + Validavel,
{
    if !item.validar() {
        return Err(format!("Item inválido: {}", item));
    }
    let serializado = item.serializar();
    Ok(format!("Processado → {}", serializado))
}

struct Pedido {
    id:         u32,
    quantidade: u32,
    valor:      f64,
}

impl Display for Pedido {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pedido#{} ({}x, R${:.2})", self.id, self.quantidade, self.valor)
    }
}

impl Serializavel for Pedido {
    fn serializar(&self) -> String {
        format!(
            r#"{{"id":{},"quantidade":{},"valor":{:.2}}}"#,
            self.id, self.quantidade, self.valor
        )
    }
}

impl Validavel for Pedido {
    fn validar(&self) -> bool {
        self.quantidade > 0 && self.valor > 0.0
    }
}

// ============================================================
// MAIN
// ============================================================

fn main() {

    // --- 1. Newtype Pattern ---
    println!("=== 1. Newtype Pattern ===");
    let mut nv: NaoVazio<&str> = NaoVazio::novo("Rust");
    nv.adicionar("Go");
    nv.adicionar("Zig");
    print!("  Primeiro: {}  |  Todos: ", nv.primeiro());
    nv.listar();

    let distancia = Km(42.195);
    println!("  {} = {}", distancia, distancia.para_milhas());

    // --- 2. Builder Pattern ---
    println!("\n=== 2. Builder Pattern Genérico ===");
    let req_get = RequisicaoBuilder::<String>::novo("https://api.exemplo.com/dados")
        .timeout(10)
        .construir();
    println!("  GET:");
    req_get.descrever();

    let req_post = RequisicaoBuilder::novo("https://api.exemplo.com/usuarios")
        .timeout(5)
        .corpo(r#"{"nome":"Ana","cargo":"dev"}"#)
        .construir();
    println!("  POST:");
    req_post.descrever();

    // --- 3. Typestate Pattern ---
    println!("\n=== 3. Typestate (estados em tempo de compilação) ===");
    let mut doc = Documento::<Rascunho>::novo("Versão inicial do artigo.");
    doc.editar("Artigo revisado com mais detalhes.");
    let doc = doc.enviar_para_revisao();
    let doc = doc.aprovar();
    println!("  Conteúdo publicado: \"{}\"", doc.conteudo());
    // doc.editar(...)  ← erro de compilação: editar não existe em Publicado

    // --- 4. Cache / Memoização ---
    println!("\n=== 4. Cache / Memoização Genérica ===");
    let mut cache: Cache<u64> = Cache::novo(|| {
        // simula cálculo demorado
        (1u64..=40).fold(1, |acc, n| acc.wrapping_mul(n))
    });
    println!("  Resultado: {}", cache.obter());
    println!("  Resultado: {}", cache.obter()); // usa cache
    println!("  Resultado: {}", cache.obter()); // usa cache

    // --- 5. Repositório Genérico ---
    println!("\n=== 5. Repositório Genérico ===");
    let mut repo_usuarios: Repositorio<Usuario> = Repositorio::novo();
    repo_usuarios.salvar(Usuario { id: 1, nome: "Alice".into(), email: "alice@ex.com".into() });
    repo_usuarios.salvar(Usuario { id: 2, nome: "Bob".into(),   email: "bob@ex.com".into()   });
    if let Some(u) = repo_usuarios.buscar(1) {
        println!("  Encontrado: {} — {}", u.nome, u.email);
    }
    println!("  Total: {}", repo_usuarios.total());

    let mut repo_produtos: Repositorio<Produto> = Repositorio::novo();
    repo_produtos.salvar(Produto { id: 10, nome: "Teclado".into(), preco: 299.90 });
    println!("  Produtos: {}", repo_produtos.total());

    // --- 6. From / Into ---
    println!("\n=== 6. Conversão Genérica (From / Into) ===");
    let fervura = Celsius(100.0);
    let em_f: Fahrenheit = fervura.into();   // Into dá de graça com From
    println!("  Fervura: {}", em_f);

    let corporal = Fahrenheit(98.6);
    let em_c = Celsius::from(corporal);
    println!("  Temperatura corporal: {}", em_c);

    let resultado = para_fahrenheit(Celsius(37.0));
    println!("  37°C = {}", resultado);

    // --- 7. Iterador Genérico ---
    println!("\n=== 7. Iterador Genérico Personalizado ===");
    let letras = Contador::novo(vec!['a','b','c','d','e','f'], 2);
    print!("  Passo 2 (chars): ");
    for ch in letras { print!("{} ", ch); }
    println!();

    let numeros = Contador::novo(vec![0,1,2,3,4,5,6,7,8,9], 3);
    let soma: i32 = numeros.sum();
    println!("  Soma passo 3: {}", soma);

    // --- 8. Funções de Ordem Superior ---
    println!("\n=== 8. Funções de Ordem Superior Genéricas ===");
    let nums = vec![1, 2, 3, 4, 5, 6];
    let dobrados  = mapear(nums.clone(), |x| x * 2);
    let pares     = filtrar(nums.clone(), |x| x % 2 == 0);
    println!("  Dobrados: {:?}", dobrados);
    println!("  Pares:    {:?}", pares);

    let dobrar_e_texto = compor(|x: i32| x * 2, |x| format!("val={}", x));
    println!("  Composta: {}", dobrar_e_texto(7));

    // --- 9. Tratamento de Erro Genérico ---
    println!("\n=== 9. Tratamento de Erro Genérico ===");
    let idade: Result<u32, ErroApp> = validar(25, |&x| x >= 18, "Deve ser maior de idade");
    println!("  Idade 25: {:?}", idade);

    let idade_inv = validar(15u32, |&x| x >= 18, "Deve ser maior de idade");
    println!("  Idade 15: {}", idade_inv.unwrap_err());

    let nome_ok = validar("Rust".to_string(), |s| !s.is_empty(), "Nome vazio");
    let tamanho = transformar_ok(nome_ok, |s| s.len());
    println!("  Tamanho do nome: {:?}", tamanho);

    // --- 10. Composição de Traits ---
    println!("\n=== 10. Composição de Comportamentos via Traits ===");
    let pedido_ok  = Pedido { id: 1, quantidade: 3,  valor: 59.90 };
    let pedido_inv = Pedido { id: 2, quantidade: 0,  valor: 0.0   };

    match processar(&pedido_ok) {
        Ok(s)  => println!("  ✓ {}", s),
        Err(e) => println!("  ✗ {}", e),
    }
    match processar(&pedido_inv) {
        Ok(s)  => println!("  ✓ {}", s),
        Err(e) => println!("  ✗ {}", e),
    }
}

// ============================================================
// RESUMO — Quando usar cada padrão genérico
// ============================================================
//
//  Padrão                  | Use quando...
//  ------------------------|---------------------------------------
//  Newtype<T>              | Precisar de semântica distinta para
//                          | um tipo existente (ex: Km vs Milhas)
//  Builder<T>              | Construir objetos complexos passo a
//                          | passo sem expor o estado interno
//  Typestate<Estado>       | Garantir transições de estado válidas
//                          | em tempo de compilação (sem runtime)
//  Cache<T>                | Calcular valores caros uma só vez e
//                          | reutilizar com tipagem flexível
//  Repositorio<T: Entidade>| CRUD reutilizável para qualquer
//                          | entidade do domínio
//  From<T> / Into<T>       | Conversões idiomáticas entre tipos
//                          | com zero custo em tempo de execução
//  Iterator<Item=T>        | Sequências preguiçosas de qualquer
//                          | tipo, compossíveis com .map/.filter
//  Fn(T) -> U              | Callbacks e pipelines de dados
//                          | totalmente genéricos
//  Result<T,E> + generics  | Validação e propagação de erros
//                          | flexíveis e reutilizáveis
//  T: A + B + C            | Exigir múltiplos contratos ao mesmo
//                          | tempo — composição de traits
// ============================================================
