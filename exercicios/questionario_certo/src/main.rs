fn de_menor(idade: u8) {
    if idade < 18 {
        println!("Menor de idade!");
        return;
    }
    
    println!("Não é menor de idade.");
}

fn main() {
    let idade: u8 = 17;
    de_menor(idade);
    de_menor(idade);
}

