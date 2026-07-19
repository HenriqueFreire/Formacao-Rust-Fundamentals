// ============================================================
//  ENTENDENDO A MARCAÇÃO LIFETIME EM RUST
// ============================================================
//
//  A anotação de lifetime ('a, 'b, 'static…) é uma das partes
//  mais confusas para quem está aprendendo Rust. Este arquivo
//  desmonta o tema peça por peça:
//
//  1.  O que a marcação NÃO faz (mito mais comum)
//  2.  O que a marcação FAZ de verdade
//  3.  Lendo a sintaxe em voz alta
//  4.  A marcação como contrato entre chamador e função
//  5.  Contrato em structs — o que 'a significa no tipo
//  6.  Quando o compilador EXIGE a marcação (e quando não)
//  7.  A interseção de lifetimes — por que 'a é o menor
//  8.  Marcação em impl blocks
//  9.  Subtyping de lifetime ('a: 'b — 'a vive mais que 'b)
//  10. Como ler erros de lifetime do compilador
//
//  Execute com:  rustc marcacao_lifetime.rs && ./marcacao_lifetime
// ============================================================

// ============================================================
// 1. O QUE A MARCAÇÃO *NÃO* FAZ
//
//  Erro de interpretação muito comum:
//  "Colocar 'a faz a referência viver mais tempo."
//
//  ERRADO. A marcação não altera absolutamente nada em
//  tempo de execução. Ela é uma INSTRUÇÃO AO COMPILADOR
//  sobre como os lifetimes dos parâmetros se relacionam
//  entre si.
//
//  Analogia: um rótulo num copo d'água não muda o quanto
//  de água há no copo — apenas descreve o conteúdo.
// ============================================================

/*  Descomentar mostra o erro:

fn nao_funciona<'a>(x: &str) -> &'a str {
    let local = String::from("local");
    &local  // ERRO: local não vive até 'a
            // A marcação 'a não "estende" a vida de local.
}

*/

// A marcação descreve; quem determina o lifetime real
// é o ESCOPO da variável no código-fonte.

// ============================================================
// 2. O QUE A MARCAÇÃO FAZ DE VERDADE
//
//  Ela cria um VÍNCULO entre os lifetimes dos parâmetros
//  e/ou do valor de retorno, dizendo ao compilador:
//  "estas referências estão relacionadas — use isso para
//   verificar se o código é seguro."
// ============================================================

//  SEM vínculo — compilador não sabe de onde vem o retorno:
//
//  fn escolher(x: &str, y: &str) -> &str { ... } // ERRO

//  COM vínculo — compilador sabe que o retorno vem de x ou y,
//  e que ambos devem ser válidos enquanto o retorno for usado:

fn escolher<'a>(x: &'a str, y: &'a str, usar_x: bool) -> &'a str {
    if usar_x { x } else { y }
}

//  O vínculo 'a diz: "o retorno vive enquanto AMBOS x e y
//  estiverem vivos" (pois qualquer um pode ser retornado).

// ============================================================
// 3. LENDO A SINTAXE EM VOZ ALTA
//
//  Cada forma de escrita tem uma leitura natural.
//  Praticar isso elimina a confusão.
// ============================================================

//  &'a str
//  -> "uma referência para str que é válida por 'a"

//  fn f<'a>(s: &'a str) -> &'a str
//  -> "para algum lifetime 'a: recebe uma &str válida por 'a
//      e retorna uma &str que é válida pelo mesmo 'a"

//  struct S<'a> { campo: &'a str }
//  -> "S carrega uma referência para str válida por 'a;
//      S não pode viver mais que 'a"

//  impl<'a> S<'a>
//  -> "implementação de S para qualquer 'a"

//  fn f<'a: 'b, 'b>(x: &'a str, y: &'b str) -> &'b str
//  -> "'a vive pelo menos tanto quanto 'b;
//      retornamos uma referência válida por 'b"

// Exemplo concreto de cada forma:

fn recebe_e_retorna<'a>(s: &'a str) -> &'a str {
    // "para qualquer 'a: recebo &str de 'a, devolvo &str de 'a"
    s
}

struct Envoltorio<'a> {
    // "Envoltorio não vive mais que o &str que guarda"
    interno: &'a str,
}

