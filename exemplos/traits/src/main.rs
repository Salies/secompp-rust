// Exemplo adaptado do Rust by Example
// https://doc.rust-lang.org/rust-by-example/trait.html

struct Ave {
    nome: &'static str,
    debicada: bool
}

// Métodos gerais para animais quaisquer
trait Animal {
    fn new(name: &'static str) -> Self;
    fn nome(&self) -> &'static str;
    fn falar(&self) {
        println!("{} diz oi!", self.nome());
    }
}

// Implementação específica para Ave
impl Ave {
    fn debicar(&mut self) {
        self.debicada = true;
    }

    fn debicada(&self) -> bool {
        return self.debicada
    }
}

// Implementação geral de Animal para Ave
impl Animal for Ave {
    fn new(nome: &'static str) -> Ave {
        Ave { nome: nome, debicada: false }
    }
    
    fn nome(&self) -> &'static str {
        self.nome
    }
    
    fn falar(&self) {
        if self.debicada() {
            println!("piu piu piu");
            return;
        }
        println!("Invista 100 ganhe 2000.");
    }
}

fn main() {
    let mut minha_ave: Ave = Animal::new("Urubu do Pix");
    // E se a gente tirar a especificação de tipo?
    
    minha_ave.falar();
    minha_ave.debicar();
    minha_ave.falar();
}