# 🦀 Formação Rust Fundamentals - App Clientes Console

[![Rust](https://img.shields.io/badge/Rust-1.94%2B-orange.svg?style=flat&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows%20%7C%20macOS-blue.svg)]()

Repositório dedicado ao aprendizado e desenvolvimento de projetos práticos da **Formação Rust Fundamentals** (DIO - Digital Innovation One).

---

## 📌 Sobre o Projeto: `app_clientes_console`

O **`app_clientes_console`** é uma aplicação interativa de linha de comando (CLI/Terminal) construída em **Rust**, demonstrando a implementação de um **CRUD completo em memória** de gerenciamento de clientes, aplicando princípios de modularização, ownership, manipulação de vetores, structs e interatividade via console.

---

## ✨ Funcionalidades

- [x] ➕ **Cadastrar Cliente**: Registro dinâmico de clientes com `ID`, `Nome`, `CPF` e `Endereço`.
- [x] 📋 **Listar Clientes**: Exibição formatada em tabela alinhada com contagem total de registros em memória.
- [x] ✏️ **Alterar Cliente**: Edição de dados cadastrais de um cliente existente com preservação de valores não alterados (`Enter` em branco).
- [x] 🗑️ **Excluir Cliente**: Busca por ID e remoção segura do registro com confirmação explícita (`S/N`).
- [x] 🧹 **Limpeza Automática de Tela**: Utilização da biblioteca `clearscreen` para experiência de terminal fluida.

---

## 📁 Estrutura do Projeto

```text
Formacao-Rust-Fundamentals/
├── 0-Aprendendo-a-programar-com-Rust/
└── 1-Desenvolvimento-de-aplicacoes-console-em-rust/
    └── 2-Desenvolvimento-de-Aplicacoes-de-Console-Com-Rust/
        └── app_clientes_console/
            ├── Cargo.toml
            └── src/
                ├── main.rs
                ├── models/
                │   ├── mod.rs
                │   └── cliente.rs           # Struct Cliente (ID, Nome, CPF, Endereço)
                ├── db/
                │   ├── mod.rs
                │   └── cliente_db.rs        # Repositório/Banco de dados em memória
                └── tela/
                    ├── mod.rs
                    ├── menu.rs               # Loop do Menu Principal
                    ├── cliente_tela.rs       # Telas de CRUD interativas
                    ├── ler.rs                # Leitura e parsing de entradas no STDIN
                    └── operacoes_basicas.rs # Utilitários (limpeza de tela, tempo)
```

---

## 🚀 Como Executar o Projeto

### Pré-requisitos
Ter o ambiente **Rust / Cargo** instalado no seu sistema. Caso não possua, instale via [rustup.rs](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Passos para rodar
1. **Navegue até a pasta da aplicação console:**
   ```bash
   cd "1-Desenvolvimento-de-aplicacoes-console-em-rust/2-Desenvolvimento-de-Aplicacoes-de-Console-Com-Rust/app_clientes_console"
   ```

2. **Execute o projeto via Cargo:**
   ```bash
   cargo run
   ```

3. **Verificar tipos/compilação (opcional):**
   ```bash
   cargo check
   ```

---

## 💻 Exemplo da Interface do Terminal

```text
=============== Menu ==============
Escolha uma das opções abaixo:

1 - Cadastrar cliente
2 - Alterar cliente
3 - Excluir cliente
4 - Listar clientes
0 - Sair do programa
```

---

## 🛡️ Licença

Este projeto é desenvolvido para fins educacionais e didáticos na Formação Rust.
