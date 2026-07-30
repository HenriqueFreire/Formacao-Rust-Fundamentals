use crate::db::cliente_db::ClienteDb;
use crate::tela::cliente_tela::*;
use crate::tela::ler::*;
use crate::tela::operacoes_basicas::*;

pub fn mostrar_menu() {
    let mut db = ClienteDb::new();

    loop {
        limpar_tela();

        println!(
            "=============== Menu ==============\n\
             Escolha uma das opções abaixo:\n\n\
             1 - Cadastrar cliente\n\
             2 - Alterar cliente\n\
             3 - Excluir cliente\n\
             4 - Listar clientes\n\
             0 - Sair do programa"
        );

        let opcao: i32 = ler_dados_int();
        match opcao {
            1 => cadastrar_cliente(&mut db),
            2 => alterar_cliente(&mut db),
            3 => excluir_cliente(&mut db),
            4 => listar_clientes(&db),
            0 => {
                limpar_tela();
                println!("Saindo do programa... Até logo!");
                esperar(1);
                break;
            }
            _ => {
                limpar_tela();
                println!("Opção inválida!");
                esperar(2);
            }
        }
    }
}
