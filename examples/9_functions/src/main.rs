fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn diff(a: i32, b: i32) -> i32 {
    return a - b;
}

fn soma_tres(a: &mut u8) {
    *a += 3 // * "deference" -- volta a ser valor
}

fn main() {
    println!("Hello, world!");
    let mut n: u8 = 3;
    // pego emprestado declarando explicitamente que o valor é mutável
    soma_tres(&mut n);
    println!("{}", n);
}
