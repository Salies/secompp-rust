use std::fs::File;
use std::io::{BufRead, BufReader};

struct Patterns {
    chat_patterns: Vec<String>,
    img_patterns: Vec<String>
}

impl Patterns {
    // Recebe os paths dos arquivos com padrões de chat e imagens.
    // Carregamos via arquivo pois os padrões estão sempre mudando. O comunismo não para!
    fn new(chat_path: &'static str, img_path: &'static str) -> Patterns {
        let chat_patterns = Patterns::read_patterns(chat_path);
        let img_patterns = Patterns::read_patterns(img_path);
        Patterns {
            chat_patterns,
            img_patterns
        }
    }

    fn read_patterns(path: &'static str) -> Vec<String> {
        let mut patterns = Vec::new();
        let file = File::open(path).expect("Não foi possível abrir o arquivo");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            patterns.push(line.unwrap());
        }
        patterns
    }

    // Daria pra fazer com HashMap também. Novamente, não vimos...
    fn check_string(&self, string: &str) -> bool {
        for pattern in &self.chat_patterns {
            if string.contains(pattern) {
                return true;
            }
        }
        false
    }

    fn check_img(&self, string: &str) -> bool {
        for pattern in &self.img_patterns {
            if string.contains(pattern) {
                return true;
            }
        }
        false
    }

    fn print_one(&self) {
        println!("{}", self.img_patterns[1]);
    }
}

fn main() {
    println!("Hello, world!");
    let patterns = Patterns::new("chat.txt", "img.txt");
    patterns.print_one();
}
