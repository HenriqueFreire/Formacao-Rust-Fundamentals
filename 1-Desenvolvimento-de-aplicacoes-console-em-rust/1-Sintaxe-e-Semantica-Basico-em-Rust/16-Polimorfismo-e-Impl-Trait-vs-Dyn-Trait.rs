/*
===============================================================================
TEMA: Polimorfismo e Impl Trait vs Dyn Trait
===============================================================================

Polimorfismo é a capacidade de tratar diferentes tipos que compartilham um 
comportamento comum de forma uniforme. No Rust, isso é feito através de Traits.

Existem duas abordagens para realizar polimorfismo no Rust:
1. Static Dispatch (Despacho Estático) via `impl Trait` ou Generics.
2. Dynamic Dispatch (Despacho Dinâmico) via Trait Objects (`dyn Trait`).

-------------------------------------------------------------------------------
1. DEFINIÇÃO DO CONTRATO (TRAIT) E TIPOS CONCRETOS
-------------------------------------------------------------------------------
*/

pub trait Sensor {
    fn ler_valor(&self) -> f64;
    fn nome(&self) -> &str;
}

struct SensorTemperatura {
    temperatura: f64,
}

impl Sensor for SensorTemperatura {
    fn ler_valor(&self) -> f64 {
        self.temperatura
    }
    fn nome(&self) -> &str {
        "Sensor de Temperatura"
    }
}

struct SensorPressao {
    pressao: f64,
}

impl Sensor for SensorPressao {
    fn ler_valor(&self) -> f64 {
        self.pressao
    }
    fn nome(&self) -> &str {
        "Sensor de Pressão"
    }
}

/*
-------------------------------------------------------------------------------
2. STATIC DISPATCH (`impl Trait`)
-------------------------------------------------------------------------------
O despacho estático resolve as chamadas de método em tempo de compilação.
O compilador usa monomorfização (gera uma cópia da função para cada tipo real).

Vantagens:
- Desempenho máximo (sem custo em tempo de execução, chamadas diretas de função).
- Permite otimizações agressivas (inlining).

Desvantagens:
- Aumenta o tamanho do binário (Code Bloat).
- Apenas um tipo concreto real por vez. Não podemos misturar tipos em coleções.
- Funções que retornam `impl Trait` devem retornar apenas UM tipo concreto sob
  todas as ramificações de código (if/else).
*/

// Aceitando impl Trait como argumento (Açúcar sintático para Generics)
fn imprimir_dados_estatico(sensor: impl Sensor) {
    println!(
        "[ESTÁTICO] {}: Leitura = {:.2}",
        sensor.nome(),
        sensor.ler_valor()
    );
}

// Retornando impl Trait (Retorna um tipo específico determinado em compilação)
fn obter_sensor_temperatura_estatico() -> impl Sensor {
    SensorTemperatura { temperatura: 23.5 }
}

/*
// ATENÇÃO: O código abaixo NÃO compila!
// Embora ambos implementem `Sensor`, o compilador precisa saber o tipo exato em
// tempo de compilação, e não é permitido retornar tipos diferentes no if/else.
fn obter_sensor_invalido(tipo: &str) -> impl Sensor {
    if tipo == "temp" {
        SensorTemperatura { temperatura: 23.5 }
    } else {
        SensorPressao { pressao: 1013.25 } // Erro! Tipos incompatíveis
    }
}
*/

/*
-------------------------------------------------------------------------------
3. DYNAMIC DISPATCH (`dyn Trait`)
-------------------------------------------------------------------------------
O despacho dinâmico resolve as chamadas de método em tempo de execução.
Ele usa ponteiros para uma tabela de métodos virtuais (vtable). No Rust, 
isso é representado por `dyn Trait`. Como o tamanho do tipo dinâmico não é 
conhecido em tempo de compilação, ele deve estar atrás de um ponteiro (`&dyn Trait`, 
`Box<dyn Trait>`, `Rc<dyn Trait>`).

Vantagens:
- Permite coleções heterogêneas (um vetor contendo tipos diferentes que 
  implementam a mesma trait).
- Reduz o tamanho do binário (apenas uma função é gerada).
- Permite retornar tipos diferentes dinamicamente em tempo de execução.

Desvantagens:
- Pequeno custo de desempenho (desreferenciação de ponteiros e vtable lookups).
- Impede certas otimizações do compilador (como inlining).
*/

