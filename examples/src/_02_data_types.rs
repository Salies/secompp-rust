#![allow(dead_code)]
pub fn example() {
    let decimal = 65.4321f32;

    // Caso quisermos converter para um unsigned de 8 bits,
    // a conversão não é feita implicitamente:
    // let integer: u8 = decimal;

    let integer = decimal as u8;

    let character = integer as char;
    println!("O caractere correspondente a {} é {}", integer, character);
    // Quando o valor não pode ser convertido, o compilador emite um erro. Por exemplo, 1000 não é um u8:
    // println!("1000 como u8 é: {}", 1000 as u8);

    let b = 1000;
    println!("1000 como u8 é: {}", b as u8);
}
