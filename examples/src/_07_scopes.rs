#[allow(dead_code)]
pub fn example() {
    // scopes
    let x = 5;
    {
        let y = 10; // y é valido apenas neste escopo
        println!("x = {}, y = {}", x, y);
    }
    // y não existe aqui

    // println!("x = {}, y = {}", x, y); // erro: y não está no escopo.

    // shadowing (sombreamento): redefinir uma variável com o mesmo nome.
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x = {}", x);

    // shadowing com tipos diferentes
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

    // shadowing com mut
    let spaces = "   ";
    let mut spaces = spaces.len();
    spaces += 1;
    println!("spaces = {}", spaces);

    // bloco de código
    let x = 5;
    let y = {
        let x = String::from("Secompp ");
        x + "2022"
    };
    println!("x = {}, y = {}", x, y);
    // x = 5, y = "Secompp 2022"
}
