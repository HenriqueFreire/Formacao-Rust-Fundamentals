/*
===============================================================================
TEMA: Análise de Performance com Valgrind e Perf no Rust
===============================================================================

O que são estas ferramentas?
- Valgrind: É uma suíte de ferramentas de simulação. A mais famosa delas é o 
  `memcheck`, que detecta vazamentos de memória (embora o Rust evite isso nativamente),
  e o `callgrind`, que rastreia a hierarquia de chamadas de funções e uso de cache.
- Perf: É a ferramenta oficial de monitoramento de performance do kernel do Linux. 
  Ela faz uma amostragem estatística do processador para descobrir quais linhas de 
  código estão gerando gargalos (os famosos "Hotspots").

-------------------------------------------------------------------------------
1. COMO PREPARAR O SEU CÓDIGO RUST PARA PERFILAMENTO (Profiling):
-------------------------------------------------------------------------------
Diferente dos debuggers, para analisar performance você quer que o código rode 
rápido, então usamos o modo `--release`. Porém, precisamos dizer ao Rust para 
manter os "símbolos de debug" (os nomes das nossas funções) no binário final, 
caso contrário o Valgrind e o Perf só nos mostrarão endereços de memória hexadecimais.

Para fazer isso, adicione estas linhas ao arquivo `Cargo.toml` do seu projeto:

```toml
[profile.release]
debug = true
