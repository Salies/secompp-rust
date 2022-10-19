use std::io;

fn main() {
    let user_input: String = get_user_input();
    println!("You typed: {}", user_input);
}

fn get_user_input() -> String {
    let mut string_from_stdin = String::new();
    println!("Please type a number between 0-100 and press enter:");
    loop {
        io::stdin()
            .read_line(&mut string_from_stdin)
            .expect("Failed to read line");
        let trimmed = string_from_stdin.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => {
                if i > 100 {
                    println!("Please type a number between 0-100 and press enter:");
                    string_from_stdin = String::new();
                    continue;
                }
                return trimmed.to_string();
            }
            Err(..) => {
                println!("Please type a number between 0-100 and press enter:");
                string_from_stdin = String::new();
                continue;
            }
        };
    }
}
