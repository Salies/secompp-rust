use std::io;

#[allow(dead_code)]
pub fn example() {
    let user_input: String = get_user_input();
    println!("You typed: {}", user_input);

    let string_to_parse = String::from("123");
    let parsed = parse_u32(string_to_parse);
    let unwrapped = parsed.unwrap();
    println!("\n\n\nParsed: {}", unwrapped);
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

#[allow(dead_code)]
fn parse_u32(input: String) -> Result<u32, ()> {
    match input.trim().parse::<u32>() {
        Ok(i) => Ok(i),
        Err(..) => Err(()),
    }
}