impl<'a> Envoltorio<'a> {
    fn novo(s: &'a str) -> Self { Envoltorio { interno: s } }
    fn conteudo(&self) -> &str { self.interno }
}

fn exemplo_leitura() {
    let texto = String::from("lendo lifetimes");
    let e = Envoltorio::novo(&texto);
    println!("  conteúdo: \"{}\"", e.conteudo());

    let a = String::from("longa");
    let resultado;
    {
        let b = String::from("x");
        resultado = escolher(&a, &b, true);
        println!("  escolhido: \"{}\"", resultado);
    }
}

// ============================================================
// 4. A MARCAÇÃO COMO CONTRATO ENTRE CHAMADOR E FUNÇÃO
//
//  Pense em 'a como uma promessa feita pelo CHAMADOR:
//  "Eu garanto que as referências passadas serão válidas
//   pelo tempo que eu usar o retorno."
//
//  O compilador verifica se essa promessa é cumprida.
// ============================================================

// Contrato: "você me dá dois &str que vivem pelo menos 'a,
//            e eu lhe devolvo um &str que vive 'a."
fn mais_longo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn exemplo_contrato() {
    println!("  -- contrato cumprido --");
    let s1 = String::from("string longa");

    // Bloco interno: s2 vive menos que s1
    {
        let s2 = String::from("xy");
        // 'a = intersecao dos escopos = escopo de s2 (o menor)
        // resultado só pode ser usado DENTRO deste bloco
        let resultado = mais_longo(s1.as_str(), s2.as_str());
        println!("  mais_longo: \"{}\"", resultado);
    } // s2 morre aqui — resultado não pode sair deste bloco

    println!("  -- contrato violado (comentado) --");
    /*
    let resultado_externo;
    {
        let s2 = String::from("xy");
        resultado_externo = mais_longo(s1.as_str(), s2.as_str());
        // ERRO: resultado_externo usaria 'a = escopo de s2,
        // mas s2 morre antes de resultado_externo ser usado.
    }
    println!("{}", resultado_externo); // uso apos destruição!
    */
}

// ============================================================
// 5. CONTRATO EM STRUCTS — O QUE 'a SIGNIFICA NO TIPO
//
//  Quando uma struct declara 'a, o compilador entende:
//  "Instâncias desse tipo não podem ter um escopo mais longo
//   do que o dado referenciado por 'a."
// ============================================================

struct Fragmento<'a> {
    texto:  &'a str,
    inicio: usize,
    fim:    usize,
}

impl<'a> Fragmento<'a> {
    fn novo(texto: &'a str, inicio: usize, fim: usize) -> Self {
        assert!(fim <= texto.len(), "fim fora dos limites");
        Fragmento { texto, inicio, fim }
    }

    // &self: o retorno herda o lifetime de self (elision regra 3)
    // Na prática, retorna 'a pois self.texto é 'a
    fn fatia(&self) -> &'a str {
        &self.texto[self.inicio..self.fim]
    }

    fn comprimento(&self) -> usize {
        self.fim - self.inicio
    }
}

fn exemplo_struct_contrato() {
    let fonte = String::from("Rust é seguro e rápido");

    let frag1 = Fragmento::novo(&fonte, 0, 4);
    let frag2 = Fragmento::novo(&fonte, 8, 14);

    println!("  frag1: \"{}\" ({} chars)", frag1.fatia(), frag1.comprimento());
    println!("  frag2: \"{}\" ({} chars)", frag2.fatia(), frag2.comprimento());

    // Fragmento não pode sobreviver à 'fonte'.
    // O compilador garante isso sem nenhum custo em runtime.
}

// ============================================================
// 6. QUANDO O COMPILADOR EXIGE A MARCAÇÃO
//
//  O compilador só pede anotação explícita quando NÃO CONSEGUE
//  inferir o vínculo. Isso ocorre em 3 situações:
//
//  a) Função retorna referência com ambiguidade de origem
//  b) Struct/Enum armazena referência
//  c) Trait object com referência interna
// ============================================================

// (a) Retorno ambíguo -> exige 'a
fn mais_curto<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() <= y.len() { x } else { y }
}

// (b) Struct com referência -> exige 'a (visto nos exemplos anteriores)

// (c) Trait object com referência -> exige 'a
trait Descricao {
    fn descricao(&self) -> &str;
}

struct Item<'a> {
    // Sem 'a, o compilador não saberia o lifetime do &str interno
    info: &'a str,
}

impl<'a> Descricao for Item<'a> {
    fn descricao(&self) -> &str { self.info }
}

// Trait object: Box<dyn Trait + 'a> — 'a é o lifetime do objeto
fn criar_item<'a>(info: &'a str) -> Box<dyn Descricao + 'a> {
    Box::new(Item { info })
}

