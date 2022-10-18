fn main() {
    // Convertendo o primitivo str pro tipo não primitivo String
    let my_str = "hello";
    let my_string = String::from(my_str);
    /*
    Sim, o Rust tem um tipo chamado String com letra maiúscula.
    Quem programa POO deve estar se perguntando: pô, tem um tipo que é um wrapper
    de um tipo primitivo, e esse tipo wrapper tem métodos, funções, acopladas a ele,
    inclusive de conversão... saporra é uma classe!
    Calma, não é classe. Tem cara de classe, tem cheiro de classe, mas não é classe.
     */
}

// Podemos criar conversores para nossos tipos customizados usando a trait From
// Por hora, pense na trait como "uma funçãozinha do tipo". Quem sabe POO e principalmente
// quem já programou C++ já sacou como funciona ou se não sacou vai sacar agora.
// Enfim, depois a gente explica melhor, por enquanto é só pra mostrar.
// NÃO PRECISA COPIAR.
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    // Com a trait From definida para o nosso tipo,
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
