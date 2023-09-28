use std::io::stdin;

fn main() {
    let furry_words = [
        "furry", "felpudo", "paw", "sonic", "shadow", "ouriço", "e621",
        "scalie", "fursuit", "uwu", "catgirl", "fluffy", "yiff", "owo"
    ];

    loop {
        let mut user_input = String::new();
        stdin().read_line(&mut user_input)
        .expect("Erro ao ler entrada do usuário!");
        for word in &furry_words {
            if user_input.to_lowercase().contains(word) {
                println!("Furry detectado!");
                break;
            }
        }
        user_input.clear();
    }
}
