use std::io;

pub fn ler_dados() -> String {
    let mut dados: String = String::new();
    io::stdin().read_line(&mut dados).expect("Falha ao ler dados");
    dados.trim().to_string()
}

pub fn ler_dados_int() -> i32 {
    let mut dados: String = String::new();
    io::stdin().read_line(&mut dados).expect("Falha ao ler dados");
    dados.trim().parse().expect("Erro ao converter dados para inteiro")
}
