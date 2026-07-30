use std::io::{self, Write};
use crate::models::cliente::Cliente;
use crate::db::cliente_db::ClienteDb;
use crate::tela::ler::{ler_dados, ler_dados_int};
use crate::tela::operacoes_basicas::{esperar, limpar_tela};

pub fn cadastrar_cliente(db: &mut ClienteDb) {
    limpar_tela();
    println!("=============== Cadastrar Cliente ===============");

    print!("Digite o ID do cliente: ");
    let _ = io::stdout().flush();
    let id = ler_dados_int();

    print!("Digite o Nome do cliente: ");
    let _ = io::stdout().flush();
    let nome = ler_dados();

    print!("Digite o CPF do cliente: ");
    let _ = io::stdout().flush();
    let cpf = ler_dados();

    print!("Digite o Endereço do cliente: ");
    let _ = io::stdout().flush();
    let endereco = ler_dados();

    let novo_cliente = Cliente {
        id,
        nome,
        cpf,
        endereco,
    };

    db.adicionar(novo_cliente);

    println!("\n✅ Cliente cadastrado com sucesso em memória!");
    esperar(2);
}

pub fn alterar_cliente(db: &mut ClienteDb) {
    limpar_tela();
    println!("=============== Alterar Cliente ===============");

    if db.todos().is_empty() {
        println!("\n⚠️  Nenhum cliente cadastrado em memória para alterar.");
        esperar(2);
        return;
    }

    print!("Digite o ID do cliente que deseja alterar: ");
    let _ = io::stdout().flush();
    let id = ler_dados_int();

    let (nome_atual, cpf_atual, endereco_atual) = match db.buscar_por_id(id) {
        Some(c) => (c.nome.clone(), c.cpf.clone(), c.endereco.clone()),
        None => {
            println!("\n❌ Cliente com ID {} não encontrado.", id);
            esperar(2);
            return;
        }
    };

    println!("\n--- Dados Atuais do Cliente ---");
    println!("ID: {}", id);
    println!("Nome: {}", nome_atual);
    println!("CPF: {}", cpf_atual);
    println!("Endereço: {}", endereco_atual);
    println!("--------------------------------");
    println!("(Pressione Enter em branco para manter o valor atual)\n");

    print!("Novo Nome [{}]: ", nome_atual);
    let _ = io::stdout().flush();
    let input_nome = ler_dados();
    let novo_nome = if input_nome.trim().is_empty() { nome_atual } else { input_nome };

    print!("Novo CPF [{}]: ", cpf_atual);
    let _ = io::stdout().flush();
    let input_cpf = ler_dados();
    let novo_cpf = if input_cpf.trim().is_empty() { cpf_atual } else { input_cpf };

    print!("Novo Endereço [{}]: ", endereco_atual);
    let _ = io::stdout().flush();
    let input_endereco = ler_dados();
    let novo_endereco = if input_endereco.trim().is_empty() { endereco_atual } else { input_endereco };

    let novo_cliente = Cliente {
        id,
        nome: novo_nome,
        cpf: novo_cpf,
        endereco: novo_endereco,
    };

    if db.alterar(id, novo_cliente) {
        println!("\n✅ Cliente alterado com sucesso!");
    } else {
        println!("\n❌ Erro ao alterar cliente.");
    }
    esperar(2);
}

pub fn excluir_cliente(db: &mut ClienteDb) {
    limpar_tela();
    println!("=============== Excluir Cliente ===============");

    if db.todos().is_empty() {
        println!("\n⚠️  Nenhum cliente cadastrado em memória para excluir.");
        esperar(2);
        return;
    }

    print!("Digite o ID do cliente que deseja excluir: ");
    let _ = io::stdout().flush();
    let id = ler_dados_int();

    let cliente_info = match db.buscar_por_id(id) {
        Some(c) => format!("ID: {} | Nome: {} | CPF: {} | Endereço: {}", c.id, c.nome, c.cpf, c.endereco),
        None => {
            println!("\n❌ Cliente com ID {} não foi encontrado.", id);
            esperar(2);
            return;
        }
    };

    println!("\n--- Cliente Selecionado ---");
    println!("{}", cliente_info);
    println!("---------------------------");

    print!("\n⚠️  Deseja realmente excluir este cliente? (S/N): ");
    let _ = io::stdout().flush();
    let confirmacao = ler_dados().to_lowercase();

    if confirmacao == "s" || confirmacao == "sim" {
        if db.excluir(id) {
            println!("\n✅ Cliente removido com sucesso em memória!");
        } else {
            println!("\n❌ Erro ao remover cliente.");
        }
    } else {
        println!("\nℹ️  Operação de exclusão cancelada.");
    }

    esperar(2);
}

pub fn listar_clientes(db: &ClienteDb) {
    limpar_tela();
    println!("=================================================================");
    println!("                      LISTA DE CLIENTES                          ");
    println!("=================================================================");

    let clientes = db.todos();
    if clientes.is_empty() {
        println!("\n⚠️  Nenhum cliente cadastrado até o momento em memória.\n");
    } else {
        println!("{:<5} | {:<20} | {:<15} | {:<25}", "ID", "NOME", "CPF", "ENDEREÇO");
        println!("-----------------------------------------------------------------");
        for cliente in clientes {
            println!(
                "{:<5} | {:<20} | {:<15} | {:<25}",
                cliente.id, cliente.nome, cliente.cpf, cliente.endereco
            );
        }
        println!("-----------------------------------------------------------------");
        println!("Total de clientes cadastrados: {}", clientes.len());
    }

    println!("\nPressione Enter para continuar...");
    let _ = ler_dados();
}
