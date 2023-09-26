fn main() {
    // insira seu comentário aqui
    let mut msg = "oi";
    println!("{}", msg);
    
    // sinta-se livre para testar outros valores aqui!
    // o que será que acontece?
    let number = 42;
    
    // o que significa essa atribuição?
    msg = match number {
        666 => "Então você é do rock? 😈 🤘",
        42 => "É a resposta.",
        333 => "Meio besta.",
        // o que a expressão abaixo captura?
        5000..=5999 => "Você sabia que existem mais de 5,000 tipos diferentes de batatas?!",
        // e essa?
        621 | 177013 => "Sai de perto de mim.",
        _ => "Sem graça."
    };
    
    // qual o valor de msg aqui?
    println!("{}", msg);
}