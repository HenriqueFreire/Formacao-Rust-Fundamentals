/*
===============================================================================
TEMA: Trabalhando com Cargo em Modo Watch (cargo-watch)
===============================================================================

O que é o `cargo-watch`?
É uma ferramenta utilitária que estende o Cargo. Ela monitora o diretório do 
seu projeto e, sempre que você salva um arquivo, ela executa automaticamente 
o comando que você definiu (como checar erros, rodar testes ou executar o app).

-------------------------------------------------------------------------------
1. INSTALAÇÃO (Execute uma única vez no seu terminal do sistema):
-------------------------------------------------------------------------------
Para instalar globalmente no seu computador, use o comando:
$ cargo install cargo-watch

-------------------------------------------------------------------------------
2. COMANDOS MAIS UTILIZADOS NO DIA A DIA:
-------------------------------------------------------------------------------

A) Modo Seguro / Rápido (Apenas checa erros de compilação):
   $ cargo watch -x check
   (Dica: Use este enquanto escreve código complexo para ver os erros do 
   compilador em tempo real sem gastar tempo gerando o binário final).

B) Modo Desenvolvimento (Checa e executa o código a cada salvamento):
   $ cargo watch -x run
   (É o mais usado. Salvou o arquivo, ele limpa a tela e roda o `main`).

C) Modo de Testes (Ótimo para TDD - Test-Driven Development):
   $ cargo watch -x test
   (Fica rodando seus testes automatizados toda vez que você muda o código).

D) Combinando comandos (Checa, testa e roda se tudo passar):
   $ cargo watch -x check -x test -x run

-------------------------------------------------------------------------------
3. EXEMPLO PRÁTICO PARA TESTAR O WATCH:
-------------------------------------------------------------------------------
*/

fn main() {
    println!("========================================");
    println!("   BEM-VINDO AO CARGO WATCH NO RUST!   ");
    println!("========================================");

    let resultado = calcular_bonus(1000.0, 0.15);
    println!("O bônus calculado foi de: R$ {}", resultado);

    /*
       EXERCÍCIO PARA FAZER AGORA COM O WATCH RODANDO:
       
       1. Abra seu terminal na pasta do projeto e rode: `cargo watch -x run`
       2. Deixe o terminal visível de um lado da tela e o editor do outro.
       3. Altere o valor do salário ou da taxa abaixo.
       4. Salve o arquivo (Ctrl+S ou Cmd+S).
       5. Olhe para o terminal: ele vai recompilar e rodar sozinho instantaneamente!
    */
    
    let salario_engenheiro = 8500.0;
    let taxa_bonus_senior = 0.25; // Experimente mudar este valor para 0.30 e salvar!
    
    println!(
        "Engenheiro de Computação - Salário com bônus: R$ {}", 
        calcular_bonus(salario_engenheiro, taxa_bonus_senior)
    );
}

/// Uma função simples apenas para termos código mutável para o teste.
fn calcular_bonus(salario: f64, taxa: f64) -> f64 {
    // Se você quebrar o código de propósito aqui (ex: esquecer o ponto e vírgula),
    // o cargo watch vai te mostrar o erro imediatamente ao salvar.
    salario + (salario * taxa)
}

/*
-------------------------------------------------------------------------------
DICA DE OURO PARA O SEU CURSO DA DIO / SANTANDER:
-------------------------------------------------------------------------------
O compilador do Rust (rustc) é famoso por ser um "professor" que dá ótimos 
conselhos de correção. Deixar o `cargo watch -x check` rodando em uma aba 
lateral do seu monitor enquanto você faz as aulas da DIO vai acelerar muito o 
seu aprendizado, pois você verá o impacto de cada linha escrita imediatamente, 
sem quebrar o ritmo do seu raciocínio para ir ao terminal digitar comandos.
*/
