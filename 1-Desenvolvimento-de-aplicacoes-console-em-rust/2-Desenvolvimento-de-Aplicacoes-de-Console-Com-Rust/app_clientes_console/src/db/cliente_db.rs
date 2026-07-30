use crate::models::cliente::Cliente;

pub struct ClienteDb {
    clientes: Vec<Cliente>,
}

impl ClienteDb {
    pub fn new() -> Self {
        ClienteDb {
            clientes: Vec::new(),
        }
    }

    pub fn adicionar(&mut self, cliente: Cliente) {
        self.clientes.push(cliente);
    }

    pub fn todos(&self) -> &Vec<Cliente> {
        &self.clientes
    }

    pub fn buscar_por_id(&self, id: i32) -> Option<&Cliente> {
        self.clientes.iter().find(|c| c.id == id)
    }

    pub fn alterar(&mut self, id: i32, novo_cliente: Cliente) -> bool {
        if let Some(cliente) = self.clientes.iter_mut().find(|c| c.id == id) {
            *cliente = novo_cliente;
            true
        } else {
            false
        }
    }

    pub fn excluir(&mut self, id: i32) -> bool {
        let tamanho_anterior = self.clientes.len();
        self.clientes.retain(|c| c.id != id);
        self.clientes.len() < tamanho_anterior
    }
}
