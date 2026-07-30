use std::io;

pub fn ler_dados() -> String {
    let mut dados: String = String::new();
    io::stdin().read_line(&mut dados).expect("Falha ao ler dados");
    dados.trim().to_string()
}

pub fn ler_dados_int() -> i32 {
    let mut dados: String = String::new();
    io::stdin().read_line(&mut dados).expect("Falha ao ler dados");
    match dados.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    }
}
