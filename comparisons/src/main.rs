use std::io;

fn main() {
    let user_input: String = get_user_input();
    println!("You typed: {}", user_input);
}

fn get_user_input() -> String {
    let mut string_from_stdin = String::new();
    println!("Please type a number between 0-100 and press enter:");
    loop {
        // Ler do stdin e armazenar em string_from_stdin. O valor de retorno é um Result, mas
        // não estamos interessados no valor de retorno.
        io::stdin()
            .read_line(&mut string_from_stdin)
            .expect("Failed to read line");

        let trimmed = string_from_stdin.trim();
        let parsed = trimmed.parse::<u32>();
        match parsed {
            Ok(i) => {
                if i > 100 {
                    println!("Please type a number between 0-100 and press enter:");
                    string_from_stdin = String::new(); // reset the string (read_line appends to it)
                    continue;
                }
                return trimmed.to_string();
            }
            Err(..) => {
                println!("Please type a number between 0-100 and press enter:");
                string_from_stdin = String::new(); // reset the string (read_line appends to it)
                continue;
            }
        };
    }
}
