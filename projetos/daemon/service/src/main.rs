use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::thread;

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1];

    // Nada de unwrap! Viva Rust.
    let (_stream, stream_handle) = OutputStream::try_default()
        .expect("Falha ao criar stream. Cheque se o dispositivo de saída está disponível.");

    let sink = Sink::try_new(&stream_handle)
        .expect("Falha ao criar sink. Cheque se o dispositivo de saída está disponível.");

    while let Ok(_) = stream.read(&mut buffer) {
        // map C para 261.63 Hz, e assim por diante
        // dava pra fazer um HashMap aqui, mas não aprendemos isso no curso
        let note: f32 = match buffer[0] {
            b'c' => 261.63,
            b'1' => 277.18,
            b'd' => 293.66,
            b'2' => 311.13,
            b'e' => 329.63,
            b'f' => 349.23,
            b'3' => 369.99,
            b'g' => 392.00,
            b'4' => 415.30,
            b'a' => 440.00,
            b'5' => 466.16,
            b'b' => 493.88,
            _ => 0.0,
        };

        let wave = SineWave::new(note);
        let source = wave.take_duration(Duration::from_secs_f32(0.25));
        sink.append(source);
        sink.sleep_until_end();

        // send response ok
        stream.write(b"ok").expect("Deu ruim!");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Não foi possível iniciar o servidor na porta 8080.");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
