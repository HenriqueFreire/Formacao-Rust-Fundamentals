/*
===============================================================================
TEMA: Coleções Dinâmicas - HashMap (Tabelas de Espalhamento)
===============================================================================

Diferente de tipos básicos como arrays, o `HashMap` não faz parte do "prelude" 
automático do Rust. Isso significa que precisamos importá-lo explicitamente 
do módulo de coleções da biblioteca padrão (`std::collections`).

Os dados de um `HashMap` são alocados dinamicamente na **Heap**.

-------------------------------------------------------------------------------
1. IMPORTAÇÃO E INICIALIZAÇÃO
-------------------------------------------------------------------------------
*/

use std::collections::HashMap;

fn main() {
    println!("========================================");
    println!("          TRABALHANDO COM HASHMAP       ");
    println!("========================================");

    // Criando um HashMap vazio mutável. 
    // Chave: ID do Dispositivo (u32) | Valor: Nome/Localização (String)
    let mut dispositivos: HashMap<u32, String> = HashMap::new();

    // -------------------------------------------------------------------------
    // A) INSERINDO DADOS (`insert`)
    // -------------------------------------------------------------------------
    dispositivos.insert(101, String::from("Sensor_Temperatura_SalaA"));
    dispositivos.insert(102, String::from("Atuador_Valvula_BlocoB"));
    dispositivos.insert(103, String::from("Microcontrolador_Elevador"));

    // Nota de Sobrescrita: Se você inserir uma chave que já existe, 
    // o Rust sobrescreve o valor antigo e te devolve o valor que foi substituído.
    let valor_antigo = dispositivos.insert(101, String::from("Sensor_Termopar_SalaA"));
    println!("Valor antigo substituído da chave 101: {:?}", valor_antigo);


    // -------------------------------------------------------------------------
    // B) ACESSANDO VALORES (`get`)
    // -------------------------------------------------------------------------
    // O método `.get()` recebe uma referência da chave e retorna uma `Option<&V>`.
    // Retorna `Some(&valor)` se existir, ou `None` se a chave não estiver na tabela.
    let id_busca = 102;
    match dispositivos.get(&id_busca) {
        Some(nome) => println!("Dispositivo {} encontrado: {}", id_busca, nome),
        None => println!("Dispositivo {} não cadastrado.", id_busca),
    }


    // -------------------------------------------------------------------------
    // C) MODIFICAÇÃO SEGURA COM A API `entry` (A joia do HashMap)
    // -------------------------------------------------------------------------
    println!("\n--- Usando a API Entry ---");
    
    // Cenário: Você quer inserir um dado APENAS se a chave ainda não existir.
    // O método `.entry()` verifica a chave. O `.or_insert()` insere o valor 
    // caso esteja vazia, e em ambos os casos retorna uma referência mutável para o valor.
    dispositivos.entry(104).or_insert(String::from("Roteador_Gateway")); // Será inserido
    dispositivos.entry(102).or_insert(String::from("Tentei_Mudar_Mas_Ja_Existe")); // Ignorado

    println!("Estado após inserções seguras: {:?}", dispositivos.get(&104));


    // -------------------------------------------------------------------------
    // D) REMOVENDO ELEMENTOS (`remove`)
    // -------------------------------------------------------------------------
    // Remove a chave e te devolve o valor que estava lá dentro (se existia).
    dispositivos.remove(&103); 


    // -------------------------------------------------------------------------
    // E) ITERANDO SOBRE UM HASHMAP
    // -------------------------------------------------------------------------
    println!("\n--- Iterando sobre os Dispositivos Ativos ---");
    // Atenção: A iteração de um HashMap é ARBITRÁRIA. Os elementos não vêm 
    // na ordem em que foram inseridos.
    for (id, nome) in &dispositivos {
        println!("Código Hardware: {} -> Identificador: {}", id, nome);
    }
}

/*
-------------------------------------------------------------------------------
VISÃO DE ENGENHARIA DA COMPUTAÇÃO (Segurança Cibernética vs Performance):
-------------------------------------------------------------------------------
Por padrão, o `HashMap` do Rust utiliza um algoritmo de hashing chamado **SipHash 1-3**. 
Ele não é o algoritmo mais rápido do mundo em termos matemáticos puros, mas foi 
escolhido por um motivo crítico de engenharia: **Resistência a ataques DoS (Denial of Service)**.

Em algoritmos mais fracos, um hacker pode enviar dados cirurgicamente calculados 
para gerar "Colisões de Hash" (fazer com que chaves diferentes gerem o mesmo índice 
na tabela). Isso transforma a busca de tempo constante $O(1)$ em uma busca linear $O(n)$, 
fazendo o processador travar em 100% tentando resolver a tabela. O SipHash previne isso.



Se para o seu projeto de Engenharia (como um sistema embarcado isolado sem internet) 
a velocidade pura for mais importante que a proteção contra ataques de colisão, 
você pode trocar o gerador de hash padrão por um mais rápido (como o `FxHash` ou `fnv`) 
adicionando um "Hasher" customizado na inicialização:

```rust
// Exemplo conceitual de troca de algoritmo para performance máxima interna:
// let mut mapa_ultra_rapido = HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default());