// Aceitando dyn Trait (através de referência)
fn imprimir_dados_dinamico(sensor: &dyn Sensor) {
    println!(
        "[DINÂMICO] {}: Leitura = {:.2}",
        sensor.nome(),
        sensor.ler_valor()
    );
}

// Retornando Box<dyn Sensor> (Permite retornar tipos diferentes sob lógica de runtime!)
fn criar_sensor_dinamico(tipo: &str) -> Box<dyn Sensor> {
    if tipo == "temp" {
        Box::new(SensorTemperatura { temperatura: 25.4 })
    } else {
        Box::new(SensorPressao { pressao: 1012.0 })
    }
}

/*
-------------------------------------------------------------------------------
4. EXECUÇÃO E COMPARAÇÃO PRÁTICA
-------------------------------------------------------------------------------
*/

fn main() {
    println!("=========================================");
    println!("      IMPL TRAIT VS DYN TRAIT            ");
    println!("=========================================");

    let temp = SensorTemperatura { temperatura: 22.1 };
    let pressao = SensorPressao { pressao: 1008.5 };

    // 1. Usando Static Dispatch (Monomorfização)
    println!("\n--> Testando Static Dispatch (impl Trait):");
    imprimir_dados_estatico(temp);
    imprimir_dados_estatico(pressao);

    // Reutilizando a função que retorna impl Trait
    let sensor_estatico = obter_sensor_temperatura_estatico();
    imprimir_dados_estatico(sensor_estatico);

    // 2. Usando Dynamic Dispatch (vtable)
    println!("\n--> Testando Dynamic Dispatch (dyn Trait):");
    let temp_ref = SensorTemperatura { temperatura: 18.9 };
    let pressao_ref = SensorPressao { pressao: 995.0 };
    
    imprimir_dados_dinamico(&temp_ref);
    imprimir_dados_dinamico(&pressao_ref);

    // 3. Coleção Heterogênea (Impossível de fazer com Static Dispatch puro!)
    println!("\n--> Testando Coleção Heterogênea (Vec<Box<dyn Sensor>>):");
    let sensores: Vec<Box<dyn Sensor>> = vec![
        Box::new(SensorTemperatura { temperatura: 26.7 }),
        Box::new(SensorPressao { pressao: 1020.1 }),
        criar_sensor_dinamico("temp"),
        criar_sensor_dinamico("pressao"),
    ];

    for sensor in sensores {
        imprimir_dados_dinamico(&*sensor);
    }
}

/*
-------------------------------------------------------------------------------
TABELA COMPARATIVA RESUMIDA:
-------------------------------------------------------------------------------
| Característica         | `impl Trait` (Static)       | `dyn Trait` (Dynamic)      |
|------------------------|-----------------------------|----------------------------|
| Resolução              | Tempo de Compilação         | Tempo de Execução          |
| Abordagem              | Monomorfização (Code Bloat) | Vtable (Virtual Pointer)   |
| Custo de Execução      | Zero (Custo zero)           | Pequeno (vtable lookup)    |
| Tamanho do Binário     | Maior                       | Menor                      |
| Coleções Heterogêneas  | Não permitida               | Permitida                  |
| Retornos Múltiplos     | Apenas um tipo concreto     | Tipos concretos variados   |
| Pointer Overhead       | Nenhum                      | Requer pointer (Box/&/etc) |
-------------------------------------------------------------------------------
*/
