struct Ovelha {
    pelada: bool,
    nome: &'static str,
}

trait Animal {
    // Não é um construtor...
    // Self refere-se ao implementador (Animal ou algum herdeiro de Animal)
    fn new(nome: &'static str) -> Self;
    // Não são getters...
    // Não precisa de implementação
    fn nome(&self) -> &'static str;
    fn barulho(&self) -> &'static str;

    // Mas pode ser implementando também
    // Quem herdar esse trait vai usar esse método por padrão
    // a não ser que ele seja sobrescrito
    fn falar(&self) {
        println!("{} diz {}", self.nome(), self.barulho());
    }
}

// Implementando funções para a struct Ovelha
// impl funciona com o trait, mas coma obrigatoriedade de implementar
// as funções declaradas
// !!! TESTAR ISSO AO VIVO !!!
// Perceba que aqui apenas implementamos métodos novos
impl Ovelha {
    fn esta_pelada(&self) -> bool {
        self.pelada
    }

    fn tosar(&mut self) {
        if self.esta_pelada() {
            // Mesmo ainda não declara (apenas abaixo), self.nome() já pode ser usada
            println!("{} já está pelada...", self.nome());
        } else {
            self.pelada = true;
            println!("{} foi tosada!", self.nome);
        }
    }
}

// Implementando o trait Animal para a struct Ovelha
// Perceba que aqui apenas sobrescrevemos os métodos delcarados em Animal
impl Animal for Ovelha {
    // Self, do trait original, é o tipo do implementador
    // No caso, Ovelha
    fn new(nome: &'static str) -> Ovelha {
        Ovelha {
            nome: nome,
            pelada: false,
        }
    }

    fn nome(&self) -> &'static str {
        self.nome
    }

    fn barulho(&self) -> &'static str {
        if self.esta_pelada() {
            "mééé! estou pelada!!!"
        } else {
            "mééé. estou quentinha"
        }
    }

    // Sobrescrevendo o método falar definido em Animal
    fn falar(&self) {
        println!("{} ovelhadamente diz: {}", self.nome, self.barulho());
    }
}

fn main() {
    // Preciso especificar Ovelha nesse caso
    let mut minha_ovelha: Ovelha = Animal::new("Shaun");
    // Ou, então
    //let mut minha_outra_ovelha = Ovelha::new("Shaun");
    // Isso não funciona
    //let mut minha_ovelha = Animal::new("Shaun");
}

/*
Apesar de não ser um orientação a objetos propriamente dita,
traits são muito úteis para quando você precisa modelar um
um universo de informação bem definido. Usaremos isso no projeto.
*/
