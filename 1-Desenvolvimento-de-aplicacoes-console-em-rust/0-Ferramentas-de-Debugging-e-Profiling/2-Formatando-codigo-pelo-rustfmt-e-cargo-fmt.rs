/*
===============================================================================
TEMA: Formatando Código com rustfmt e cargo fmt
===============================================================================

O que são estas ferramentas?
- `rustfmt`: É a ferramenta oficial do Rust que lê o teu código-fonte e o 
  reescreve seguindo o guia de estilo padrão da comunidade (RFCs de estilo).
- `cargo fmt`: É um "wrapper" (um atalho) do Cargo que procura todos os arquivos 
  `.rs` dentro do teu projeto e aplica o `rustfmt` em todos eles de uma só vez.

-------------------------------------------------------------------------------
1. COMO USAR NO TERMINAL:
-------------------------------------------------------------------------------

A) Formatar o projeto inteiro (O mais utilizado):
   $ cargo fmt
   (Este comando não gera nenhuma saída textual se correr bem; ele simplesmente 
   altera e organiza os teus arquivos instantaneamente).

B) Apenas verificar se o código está formatado (Sem alterar os arquivos):
   $ cargo fmt -- --check
   (Muito utilizado em sistemas de Integração Contínua - CI/CD - para garantir 
   que nenhum programador enviou código desformatado para o repositório).

C) Formatar um arquivo isolado (Sem usar o Cargo):
   $ rustfmt main.rs

-------------------------------------------------------------------------------
2. EXEMPLO PRÁTICO (Código propositadamente desformatado):
-------------------------------------------------------------------------------
*/

// O código abaixo funciona perfeitamente para o compilador, mas está 
// horrivelmente formatado, dificultando a leitura humana (Code Smell).

fn      funcao_mal_formatada(a:i32,      b:i32)->i32{
let resultado=a       +b;
    println!("Soma: {}",    resultado);
         resultado
}

fn main() {
    println!("========================================");
    println!("   TESTANDO A FORMATAÇÃO AUTOMÁTICA    ");
    println!("========================================");

    // Repare no desalinhamento dos argumentos e nos espaços desnecessários:
    let x = 
    10;
    let y   =   20;
    
    let _res = funcao_mal_formatada(   x,
                                       y 
    );

    /*
       EXERCÍCIO PARA FAZER AGORA NO TEU EDITOR:
       
       1. Copia este código exatamente como está para o teu arquivo `main.rs`.
       2. Abre o terminal na pasta do projeto.
       3. Executa o comando:
          $ cargo fmt
       4. Volta ao teu editor de texto imediatamente. Vais ver que todo o 
          espaçamento foi corrigido, as chavetas foram alinhadas e o código 
          ficou limpo e legível!
    */
}

/*
-------------------------------------------------------------------------------
DICA EXTRA: Customização através do `rustfmt.toml`
-------------------------------------------------------------------------------
Embora o Rust adote um estilo universal por padrão, tu podes customizar algumas 
regras do formatador para o teu projeto. Para isso, basta criar um arquivo chamado 
`rustfmt.toml` na raiz do projeto.

Exemplo de configurações comuns que podes adicionar lá dentro:
```toml
# Define a largura máxima da linha (padrão é 100)
max_width = 120

# Força o uso de strings brutas ou normaliza aspas se necessário
format_strings = true

# Altera a forma como o Rust agrupa imports (use std::...)
imports_granularity = "Crate"