fn exemplo_quando_exige() {
    let desc = String::from("item importante");
    let item = criar_item(&desc);
    println!("  descrição: \"{}\"", item.descricao());
}

// ============================================================
// 7. A INTERSEÇÃO DE LIFETIMES — POR QUE 'a É O MENOR
//
//  Quando dois parâmetros compartilham 'a, o compilador
//  usa a INTERSEÇÃO (o menor) dos dois escopos reais.
//  Isso garante que o retorno é válido em todos os casos.
// ============================================================

//  Visualização:
//
//  fn mais_longo<'a>(x: &'a str, y: &'a str) -> &'a str
//
//  escopo de x: |========================|
//  escopo de y:      |==========|
//  'a (intersecao):  |==========|
//  retorno válido:   |==========|
//
//  O retorno pode ser x OU y — portanto deve ser válido
//  apenas enquanto AMBOS são válidos: o menor dos dois.

fn demonstrar_intersecao() {
    let longa = String::from("string longa longa longa");

    // 'a = escopo do bloco (o menor)
    let resultado = {
        let curta = String::from("curta");
        // 'a = intersecao = escopo de curta
        let r = mais_longo(longa.as_str(), curta.as_str());
        // Só podemos usar r aqui dentro (onde curta ainda vive)
        println!("  intersecao: \"{}\"", r);
        // Se quiséssemos retornar r, 'a precisaria ser maior
        String::from(r) // copia para poder usar fora do bloco
    };

    println!("  cópia fora do bloco: \"{}\"", resultado);
}

// ============================================================
// 8. MARCAÇÃO EM IMPL BLOCKS
//
//  Há dois lifetimes distintos num impl:
//  - O lifetime da referência guardada no tipo ('a)
//  - O lifetime de &self em cada chamada de método
//
//  Na maioria dos casos o compilador elide o de &self,
//  mas às vezes precisamos ser explícitos.
// ============================================================

struct Janela<'a> {
    dados:   &'a [i32],
    inicio:  usize,
    tamanho: usize,
}

impl<'a> Janela<'a> {
    fn nova(dados: &'a [i32], tamanho: usize) -> Self {
        Janela { dados, inicio: 0, tamanho }
    }

    // Retorno usa 'a (lifetime dos dados), não de &self.
    // Isso permite usar o retorno mesmo depois que self some.
    fn fatia_atual(&self) -> &'a [i32] {
        let fim = (self.inicio + self.tamanho).min(self.dados.len());
        &self.dados[self.inicio..fim]
    }

    fn avancar(&mut self) -> bool {
        if self.inicio + self.tamanho < self.dados.len() {
            self.inicio += 1;
            true
        } else {
            false
        }
    }
}

fn exemplo_impl_lifetime() {
    let numeros = vec![10, 20, 30, 40, 50, 60];
    let mut janela = Janela::nova(&numeros, 3);

    println!("  Janelas deslizantes:");
    loop {
        let fatia = janela.fatia_atual();
        let soma: i32 = fatia.iter().sum();
        println!("    {:?} -> soma = {}", fatia, soma);
        if !janela.avancar() { break; }
    }
}

// ============================================================
// 9. SUBTYPING DE LIFETIME ('a: 'b — 'a OUTLIVES 'b)
//
//  'a: 'b significa "'a vive pelo menos tanto quanto 'b".
//  Permite usar uma referência de vida longa onde se espera
//  uma de vida curta — o contrário nunca é seguro.
// ============================================================

// 'longa: 'curta garante que 'longa >= 'curta
// Retornamos 'curta, pois é o que o chamador pode usar.
fn pegar_curta<'longa: 'curta, 'curta>(
    x: &'longa str,
    y: &'curta str,
) -> &'curta str {
    // x vive mais que y; podemos retornar qualquer um como 'curta
    if x.len() > y.len() { x } else { y }
}

// Struct que exige que 'b viva pelo menos tanto quanto 'a
struct Referenciador<'a, 'b: 'a> {
    principal:  &'a str,
    secundario: &'b str,
}

impl<'a, 'b: 'a> Referenciador<'a, 'b> {
    fn novo(principal: &'a str, secundario: &'b str) -> Self {
        Referenciador { principal, secundario }
    }

    fn exibir(&self) {
        println!("  principal:  \"{}\"", self.principal);
        println!("  secundario: \"{}\"", self.secundario);
    }
}

