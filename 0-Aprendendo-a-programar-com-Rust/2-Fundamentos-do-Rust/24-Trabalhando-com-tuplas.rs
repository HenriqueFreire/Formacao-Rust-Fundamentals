// Trabalhando com Tuplas em Rust
//
// Uma tupla é uma forma geral de agrupar um número de valores com uma variedade 
// de tipos em um único tipo composto. Tuplas têm um comprimento fixo: uma vez 
// declaradas, elas não podem crescer ou diminuir de tamanho.

fn main() {
    // 1. Criação de uma Tupla
    // Podemos agrupar diferentes tipos (i32, f64, u8, &str)
    let pessoa: (i32, &str, bool) = (30, "Henrique", true);

    // 2. Acessando Elementos por Índice
    // Usamos o ponto (.) seguido do índice (base zero)
    let idade = pessoa.0;
    let nome = pessoa.1;
    let ativo = pessoa.2;

    println!("Pessoa - Nome: {}, Idade: {}, Ativo: {}", nome, idade, ativo);

    // 3. Desestruturação (Destructuring)
    // Podemos quebrar uma tupla em variáveis individuais
    let (x, y, z) = (10, 20, 30);
    println!("Valores desestruturados: x={}, y={}, z={}", x, y, z);

    // 4. Tuplas como Retorno de Funções
    // Útil para retornar mais de um valor sem precisar criar uma Struct
    let (resultado, status) = dividir_com_status(10.0, 2.0);
    println!("Divisão: {} | Status: {}", resultado, status);

    // 5. Tupla Vazia (Unit Type)
    // Uma tupla sem nenhum elemento é chamada de 'unit' e seu valor é ().
    // É o que funções que não retornam nada explicitamente devolvem.
    let unit: () = ();
    println!("Valor Unit: {:?}", unit);
}

fn dividir_com_status(a: f64, b: f64) -> (f64, String) {
    if b == 0.0 {
        (0.0, String::from("Erro: Divisão por zero"))
    } else {
        (a / b, String::from("Sucesso"))
    }
}

/*
CARACTERÍSTICAS DAS TUPLAS:
1. Heterogêneas: Podem conter tipos diferentes no mesmo agrupamento.
2. Tamanho Fixo: Não podem ser redimensionadas após a criação.
3. Uso comum: Ótimas para retornos rápidos de funções ou pequenos agrupamentos 
   temporários de dados relacionados.
4. Tipagem: O tipo da tupla é definido pelos tipos de seus elementos na ordem exata.
   Ex: (i32, f64) é um tipo diferente de (f64, i32).
*/
