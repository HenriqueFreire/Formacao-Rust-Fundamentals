// Rodando nosso primeiro projeto em Rust

/*
Agora que o ambiente está configurado, vamos criar e executar o seu primeiro projeto utilizando o Cargo, a ferramenta oficial do Rust para gerenciamento de projetos.
*/

// 1. Criando o Projeto
// O comando 'cargo new' cria uma nova pasta com a estrutura básica de um projeto Rust.
/*
No terminal:
$ cargo new ola_mundo
*/

// 2. Entendendo a Estrutura do Projeto
// Após o comando anterior, você terá a seguinte estrutura:
/*
ola_mundo/
├── Cargo.toml  // Arquivo de configuração e dependências
└── src/
    └── main.rs // Onde o código-fonte principal reside
*/

// 3. O Código 'Hello World'
// Por padrão, o Rust cria um arquivo 'src/main.rs' com este conteúdo:
/*
fn main() {
    println!("Hello, world!");
}
*/

// 4. Executando o Projeto
// Existem duas formas principais de rodar o seu código:

// A) Compilar e rodar em um único passo (Mais comum no desenvolvimento):
/*
$ cd ola_mundo
$ cargo run

Saída:
   Compiling ola_mundo v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in ...s
     Running `target/debug/ola_mundo`
Hello, world!
*/

// B) Apenas compilar (Para verificar erros ou preparar o binário):
/*
$ cargo build

Isso cria um executável em: target/debug/ola_mundo
*/

// 5. Compilação para Produção (Release)
// Quando você estiver pronto para distribuir seu programa, use a flag --release.
// Isso otimiza o código para performance máxima, mas a compilação demora um pouco mais.
/*
$ cargo build --release

O executável otimizado estará em: target/release/ola_mundo
*/

fn main() {
    // Para rodar este arquivo específico individualmente sem o Cargo:
    // 1. rustc 4-Rodando-nosso-primeiro-projeto-em-Rust.rs
    // 2. ./4-Rodando-nosso-primeiro-projeto-em-Rust
    
    println!("Parabéns! Você aprendeu a criar, compilar e rodar projetos em Rust.");
}