fn exemplo_subtyping() {
    let longa  = String::from("referência de vida longa");
    let curta  = String::from("curta");

    let r = pegar_curta(&longa, &curta);
    println!("  pegar_curta: \"{}\"", r);

    let ref_ = Referenciador::novo(&curta, &longa);
    ref_.exibir();
    // 'b (longa) vive mais que 'a (curta): restrição 'b: 'a ok
}

// ============================================================
// 10. COMO LER ERROS DE LIFETIME DO COMPILADOR
//
//  O compilador do Rust dá mensagens detalhadas.
//  Entender o padrão das mensagens acelera muito o debug.
// ============================================================

/*
  Erro típico 1 — retorno não vive o suficiente:

  fn ruim() -> &str {
      let s = String::from("local");
      &s
  }
  // error[E0106]: missing lifetime specifier
  // help: this function's return type contains a borrowed value,
  //       but there is no value for it to be borrowed from

  Leitura: "Você quer retornar uma referência, mas o dado
            original some quando a função termina."
  Solucao: retornar String (por valor) ou receber &str como parâmetro.

  ---

  Erro típico 2 — referência não vive o suficiente:

  let r;
  {
      let x = 5;
      r = &x;
  }
  println!("{}", r);
  // error[E0597]: `x` does not live long enough
  // note: `x` dropped here while still borrowed

  Leitura: "r tenta usar x depois que x foi destruído."
  Solucao: mover x para fora do bloco interno.

  ---

  Erro típico 3 — lifetime de retorno não corresponde:

  fn pegar<'a>(x: &'a str, y: &str) -> &'a str {
      y  // ERRO: y não tem lifetime 'a
  }
  // error[E0623]: lifetime mismatch
  // note: ...but data from `y` is returned here

  Leitura: "Você prometeu retornar algo de 'a, mas está
            retornando y que tem um lifetime diferente."
  Solucao: dar o mesmo lifetime a y, ou retornar x.
*/

// Padrão de diagnóstico — versão funcional comentada acima:

fn pegar_x<'a>(x: &'a str, _y: &str) -> &'a str {
    x // correto: retornamos x, que tem exatamente 'a
}

fn exemplo_leitura_erros() {
    let base = String::from("base de dados");
    let extra;
    {
        let auxiliar = String::from("auxiliar");
        // Usamos _y mas retornamos x — sem problema de lifetime
        extra = pegar_x(&base, &auxiliar);
        println!("  extra (dentro do bloco): \"{}\"", extra);
    }
    // extra ainda é válido: depende só de `base`, não de `auxiliar`
    println!("  extra (fora do bloco):   \"{}\"", extra);
}

// ============================================================
// MAIN
// ============================================================

fn main() {
    println!("=== 3. Lendo a Sintaxe ===");
    exemplo_leitura();

    println!("\n=== 4. Marcação como Contrato ===");
    exemplo_contrato();

    println!("\n=== 5. Contrato em Structs ===");
    exemplo_struct_contrato();

    println!("\n=== 6. Quando o Compilador Exige ===");
    exemplo_quando_exige();

    println!("\n=== 7. Interseção de Lifetimes ===");
    demonstrar_intersecao();

    println!("\n=== 8. Marcação em impl Blocks ===");
    exemplo_impl_lifetime();

    println!("\n=== 9. Subtyping de Lifetime ===");
    exemplo_subtyping();

    println!("\n=== 10. Lendo Erros de Lifetime ===");
    exemplo_leitura_erros();
}

// ============================================================
// MAPA MENTAL — Marcação Lifetime em Rust
// ============================================================
//
//  PERGUNTA                        RESPOSTA
//  ──────────────────────────────────────────────────────────
//  'a muda quanto tempo a          NAO. Apenas descreve o
//  referência vive?                vínculo entre lifetimes.
//
//  Quando anotar?                  Quando o compilador não
//                                  consegue inferir o vínculo
//                                  (ambiguidade de origem).
//
//  O que 'a em fn f<'a> significa? "Para qualquer lifetime
//                                   'a que o chamador escolher"
//
//  O que 'a em struct S<'a> faz?   Garante que S não vive
//                                   mais que o dado em 'a.
//
//  O que 'a: 'b significa?         'a vive pelo menos tanto
//                                   quanto 'b (outlives).
//
//  Custo em runtime?               ZERO. É análise estática.
//
//  Elision (omissão) é segura?     Sim. O compilador aplica
//                                   regras determinísticas.
//
//  Regras de Elision:
//    1. Cada &parâmetro -> lifetime implícito próprio
//    2. Um único &parâmetro -> retorno herda dele
//    3. &self / &mut self -> retorno herda de self
// ============================================================
