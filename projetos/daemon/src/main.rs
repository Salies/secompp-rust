use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, source::Source};

struct Player {
    sink: Sink,
}

impl Player {
    fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Player {
            sink,
        }
    }

    // Toca música a partir de um arquivo (str)
    fn play(&mut self, file: &str) {
        let file = File::open(file).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        self.sink.clear();
        self.sink.append(source);
        self.sink.play();
    }

    fn pause(&mut self) {
        self.sink.pause();
    }
}

fn main() {
    println!("Hello, world!");
}
