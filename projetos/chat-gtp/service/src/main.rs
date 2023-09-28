use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs::File;

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
}

fn handle_client(mut stream: TcpStream) {
    // Criando um buffer, tamanho padrão.
    let mut buffer = [0; 1024];

    // Instanciando o nosso detector de padrões para atender esse cliente.
    let pattern_matcher = Patterns::new("chat.txt", "img.txt");

    // Lendo os bytes recebidos.
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Convertendo os bytes para string.
            if let Ok(data) = String::from_utf8(buffer.to_vec()) {
                println!("Recebido: {}", data);

                // Como sabemos que está tudo certo (viva tratamento de erros do Rust!)
                // podemos verificar a string com o nosso detector de padrões.
                let mut response = "0";

                // Se a string começa com data:image/jpeg;base64, então é uma imagem.
                if data.starts_with("data:image/jpeg;base64,") {
                    // Verificando a imagem.
                    if pattern_matcher.check_img(&data) {
                        response = "1";
                    }
                }
                // Se não for uma imagem, então é um chat.
                else {
                    // Verificando o chat.
                    if pattern_matcher.check_string(&data) {
                        response = "1";
                    }
                }

                // Mandando uma resposta.
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Erro ao enviar resposta: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Erro ao ler do cliente: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Endereço não disponível.");

    println!("Servidor rodando em 127.0.0.1:8080");

    // Accept connections and spawn a new thread for each one.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client.
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Erro para aceitar conexão: {}", e);
            }
        }
    }
}
