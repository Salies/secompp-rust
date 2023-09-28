use base64::{Engine as _, engine::general_purpose};
use std::io::stdin;

fn to_base64(input: &[u8]) -> String {
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(input);
    return encoded;
}

fn main() {
    let mut user_input = String::new();
    loop {
        stdin().read_line(&mut user_input).expect("Falha ao ler linha do usuário");
        let encoded = to_base64(user_input.as_bytes());
        println!("{}", encoded);
        user_input.clear();
    }
}